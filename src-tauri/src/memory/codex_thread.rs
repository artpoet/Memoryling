use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::model::{
    AccessScope, CodexThreadCandidate, CodexThreadCatalog, ConsentScopeV1, ImportPreview,
    NormalizedMemoryEvent, PreparedImport, PreviewRecord, PreviewTimeRange, SourceOption,
    CODEX_THREAD_ADAPTER_ID, CODEX_THREAD_ADAPTER_VERSION, MEMORY_EVENT_SCHEMA_VERSION,
};

const EXPECTED_CODEX_VERSION: &str = "0.134.0";
const APP_SERVER_TIMEOUT: Duration = Duration::from_secs(10);
const PROCESS_CLEANUP_TIMEOUT: Duration = Duration::from_secs(1);
const MAX_STDOUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_STDOUT_LINE_BYTES: usize = 1024 * 1024;
const MAX_FINAL_ANSWER_BYTES: usize = 256 * 1024;
const MAX_IDENTIFIER_BYTES: usize = 512;
const THREAD_LIMIT: u64 = 25;

const CONNECTOR_ERROR: &str =
    "The version-bound Codex thread-history pilot could not read this source safely.";
const UNSUPPORTED_VERSION_ERROR: &str =
    "This Codex version is not supported by the thread-history pilot.";

#[derive(Debug, Clone)]
struct InternalCandidate {
    raw_thread_id: String,
    display_name: String,
}

/// Raw Codex thread IDs live only in this non-serializable Rust value.
#[derive(Debug, Clone)]
pub(crate) struct InternalCatalog {
    catalog_id: String,
    candidates: HashMap<String, InternalCandidate>,
}

impl InternalCatalog {
    pub(crate) fn catalog_id(&self) -> &str {
        &self.catalog_id
    }
}

pub(crate) fn load_catalog() -> Result<(CodexThreadCatalog, InternalCatalog), String> {
    let result = run_app_server_call("thread/list", thread_list_params())?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| CONNECTOR_ERROR.to_string())?
        .as_nanos()
        .to_string();
    parse_thread_list(result, &nonce)
}

pub(crate) fn preview_selected_thread(
    catalog: &InternalCatalog,
    catalog_id: &str,
    candidate_id: &str,
) -> Result<(ImportPreview, PreparedImport), String> {
    if catalog.catalog_id != catalog_id {
        return Err("The Codex thread catalog expired. List threads again.".to_string());
    }
    let candidate = catalog
        .candidates
        .get(candidate_id)
        .ok_or_else(|| "The selected Codex thread is not in this catalog.".to_string())?;
    let result = run_app_server_call(
        "thread/read",
        json!({
            "threadId": candidate.raw_thread_id,
            "includeTurns": true
        }),
    )?;
    prepare_thread_import(result, candidate)
}

fn thread_list_params() -> Value {
    json!({
        "cursor": null,
        "limit": THREAD_LIMIT,
        "sortKey": "updated_at",
        "sortDirection": "desc",
        "sourceKinds": ["cli", "vscode"],
        "archived": false,
        "useStateDbOnly": true
    })
}

