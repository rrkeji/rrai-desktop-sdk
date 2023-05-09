mod abilities;
mod constants;
mod handlers;
mod migration;
mod models;
mod utils;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    //
    tauri::async_runtime::block_on(async move {
        //执行数据库脚本
        tracing::debug!("执行数据库脚本");
        migration::init_database().await
    })
    .expect("执行数据库脚本失败！");

    Builder::new("rrai-ability")
        .invoke_handler(tauri::generate_handler![
            handlers::auto_scan,
            handlers::list_abilities,
            handlers::insert_ability,
            handlers::update_ability,
            handlers::delete_ability,
        ])
        .setup(|app| {
            // app.manage(SqliteMap::default());
            Ok(())
        })
        .build()
}
