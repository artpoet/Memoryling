mod adapter;
mod codex_thread;
mod model;
mod store;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[cfg(test)]
pub(crate) static SENSITIVE_HANDLER_ENTRIES: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[cfg(test)]
fn record_sensitive_handler_entry() {
    SENSITIVE_HANDLER_ENTRIES.fetch_add(1, Ordering::SeqCst);
}

#[cfg(not(test))]
fn record_sensitive_handler_entry() {}

use tauri::{path::BaseDirectory, AppHandle, Manager, State};

use crate::caller::{MainCaller, RenderCaller};

pub(crate) use model::{
    ApproveImportRequest, CodexThreadCatalog, CreatureRenderState, ImportPreview, MemoryState,
    SourceOption,
};

use codex_thread::InternalCatalog;
use model::PreparedImport;
use store::MemoryStore;

const CATALOG_TTL: Duration = Duration::from_secs(10 * 60);
const PREVIEW_TTL: Duration = Duration::from_secs(10 * 60);

struct PendingImport {
    prepared: PreparedImport,
    created_at: Instant,
}

struct PendingCatalog {
    catalog: InternalCatalog,
    created_at: Instant,
}

#[derive(Default)]
pub(crate) struct PendingImports {
    imports: Mutex<HashMap<String, PendingImport>>,
    catalogs: Mutex<HashMap<String, PendingCatalog>>,
    sequence: AtomicU64,
    generation: AtomicU64,
    source_operation: Arc<Mutex<()>>,
}

impl PendingImports {
    fn insert(&self, prepared: PreparedImport) -> Result<String, String> {
        self.insert_if_current(self.generation(), prepared)
    }

    fn insert_if_current(
        &self,
        expected_generation: u64,
        prepared: PreparedImport,
    ) -> Result<String, String> {
        let sequence = self.sequence.fetch_add(1, Ordering::Relaxed).to_string();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| "Memoryling could not create a preview token.".to_string())?
            .as_nanos()
            .to_string();
        let preview_id = adapter::stable_id(
            "preview",
            &[&prepared.source_content_hash, &sequence, &timestamp],
        );
        let mut imports = self
            .imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?;
        if self.generation() != expected_generation {
            return Err("The detail session changed. Start the preview again.".to_string());
        }
        imports.clear();
        imports.insert(
            preview_id.clone(),
            PendingImport {
                prepared,
                created_at: Instant::now(),
            },
        );
        Ok(preview_id)
    }

    fn use_for_approval<T>(
        &self,
        preview_id: &str,
        source_id: &str,
        operation: impl FnOnce(&PreparedImport) -> Result<T, String>,
    ) -> Result<T, String> {
        let mut imports = self
            .imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?;
        imports.retain(|_, entry| entry.created_at.elapsed() <= PREVIEW_TTL);
        let prepared = imports
            .get(preview_id)
            .ok_or_else(|| "The import preview expired. Preview the source again.".to_string())?;
        if prepared.prepared.source.id != source_id {
            return Err("The import preview does not match the selected source.".to_string());
        }
        let result = operation(&prepared.prepared)?;
        imports.remove(preview_id);
        Ok(result)
    }

    fn discard(&self, preview_id: &str) -> Result<(), String> {
        self.imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?
            .remove(preview_id);
        self.catalogs
            .lock()
            .map_err(|_| "Memoryling could not access the Codex work-record catalog.".to_string())?
            .remove(preview_id);
        Ok(())
    }

    fn replace_catalog(
        &self,
        expected_generation: u64,
        catalog: InternalCatalog,
    ) -> Result<(), String> {
        let catalog_id = catalog.catalog_id().to_string();
        let mut catalogs = self.catalogs.lock().map_err(|_| {
            "Memoryling could not access the Codex work-record catalog.".to_string()
        })?;
        if self.generation() != expected_generation {
            return Err("The detail session changed. Browse again.".to_string());
        }
        catalogs.clear();
        catalogs.insert(
            catalog_id,
            PendingCatalog {
                catalog,
                created_at: Instant::now(),
            },
        );
        Ok(())
    }

    fn take_catalog_for_preview(&self, catalog_id: &str) -> Result<InternalCatalog, String> {
        let mut catalogs = self.catalogs.lock().map_err(|_| {
            "Memoryling could not access the Codex work-record catalog.".to_string()
        })?;
        catalogs.retain(|_, entry| entry.created_at.elapsed() <= CATALOG_TTL);
        catalogs
            .remove(catalog_id)
            .map(|entry| entry.catalog)
            .ok_or_else(|| "The Codex work-record catalog expired. Browse again.".to_string())
    }

    fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }

    fn source_operation(&self) -> Arc<Mutex<()>> {
        Arc::clone(&self.source_operation)
    }

    pub(crate) fn clear_all(&self) -> Result<(), String> {
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?
            .clear();
        self.catalogs
            .lock()
            .map_err(|_| "Memoryling could not access the Codex work-record catalog.".to_string())?
            .clear();
        Ok(())
    }
}