fn parse_thread_list(
    result: Value,
    nonce: &str,
) -> Result<(CodexThreadCatalog, InternalCatalog), String> {
    let object = result.as_object().ok_or_else(connector_error)?;
    let data = object
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(connector_error)?;
    if data.len() > THREAD_LIMIT as usize {
        return Err(CONNECTOR_ERROR.to_string());
    }

    let mut raw_ids = Vec::with_capacity(data.len());
    for thread in data {
        let thread = thread.as_object().ok_or_else(connector_error)?;
        let raw_id = bounded_identifier(thread.get("id")).ok_or_else(connector_error)?;
        let _ = unix_timestamp(thread.get("updatedAt")).ok_or_else(connector_error)?;
        let status = thread_status(thread.get("status")).ok_or_else(connector_error)?;
        if eligible_status(status) {
            raw_ids.push(raw_id.to_string());
        }
    }
    if raw_ids.iter().collect::<HashSet<_>>().len() != raw_ids.len() {
        return Err(CONNECTOR_ERROR.to_string());
    }

    let mut catalog_parts = Vec::with_capacity(raw_ids.len() + 1);
    catalog_parts.push(nonce);
    catalog_parts.extend(raw_ids.iter().map(String::as_str));
    let catalog_id = stable_id("catalog", &catalog_parts);
    let mut candidates = Vec::with_capacity(raw_ids.len());
    let mut internal = HashMap::with_capacity(raw_ids.len());
    let mut visible_index = 0usize;

    for thread in data {
        let thread = thread.as_object().ok_or_else(connector_error)?;
        let status = thread_status(thread.get("status")).ok_or_else(connector_error)?;
        if !eligible_status(status) {
            continue;
        }
        let raw_id = bounded_identifier(thread.get("id")).ok_or_else(connector_error)?;
        let updated_at = unix_timestamp(thread.get("updatedAt")).ok_or_else(connector_error)?;
        visible_index += 1;
        let candidate_id = stable_id("candidate", &[&catalog_id, raw_id]);
        let display_name = format!("Codex work record {visible_index:02}");
        candidates.push(CodexThreadCandidate {
            candidate_id: candidate_id.clone(),
            display_name: display_name.clone(),
            updated_at: format_timestamp(updated_at)?,
            source_kind: "codex-work-record".to_string(),
        });
        internal.insert(
            candidate_id,
            InternalCandidate {
                raw_thread_id: raw_id.to_string(),
                display_name,
            },
        );
    }

    Ok((
        CodexThreadCatalog {
            catalog_id: catalog_id.clone(),
            candidates,
        },
        InternalCatalog {
            catalog_id,
            candidates: internal,
        },
    ))
}

