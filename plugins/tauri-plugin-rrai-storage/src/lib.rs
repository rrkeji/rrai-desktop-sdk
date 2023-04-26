mod constants;
mod files;
mod handlers;
mod migration;
mod models;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    //执行数据库脚本
    tauri::async_runtime::block_on(async move {
        //执行数据库脚本
        tracing::debug!("执行数据库脚本");
        migration::init_database().await
    })
    .expect("执行数据库脚本失败！");

    Builder::new("rrai-storage")
        .invoke_handler(tauri::generate_handler![
            handlers::list_files,
            handlers::list_files_by_category,
            handlers::insert_file,
            handlers::update_file,
            handlers::delete_file,
            handlers::create_dir,
        ])
        .build()
}