#[tauri::command]
pub(crate) fn list_memory_sources(_caller: MainCaller) -> Result<Vec<SourceOption>, String> {
    record_sensitive_handler_entry();
    adapter::list_sources()
}

#[tauri::command]
pub(crate) fn preview_memory_source<R: tauri::Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    pending: State<'_, PendingImports>,
    source_id: String,
) -> Result<ImportPreview, String> {
    record_sensitive_handler_entry();
    let source_path = app
        .path()
        .resolve(
            "fixtures/codex-first-memory-v1.json",
            BaseDirectory::Resource,
        )
        .map_err(|_| "Memoryling could not resolve its bundled synthetic source.".to_string())?;
    let (mut preview, prepared) = adapter::preview_source(&source_id, &source_path)?;
    preview.preview_id = pending.insert(prepared)?;
    Ok(preview)
}

#[tauri::command]
pub(crate) async fn list_codex_threads(
    _caller: MainCaller,
    pending: State<'_, PendingImports>,
) -> Result<CodexThreadCatalog, String> {
    record_sensitive_handler_entry();
    let generation = pending.generation();
    let (catalog, internal) = tauri::async_runtime::spawn_blocking(codex_thread::load_catalog)
        .await
        .map_err(|_| "The local Codex work-record operation stopped unexpectedly.".to_string())??;
    pending.replace_catalog(generation, internal)?;
    Ok(catalog)
}

#[tauri::command]
pub(crate) async fn preview_codex_thread<R: tauri::Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    pending: State<'_, PendingImports>,
    catalog_id: String,
    candidate_id: String,
) -> Result<ImportPreview, String> {
    record_sensitive_handler_entry();
    let generation = pending.generation();
    let catalog = pending.take_catalog_for_preview(&catalog_id)?;
    let source_operation = pending.source_operation();
    let app_for_read = app.clone();
    let catalog_id_for_read = catalog_id.clone();
    let (mut preview, prepared) = tauri::async_runtime::spawn_blocking(move || {
        let _source_guard = source_operation
            .lock()
            .map_err(|_| "Memoryling could not reserve the local source operation.".to_string())?;
        store_for(&app_for_read)?.ensure_no_approved_source()?;
        codex_thread::preview_selected_thread(&catalog, &catalog_id_for_read, &candidate_id)
    })
    .await
    .map_err(|_| "The local Codex work-record operation stopped unexpectedly.".to_string())??;
    let source_operation = pending.source_operation();
    let _source_guard = source_operation
        .lock()
        .map_err(|_| "Memoryling could not reserve the local source operation.".to_string())?;
    store_for(&app)?.ensure_no_approved_source()?;
    preview.preview_id = pending.insert_if_current(generation, prepared)?;
    Ok(preview)
}

#[tauri::command]
pub(crate) fn cancel_memory_preview(
    _caller: MainCaller,
    pending: State<'_, PendingImports>,
    preview_id: String,
) -> Result<(), String> {
    record_sensitive_handler_entry();
    pending.discard(&preview_id)
}

#[tauri::command]
pub(crate) fn get_memory_state<R: tauri::Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
) -> Result<MemoryState, String> {
    record_sensitive_handler_entry();
    store_for(&app)?.state()
}