fn prepare_thread_import(
    result: Value,
    candidate: &InternalCandidate,
) -> Result<(ImportPreview, PreparedImport), String> {
    let thread = result
        .as_object()
        .and_then(|object| object.get("thread"))
        .and_then(Value::as_object)
        .ok_or_else(connector_error)?;
    let returned_id = bounded_identifier(thread.get("id")).ok_or_else(connector_error)?;
    if returned_id != candidate.raw_thread_id {
        return Err(CONNECTOR_ERROR.to_string());
    }
    if !eligible_status(thread_status(thread.get("status")).ok_or_else(connector_error)?) {
        return Err(
            "The selected Codex thread is not available as a completed work record.".into(),
        );
    }
    let updated_at = unix_timestamp(thread.get("updatedAt")).ok_or_else(connector_error)?;
    let turns = thread
        .get("turns")
        .and_then(Value::as_array)
        .ok_or_else(connector_error)?;
    let mut completed_turn = None;
    for turn in turns {
        let turn = turn.as_object().ok_or_else(connector_error)?;
        let _ = bounded_identifier(turn.get("id")).ok_or_else(connector_error)?;
        match turn.get("status").and_then(Value::as_str) {
            Some("completed") => completed_turn = Some(turn),
            Some("interrupted" | "failed" | "inProgress") => {}
            _ => return Err(CONNECTOR_ERROR.to_string()),
        }
    }
    let completed_turn = completed_turn
        .ok_or_else(|| "The selected Codex thread has no completed turn.".to_string())?;
    if !matches!(
        completed_turn.get("itemsView").and_then(Value::as_str),
        None | Some("full")
    ) {
        return Err(CONNECTOR_ERROR.to_string());
    }
    let completed_at = match completed_turn.get("completedAt") {
        None | Some(Value::Null) => updated_at,
        value => unix_timestamp(value).ok_or_else(connector_error)?,
    };
    let source_timestamp = format_timestamp(completed_at)?;
    let items = completed_turn
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(connector_error)?;

    let mut final_answer = None;
    let mut legacy_answer = None;
    for item in items {
        let item = item.as_object().ok_or_else(connector_error)?;
        if item.get("type").and_then(Value::as_str) != Some("agentMessage") {
            continue;
        }
        let item_id = bounded_identifier(item.get("id")).ok_or_else(connector_error)?;
        let text = item
            .get("text")
            .and_then(Value::as_str)
            .ok_or_else(connector_error)?;
        match item.get("phase") {
            Some(Value::String(phase)) if phase == "final_answer" => {
                final_answer = Some((item_id, text));
            }
            Some(Value::String(phase)) if phase == "commentary" => {}
            None | Some(Value::Null) => legacy_answer = Some((item_id, text)),
            _ => return Err(CONNECTOR_ERROR.to_string()),
        }
    }
    let (raw_item_id, raw_text) = final_answer
        .or(legacy_answer)
        .ok_or_else(|| "The selected completed turn has no final answer.".to_string())?;
    let normalized_text = raw_text.trim();
    if normalized_text.is_empty() || normalized_text.len() > MAX_FINAL_ANSWER_BYTES {
        return Err("The selected final answer is empty or too large to import.".to_string());
    }

    let source_id = stable_id(
        "source",
        &[CODEX_THREAD_ADAPTER_ID, &candidate.raw_thread_id],
    );
    let source_record_id = stable_id(
        "record",
        &[&source_id, &candidate.raw_thread_id, raw_item_id],
    );
    let content_hash = sha256(normalized_text.as_bytes());
    let event = NormalizedMemoryEvent {
        id: stable_id("memory", &[&source_id, &source_record_id]),
        schema_version: MEMORY_EVENT_SCHEMA_VERSION,
        source_id: source_id.clone(),
        source_record_id: source_record_id.clone(),
        source_timestamp: source_timestamp.clone(),
        kind: "completion".to_string(),
        normalized_text: normalized_text.to_string(),
        content_hash: content_hash.clone(),
    };
    let source = SourceOption {
        id: source_id.clone(),
        adapter_id: CODEX_THREAD_ADAPTER_ID.to_string(),
        adapter_version: CODEX_THREAD_ADAPTER_VERSION,
        display_name: candidate.display_name.clone(),
        locator: stable_id(
            "locator",
            &[CODEX_THREAD_ADAPTER_ID, &candidate.raw_thread_id],
        ),
        fixture_only: false,
    };
    let consent_scope = ConsentScopeV1 {
        schema_version: 1,
        revision: 1,
        source_id,
        adapter_id: CODEX_THREAD_ADAPTER_ID.to_string(),
        adapter_version: CODEX_THREAD_ADAPTER_VERSION,
        data_categories: vec!["user-confirmed-completion".to_string()],
        purposes: vec!["local-creature-derivation".to_string()],
        read_only: true,
    };
    let consent_scope_json = serde_json::to_string(&consent_scope)
        .map_err(|_| "Memoryling could not bind the consent scope.".to_string())?;
    let consent_scope_hash = sha256(consent_scope_json.as_bytes());
    let prepared = PreparedImport {
        source: source.clone(),
        source_content_hash: sha256(
            format!("{}\0{}", candidate.raw_thread_id, content_hash).as_bytes(),
        ),
        events: vec![event],
        consent_scope: consent_scope.clone(),
        consent_scope_json,
        consent_scope_hash: consent_scope_hash.clone(),
    };
    let preview = ImportPreview {
        preview_id: String::new(),
        source,
        record_count: 1,
        time_range: PreviewTimeRange {
            start: source_timestamp.clone(),
            end: source_timestamp.clone(),
        },
        records: vec![PreviewRecord {
            id: source_record_id,
            source_timestamp,
            kind: "completion".to_string(),
            text_preview: None,
            character_count: normalized_text.chars().count(),
            content_hash,
        }],
        access_scope: AccessScope {
            read_only: true,
            source_write_access: false,
            network_access: false,
            arbitrary_path_access: false,
        },
        consent_scope,
        consent_scope_hash,
    };
    Ok((preview, prepared))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThreadStatus {
    Active,
    Idle,
    NotLoaded,
    SystemError,
}

fn thread_status(value: Option<&Value>) -> Option<ThreadStatus> {
    match value?.get("type")?.as_str()? {
        "active" => Some(ThreadStatus::Active),
        "idle" => Some(ThreadStatus::Idle),
        "notLoaded" => Some(ThreadStatus::NotLoaded),
        "systemError" => Some(ThreadStatus::SystemError),
        _ => None,
    }
}

fn eligible_status(status: ThreadStatus) -> bool {
    matches!(status, ThreadStatus::Idle | ThreadStatus::NotLoaded)
}

fn bounded_identifier(value: Option<&Value>) -> Option<&str> {
    let value = value?.as_str()?;
    (!value.trim().is_empty() && value.len() <= MAX_IDENTIFIER_BYTES).then_some(value)
}

fn unix_timestamp(value: Option<&Value>) -> Option<i64> {
    let timestamp = value?.as_i64()?;
    OffsetDateTime::from_unix_timestamp(timestamp)
        .ok()
        .map(|_| timestamp)
}

fn format_timestamp(timestamp: i64) -> Result<String, String> {
    let value =
        OffsetDateTime::from_unix_timestamp(timestamp).map_err(|_| CONNECTOR_ERROR.to_string())?;
    value
        .format(&Rfc3339)
        .map_err(|_| CONNECTOR_ERROR.to_string())
}

fn connector_error() -> String {
    CONNECTOR_ERROR.to_string()
}

fn stable_id(prefix: &str, values: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update([0]);
    }
    let hash = hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("{prefix}_{}", &hash[..24])
}

