mod constants;
mod dataset;
mod handlers;
mod ipfs;
mod meta;
mod request;
pub mod task;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rrai-idns")
        .invoke_handler(tauri::generate_handler![
            handlers::set_context_value,
            handlers::get_context_value,
            handlers::schema_by_model,
            //
            handlers::dataset_create_row,
            handlers::update_dataset_row,
            handlers::query_dataset_row,
            handlers::remove_dataset_row,
            handlers::dataset_rows_search,
            handlers::dataset_rows_search_by_model,
            handlers::dataset_rows_search_owned,
            handlers::dataset_create_by_model_id,
            //
            handlers::tasks_task_publish,
            handlers::tasks_worker_wakeup,
            handlers::tasks_task_take,
            handlers::tasks_task_query_by_id,
            handlers::tasks_task_process_result,
            //
            handlers::ipfs_files_search,
            handlers::ipfs_files_create_with_local_file,
            handlers::ipfs_files_create_with_bytes_content,
            handlers::ipfs_files_create_with_string_content,
            handlers::ipfs_files_create,
            handlers::ipfs_files_mkdirs,
            handlers::ipfs_files_update,
            handlers::ipfs_files_remove,
            handlers::ipfs_string_content,
            handlers::ipfs_pins_status,
            handlers::ipfs_pins_unpin,
            handlers::ipfs_pins_pin,
        ])
        .setup(|app| {
            app.manage(handlers::ContextState::default());
            Ok(())
        })
        .build()
}