#[tauri::command]
pub(crate) fn approve_memory_import<R: tauri::Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    pending: State<'_, PendingImports>,
    request: ApproveImportRequest,
) -> Result<MemoryState, String> {
    record_sensitive_handler_entry();
    let source_operation = pending.source_operation();
    let _source_guard = source_operation
        .lock()
        .map_err(|_| "Memoryling could not reserve the local source operation.".to_string())?;
    let store = store_for(&app)?;
    let state = pending.use_for_approval(&request.preview_id, &request.source_id, |prepared| {
        let provided_scope_hash = request.consent_scope_hash.as_deref();
        if provided_scope_hash.is_some_and(|hash| hash != prepared.consent_scope_hash.as_str())
            || (prepared.source.adapter_id == model::CODEX_THREAD_ADAPTER_ID
                && provided_scope_hash != Some(prepared.consent_scope_hash.as_str()))
        {
            return Err(
                "The consent scope changed. Preview the selected source again.".to_string(),
            );
        }
        store.approve_import(prepared, &request.selected_record_ids)
    })?;
    crate::desktop_shell::emit_creature_state_changed(&app);
    Ok(state)
}

#[tauri::command]
pub(crate) fn forget_memory_source<R: tauri::Runtime>(
    _caller: MainCaller,
    app: AppHandle<R>,
    source_id: String,
) -> Result<MemoryState, String> {
    record_sensitive_handler_entry();
    let pending = app
        .try_state::<PendingImports>()
        .ok_or_else(|| "Memoryling could not access the local source operation.".to_string())?;
    let source_operation = pending.source_operation();
    let _source_guard = source_operation
        .lock()
        .map_err(|_| "Memoryling could not reserve the local source operation.".to_string())?;
    let state = store_for(&app)?.forget_source(&source_id)?;
    crate::desktop_shell::emit_creature_state_changed(&app);
    Ok(state)
}

#[tauri::command]
pub(crate) fn get_creature_render_state(
    _caller: RenderCaller,
    app: AppHandle,
) -> Result<CreatureRenderState, String> {
    store_for(&app)?.creature_render_state()
}

pub(crate) fn store_for<R: tauri::Runtime>(app: &AppHandle<R>) -> Result<MemoryStore, String> {
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|_| "Memoryling could not resolve its local app-data directory.".to_string())?;
    Ok(MemoryStore::new(directory.join("memoryling.sqlite3")))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn fixture_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/codex-first-memory-v1.json")
    }

    #[test]
    fn pending_preview_token_binds_approval_to_the_prepared_source() {
        let pending = PendingImports::default();
        let prepared = adapter::prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse");
        let preview_id = pending
            .insert(prepared.clone())
            .expect("preview should be stored");

        assert!(pending
            .use_for_approval(&preview_id, "codex.other", |_| Ok(()))
            .is_err());
        assert!(pending
            .use_for_approval(&preview_id, "codex.synthetic.first-memory", |_| {
                Err::<(), _>("temporary store failure".to_string())
            })
            .is_err());

        let recovered_source_id = pending
            .use_for_approval(&preview_id, "codex.synthetic.first-memory", |recovered| {
                Ok(recovered.source.id.clone())
            })
            .expect("a failed store attempt must leave the preview retryable");
        assert_eq!(recovered_source_id, prepared.source.id);
        assert!(pending
            .use_for_approval(&preview_id, "codex.synthetic.first-memory", |_| Ok(()))
            .is_err());
    }

    #[test]
    fn clearing_detail_session_invalidates_late_async_preview_results() {
        let pending = PendingImports::default();
        let generation = pending.generation();
        let prepared = adapter::prepare_import("codex.synthetic.first-memory", &fixture_path())
            .expect("fixture should parse");

        pending.clear_all().expect("session should clear");
        assert_eq!(pending.generation(), generation + 1);
        assert!(pending
            .insert_if_current(generation, prepared)
            .expect_err("late preview result must not re-enter pending state")
            .contains("session changed"));
    }
}
