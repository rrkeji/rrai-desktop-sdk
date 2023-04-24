mod app_downloader;
mod app_server;
mod models;
mod proxy;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use app_downloader::download;

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::async_runtime::block_on(async move {
        //启动HTTP文件服务
        tracing::debug!("启动HTTP文件服务");
        app_server::Server::new()
            .start()
            .expect("启动HTTP文件服务失败");
    });

    //
    Builder::new("rrai-appbox")
        .invoke_handler(tauri::generate_handler![download,])
        .setup(|app| Ok(()))
        .build()
}

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;