fn sha256(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn run_app_server_call(method: &str, params: Value) -> Result<Value, String> {
    let deadline = Instant::now() + APP_SERVER_TIMEOUT;
    let executable = codex_executable()?;
    verify_codex_version(&executable, deadline)?;
    remaining_until(deadline)?;
    let mut command = Command::new(&executable);
    command
        .arg("app-server")
        .arg("--listen")
        .arg("stdio://")
        .current_dir(executable.parent().ok_or_else(connector_error)?)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    configure_hidden(&mut command);
    let mut child = command.spawn().map_err(|_| CONNECTOR_ERROR.to_string())?;
    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            let _ = terminate(&mut child);
            return Err(CONNECTOR_ERROR.to_string());
        }
    };
    let receiver = spawn_stdout_reader(stdout);

    let operation = (|| {
        write_message(
            &mut child,
            &json!({
                "method": "initialize",
                "id": 1,
                "params": {
                    "clientInfo": {
                        "name": "memoryling",
                        "title": "Memoryling",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }
            }),
        )?;
        let initialized = wait_for_response(&receiver, 1, deadline)?;
        let initialized = initialized.as_object().ok_or_else(connector_error)?;
        for field in ["codexHome", "platformFamily", "platformOs", "userAgent"] {
            if bounded_identifier(initialized.get(field)).is_none() {
                return Err(CONNECTOR_ERROR.to_string());
            }
        }
        if initialized.get("platformOs").and_then(Value::as_str) != Some("windows") {
            return Err(CONNECTOR_ERROR.to_string());
        }
        write_message(&mut child, &json!({ "method": "initialized" }))?;
        write_message(
            &mut child,
            &json!({ "method": method, "id": 2, "params": params }),
        )?;
        wait_for_response(&receiver, 2, deadline)
    })();
    let cleanup = terminate(&mut child);
    match (operation, cleanup) {
        (_, Err(_)) => Err(CONNECTOR_ERROR.to_string()),
        (result, Ok(())) => result,
    }
}

