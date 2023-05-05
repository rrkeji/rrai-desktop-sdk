mod app_downloader;
mod app_server;
mod models;
mod proxy;

use tauri::{
    http::{Request as HttpRequest, Response as HttpResponse},
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};
use url::Url;

use app_downloader::download;

// Fn(&AppHandle<R>, &HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> + Send + Sync + 'static
pub fn rrapp_protocol<R: Runtime>(
    app_handle: &AppHandle<R>,
    request: &HttpRequest,
) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let url: Url = request.uri().parse()?;

    let app_id = url
        .host_str()
        .map_or(Err(anyhow::anyhow!("不合法的URL")), |host| Ok(host))?;

    tracing::debug!("{}=={}", url, app_id);
    tracing::debug!("req:{:?}", request);

    let mut buf = Vec::new();

    tauri::http::ResponseBuilder::new()
        .header("Origin", "*")
        .mimetype("image/svg+xml")
        .header("Content-Length", buf.len())
        .status(200)
        .body(buf)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
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
