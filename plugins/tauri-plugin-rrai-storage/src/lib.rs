mod constants;
mod files;
mod handlers;
mod ipfs;
mod migration;
mod models;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    //异步非阻塞
    tauri::async_runtime::spawn(async move {
        //启动 ipfs
        let (mut rx, mut child) = tauri::api::process::Command::new_sidecar("ipfs")
            .expect("failed to create `ipfs` binary command")
            .args(["daemon"])
            .spawn()
            .expect("Failed to spawn sidecar");

        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let tauri::api::process::CommandEvent::Stdout(line) = event {
                tracing::debug!("IPFS:{}", line);
            }
        }
    })
    .expect("执行数据库脚本失败！");
    //
    tauri::async_runtime::block_on(async move {
        //判断是否启动 ipfs
        //判断是否存在配置文件

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
            handlers::ipfs_add_content,
            handlers::ipfs_get_content,
        ])
        .build()
}