fn verify_codex_version(executable: &Path, deadline: Instant) -> Result<(), String> {
    let remaining = remaining_until(deadline)?;
    let mut command = Command::new(executable);
    command
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    configure_hidden(&mut command);
    let mut child = command.spawn().map_err(|_| CONNECTOR_ERROR.to_string())?;
    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            let _ = terminate(&mut child);
            return Err(CONNECTOR_ERROR.to_string());
        }
    };
    let receiver = spawn_stdout_reader(stdout);
    let result = match receiver.recv_timeout(remaining) {
        Ok(OutputMessage::Line(bytes)) => std::str::from_utf8(&bytes)
            .ok()
            .map(str::trim)
            .filter(|line| *line == format!("codex-cli {EXPECTED_CODEX_VERSION}"))
            .map(|_| ())
            .ok_or_else(|| UNSUPPORTED_VERSION_ERROR.to_string()),
        _ => Err(CONNECTOR_ERROR.to_string()),
    };
    let cleanup = terminate(&mut child);
    match (result, cleanup) {
        (_, Err(_)) => Err(CONNECTOR_ERROR.to_string()),
        (result, Ok(())) => result,
    }
}

fn configure_hidden(command: &mut Command) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000);
    }
}

fn codex_executable() -> Result<PathBuf, String> {
    let local_app_data = std::env::var_os("LOCALAPPDATA").ok_or_else(connector_error)?;
    codex_executable_from(local_app_data)
}

fn codex_executable_from(local_app_data: OsString) -> Result<PathBuf, String> {
    let executable = Path::new(&local_app_data)
        .join("Programs")
        .join("OpenAI")
        .join("Codex")
        .join("bin")
        .join("codex.exe");
    executable
        .is_file()
        .then_some(executable)
        .ok_or_else(connector_error)
}

fn write_message(child: &mut Child, message: &Value) -> Result<(), String> {
    let stdin = child.stdin.as_mut().ok_or_else(connector_error)?;
    serde_json::to_writer(&mut *stdin, message).map_err(|_| CONNECTOR_ERROR.to_string())?;
    stdin
        .write_all(b"\n")
        .and_then(|_| stdin.flush())
        .map_err(|_| CONNECTOR_ERROR.to_string())
}

enum OutputMessage {
    Line(Vec<u8>),
    TooLarge,
    End,
}

fn spawn_stdout_reader(mut stdout: impl Read + Send + 'static) -> Receiver<OutputMessage> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let mut chunk = [0_u8; 8192];
        let mut line = Vec::new();
        let mut total = 0usize;
        let mut oversized = false;
        loop {
            let read = match stdout.read(&mut chunk) {
                Ok(0) | Err(_) => break,
                Ok(read) => read,
            };
            total = total.saturating_add(read);
            if total > MAX_STDOUT_BYTES {
                if !oversized {
                    let _ = sender.send(OutputMessage::TooLarge);
                    oversized = true;
                }
                continue;
            }
            for byte in &chunk[..read] {
                if *byte == b'\n' {
                    let completed = std::mem::take(&mut line);
                    if sender.send(OutputMessage::Line(completed)).is_err() {
                        return;
                    }
                } else if line.len() < MAX_STDOUT_LINE_BYTES {
                    line.push(*byte);
                } else if !oversized {
                    let _ = sender.send(OutputMessage::TooLarge);
                    oversized = true;
                }
            }
        }
        if !line.is_empty() && !oversized {
            let _ = sender.send(OutputMessage::Line(line));
        }
        let _ = sender.send(OutputMessage::End);
    });
    receiver
}

fn wait_for_response(
    receiver: &Receiver<OutputMessage>,
    expected_id: i64,
    deadline: Instant,
) -> Result<Value, String> {
    loop {
        let remaining = remaining_until(deadline)?;
        let message = receiver
            .recv_timeout(remaining)
            .map_err(|_| CONNECTOR_ERROR.to_string())?;
        let bytes = match message {
            OutputMessage::Line(bytes) => bytes,
            OutputMessage::TooLarge | OutputMessage::End => return Err(CONNECTOR_ERROR.to_string()),
        };
        let value: Value =
            serde_json::from_slice(&bytes).map_err(|_| CONNECTOR_ERROR.to_string())?;
        let object = value.as_object().ok_or_else(connector_error)?;
        match object.get("id").and_then(Value::as_i64) {
            Some(id) if id == expected_id => {
                if object.get("error").is_some() {
                    return Err(CONNECTOR_ERROR.to_string());
                }
                return object.get("result").cloned().ok_or_else(connector_error);
            }
            Some(_) => return Err(CONNECTOR_ERROR.to_string()),
            None if object.get("method").and_then(Value::as_str).is_some() => continue,
            None => return Err(CONNECTOR_ERROR.to_string()),
        }
    }
}

