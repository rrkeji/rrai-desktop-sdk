pub mod abilities;
mod constants;
mod handlers;
mod migration;
mod models;
mod tasks;
mod terminal;
mod utils;
mod workspaces;

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
        let res = migration::init_database().await;
        res
    })
    .expect("执行数据库脚本失败！");

    Builder::new("rrai-ability")
        .invoke_handler(tauri::generate_handler![
            handlers::env_auto_scan,
            handlers::perform_task_status,
            handlers::perform_task,
            //
            handlers::list_abilities,
            handlers::insert_ability,
            handlers::update_ability,
            handlers::update_ability_settings,
            handlers::delete_ability,
            //
            handlers::list_tasks,
            handlers::query_task_by_task_id,
            handlers::query_task_status,
            handlers::delete_task,
            //
            handlers::new_terminal,
            handlers::terminal_execute_command,
            handlers::terminal_stdout,
            handlers::terminal_stderr,
            handlers::terminal_status,
            handlers::terminal_interrupt_command,
            handlers::close_terminal,
        ])
        .setup(|app| {
            // app.manage(SqliteMap::default());
            Ok(())
        })
        .build()
}
