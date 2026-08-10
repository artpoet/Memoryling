mod memory;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(memory::PendingImports::default())
        .invoke_handler(tauri::generate_handler![
            memory::list_memory_sources,
            memory::preview_memory_source,
            memory::cancel_memory_preview,
            memory::get_memory_state,
            memory::approve_memory_import,
            memory::forget_memory_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
