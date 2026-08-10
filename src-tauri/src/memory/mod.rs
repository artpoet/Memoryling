mod adapter;
mod model;
mod store;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{path::BaseDirectory, AppHandle, Manager, State};

pub(crate) use model::{ApproveImportRequest, ImportPreview, MemoryState, SourceOption};

use model::PreparedImport;
use store::MemoryStore;

#[derive(Default)]
pub(crate) struct PendingImports {
    imports: Mutex<HashMap<String, PreparedImport>>,
    sequence: AtomicU64,
}

impl PendingImports {
    fn insert(&self, prepared: PreparedImport) -> Result<String, String> {
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
        let source_id = prepared.source.id.clone();
        let mut imports = self
            .imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?;
        imports.retain(|_, import| import.source.id != source_id);
        imports.insert(preview_id.clone(), prepared);
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
        let prepared = imports
            .get(preview_id)
            .ok_or_else(|| "The import preview expired. Preview the source again.".to_string())?;
        if prepared.source.id != source_id {
            return Err("The import preview does not match the selected source.".to_string());
        }
        let result = operation(prepared)?;
        imports.remove(preview_id);
        Ok(result)
    }

    fn discard(&self, preview_id: &str) -> Result<(), String> {
        self.imports
            .lock()
            .map_err(|_| "Memoryling could not access the pending preview.".to_string())?
            .remove(preview_id);
        Ok(())
    }
}

#[tauri::command]
pub(crate) fn list_memory_sources() -> Result<Vec<SourceOption>, String> {
    adapter::list_sources()
}

#[tauri::command]
pub(crate) fn preview_memory_source(
    app: AppHandle,
    pending: State<'_, PendingImports>,
    source_id: String,
) -> Result<ImportPreview, String> {
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
pub(crate) fn cancel_memory_preview(
    pending: State<'_, PendingImports>,
    preview_id: String,
) -> Result<(), String> {
    pending.discard(&preview_id)
}

#[tauri::command]
pub(crate) fn get_memory_state(app: AppHandle) -> Result<MemoryState, String> {
    store_for(&app)?.state()
}

#[tauri::command]
pub(crate) fn approve_memory_import(
    app: AppHandle,
    pending: State<'_, PendingImports>,
    request: ApproveImportRequest,
) -> Result<MemoryState, String> {
    let store = store_for(&app)?;
    pending.use_for_approval(&request.preview_id, &request.source_id, |prepared| {
        store.approve_import(prepared, &request.selected_record_ids)
    })
}

#[tauri::command]
pub(crate) fn forget_memory_source(
    app: AppHandle,
    source_id: String,
) -> Result<MemoryState, String> {
    adapter::ensure_supported_source(&source_id)?;
    store_for(&app)?.forget_source(&source_id)
}

fn store_for(app: &AppHandle) -> Result<MemoryStore, String> {
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
}