fn remaining_until(deadline: Instant) -> Result<Duration, String> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .ok_or_else(connector_error)
}

trait TerminationTarget {
    fn close_stdin(&mut self);
    fn has_exited(&mut self) -> io::Result<bool>;
    fn kill(&mut self) -> io::Result<()>;
}

impl TerminationTarget for Child {
    fn close_stdin(&mut self) {
        let _ = self.stdin.take();
    }

    fn has_exited(&mut self) -> io::Result<bool> {
        self.try_wait().map(|status| status.is_some())
    }

    fn kill(&mut self) -> io::Result<()> {
        Child::kill(self)
    }
}

fn terminate(child: &mut Child) -> Result<(), String> {
    terminate_target(child, PROCESS_CLEANUP_TIMEOUT)
}

fn terminate_target(
    child: &mut impl TerminationTarget,
    cleanup_timeout: Duration,
) -> Result<(), String> {
    child.close_stdin();
    if child
        .has_exited()
        .map_err(|_| CONNECTOR_ERROR.to_string())?
    {
        return Ok(());
    }
    child.kill().map_err(|_| CONNECTOR_ERROR.to_string())?;
    let deadline = Instant::now() + cleanup_timeout;
    loop {
        if child
            .has_exited()
            .map_err(|_| CONNECTOR_ERROR.to_string())?
        {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(CONNECTOR_ERROR.to_string());
        }
        thread::sleep(Duration::from_millis(10));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    enum FakeWait {
        Running,
        Exited,
        Error,
    }

    struct FakeTerminationTarget {
        waits: VecDeque<FakeWait>,
        kill_succeeds: bool,
        stdin_closed: bool,
        kill_called: bool,
    }

    impl FakeTerminationTarget {
        fn new(waits: impl IntoIterator<Item = FakeWait>, kill_succeeds: bool) -> Self {
            Self {
                waits: waits.into_iter().collect(),
                kill_succeeds,
                stdin_closed: false,
                kill_called: false,
            }
        }
    }

    impl TerminationTarget for FakeTerminationTarget {
        fn close_stdin(&mut self) {
            self.stdin_closed = true;
        }

        fn has_exited(&mut self) -> io::Result<bool> {
            match self.waits.pop_front().unwrap_or(FakeWait::Running) {
                FakeWait::Running => Ok(false),
                FakeWait::Exited => Ok(true),
                FakeWait::Error => Err(io::Error::other("wait failed")),
            }
        }

        fn kill(&mut self) -> io::Result<()> {
            self.kill_called = true;
            self.kill_succeeds
                .then_some(())
                .ok_or_else(|| io::Error::other("kill failed"))
        }
    }

    fn status(kind: &str) -> Value {
        json!({ "type": kind })
    }

    fn candidate() -> InternalCandidate {
        InternalCandidate {
            raw_thread_id: "thr_private_123".to_string(),
            display_name: "Codex work record 01".to_string(),
        }
    }

    fn read_result(items: Value) -> Value {
        json!({
            "thread": {
                "id": "thr_private_123",
                "updatedAt": 1_786_476_600_i64,
                "status": status("idle"),
                "turns": [
                    { "id": "turn-old", "status": "completed", "itemsView": "full", "completedAt": 1_786_476_500_i64, "items": [
                        { "type": "agentMessage", "id": "old", "text": "old", "phase": "final_answer" }
                    ]},
                    { "id": "turn-interrupted", "status": "interrupted", "itemsView": "full", "items": [
                        { "type": "agentMessage", "id": "ignored", "text": "ignored", "phase": "final_answer" }
                    ]},
                    { "id": "turn-final", "status": "completed", "itemsView": "full", "completedAt": 1_786_476_550_i64, "items": items }
                ]
            }
        })
    }

    #[test]
    fn list_request_is_bounded_state_db_only_and_interactive() {
        assert_eq!(
            thread_list_params(),
            json!({
                "cursor": null,
                "limit": 25,
                "sortKey": "updated_at",
                "sortDirection": "desc",
                "sourceKinds": ["cli", "vscode"],
                "archived": false,
                "useStateDbOnly": true
            })
        );
    }

    #[test]
    fn catalog_excludes_active_threads_and_never_serializes_private_metadata() {
        let raw_id = "thr_private_123";
        let raw_title = "Secret client title";
        let raw_path = r"C:\Users\private\project";
        let result = json!({
            "data": [
                {
                    "id": raw_id,
                    "updatedAt": 1_786_476_600_i64,
                    "status": status("idle"),
                    "name": raw_title,
                    "preview": "private summary",
                    "cwd": raw_path
                },
                {
                    "id": "thr_active",
                    "updatedAt": 1_786_476_601_i64,
                    "status": status("active")
                },
                {
                    "id": "thr_system_error",
                    "updatedAt": 1_786_476_602_i64,
                    "status": status("systemError")
                }
            ],
            "nextCursor": null
        });
        let (public, internal) = parse_thread_list(result, "nonce").expect("list should parse");
        assert_eq!(public.candidates.len(), 1);
        assert_eq!(internal.candidates.len(), 1);
        let encoded = serde_json::to_string(&public).expect("catalog should serialize");
        for secret in [
            raw_id,
            raw_title,
            raw_path,
            "private summary",
            "thr_active",
            "thr_system_error",
        ] {
            assert!(!encoded.contains(secret));
        }
        assert!(public.candidates[0].candidate_id.starts_with("candidate_"));
    }

    #[test]
    fn selected_preview_uses_last_completed_final_and_redacts_content() {
        let private_text = "A private final answer with unicode 記憶。";
        let result = read_result(json!([
            { "type": "agentMessage", "id": "legacy", "text": "legacy", "phase": null },
            { "type": "agentMessage", "id": "comment", "text": "comment", "phase": "commentary" },
            { "type": "agentMessage", "id": "final", "text": private_text, "phase": "final_answer" }
        ]));
        let (preview, prepared) =
            prepare_thread_import(result, &candidate()).expect("preview should parse");
        assert_eq!(prepared.events[0].normalized_text, private_text);
        assert_eq!(preview.records[0].text_preview, None);
        assert_eq!(
            preview.records[0].character_count,
            private_text.chars().count()
        );
        assert_eq!(preview.records[0].content_hash.len(), 64);
        assert_eq!(preview.consent_scope_hash, prepared.consent_scope_hash);
        let public_json = serde_json::to_string(&preview).expect("preview should serialize");
        for private in [private_text, "thr_private_123", "final"] {
            assert!(!public_json.contains(private));
        }
        assert!(!public_json.contains("textPreview"));
    }

    #[test]
    fn legacy_null_phase_is_a_fallback_but_unknown_phase_fails_closed() {
        let (preview, prepared) = prepare_thread_import(
            read_result(json!([
                { "type": "agentMessage", "id": "legacy", "text": "legacy answer", "phase": null }
            ])),
            &candidate(),
        )
        .expect("legacy answer should be supported narrowly");
        assert_eq!(preview.records[0].character_count, 13);
        assert_eq!(prepared.events[0].normalized_text, "legacy answer");

        assert!(prepare_thread_import(
            read_result(json!([
                { "type": "agentMessage", "id": "future", "text": "future", "phase": "future_phase" }
            ])),
            &candidate(),
        )
        .is_err());
    }

    #[test]
    fn invalid_mismatched_active_empty_and_oversize_responses_fail_closed() {
        let mut mismatched = read_result(json!([
            { "type": "agentMessage", "id": "final", "text": "answer", "phase": "final_answer" }
        ]));
        mismatched["thread"]["id"] = json!("thr_other");
        assert!(prepare_thread_import(mismatched, &candidate()).is_err());

        let mut active = read_result(json!([
            { "type": "agentMessage", "id": "final", "text": "answer", "phase": "final_answer" }
        ]));
        active["thread"]["status"] = status("active");
        assert!(prepare_thread_import(active, &candidate()).is_err());

        let mut failed = read_result(json!([
            { "type": "agentMessage", "id": "final", "text": "answer", "phase": "final_answer" }
        ]));
        failed["thread"]["status"] = status("systemError");
        assert!(prepare_thread_import(failed, &candidate()).is_err());

        assert!(prepare_thread_import(
            read_result(json!([
                { "type": "agentMessage", "id": "final", "text": "   ", "phase": "final_answer" }
            ])),
            &candidate(),
        )
        .is_err());
        let oversize = "x".repeat(MAX_FINAL_ANSWER_BYTES + 1);
        assert!(prepare_thread_import(
            read_result(json!([
                { "type": "agentMessage", "id": "final", "text": oversize, "phase": "final_answer" }
            ])),
            &candidate(),
        )
        .is_err());
    }

    #[test]
    fn consent_scope_json_is_canonical_and_hash_bound() {
        let (_, prepared) = prepare_thread_import(
            read_result(json!([
                { "type": "agentMessage", "id": "final", "text": "answer", "phase": "final_answer" }
            ])),
            &candidate(),
        )
        .expect("preview should parse");
        assert_eq!(
            serde_json::to_string(&prepared.consent_scope).expect("scope should serialize"),
            prepared.consent_scope_json
        );
        assert_eq!(
            sha256(prepared.consent_scope_json.as_bytes()),
            prepared.consent_scope_hash
        );
        assert_eq!(
            prepared.consent_scope.data_categories,
            ["user-confirmed-completion"]
        );
    }

    #[test]
    fn process_cleanup_is_bounded_and_reports_kill_or_wait_failures() {
        let mut exited = FakeTerminationTarget::new([FakeWait::Exited], true);
        assert!(terminate_target(&mut exited, Duration::ZERO).is_ok());
        assert!(exited.stdin_closed);
        assert!(!exited.kill_called);

        let mut killed = FakeTerminationTarget::new([FakeWait::Running, FakeWait::Exited], true);
        assert!(terminate_target(&mut killed, Duration::from_millis(20)).is_ok());
        assert!(killed.stdin_closed);
        assert!(killed.kill_called);

        let mut kill_failure = FakeTerminationTarget::new([FakeWait::Running], false);
        assert!(terminate_target(&mut kill_failure, Duration::ZERO).is_err());

        let mut wait_failure =
            FakeTerminationTarget::new([FakeWait::Running, FakeWait::Error], true);
        assert!(terminate_target(&mut wait_failure, Duration::from_millis(20)).is_err());

        let mut never_exits =
            FakeTerminationTarget::new([FakeWait::Running, FakeWait::Running], true);
        assert!(terminate_target(&mut never_exits, Duration::ZERO).is_err());
    }

    #[test]
    fn version_and_app_server_share_one_operation_deadline() {
        assert!(remaining_until(Instant::now() + Duration::from_secs(1)).is_ok());
        assert!(remaining_until(Instant::now()).is_err());
    }

    #[cfg(target_os = "windows")]
    #[test]
    #[ignore = "requires the exact supported Codex Desktop CLI and reads only its state-DB catalog"]
    fn live_list_smoke_returns_only_content_minimized_candidates() {
        let (public, internal) = load_catalog().expect("supported local list should load");
        assert_eq!(public.catalog_id, internal.catalog_id());
        assert!(public.candidates.len() <= THREAD_LIMIT as usize);
        let encoded = serde_json::to_string(&public).expect("catalog should serialize");
        for forbidden in ["cwd", "path", "preview", "threadId", "sessionId"] {
            assert!(!encoded.contains(forbidden));
        }
    }
}
