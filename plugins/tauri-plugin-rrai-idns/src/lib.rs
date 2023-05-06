mod constants;
mod dataset;
mod handlers;
mod meta;
mod request;

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
            handlers::insert_dataset_row,
            handlers::update_dataset_row,
            handlers::query_dataset_row,
            handlers::remove_dataset_row,
            handlers::dataset_rows,
        ])
        .setup(|app| {
            app.manage(handlers::ContextState::default());
            Ok(())
        })
        .build()
}
