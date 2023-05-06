mod constants;
mod handlers;
mod rrapp;

use rrai_desktop_sdk_common::utils::read_bytes_from_pathbuf;
use tauri::{
    http::{Request as HttpRequest, Response as HttpResponse},
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};
use url::Url;

pub fn rrapp_protocol<R: Runtime>(
    _app_handle: &AppHandle<R>,
    request: &HttpRequest,
) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let url: Url = request.uri().parse()?;

    let app_cid = url
        .host_str()
        .map_or(Err(anyhow::anyhow!("不合法的URL")), |host| Ok(host))?;

    //app_cid 即为内容ID，为下载应用的路径中的一部分
    tracing::debug!("请求地址:{}", url);

    let url = url.path().to_string();
    let mut file_path = if url.contains("?") {
        let (first, _last) = url.split_at(url.find("?").unwrap());
        String::from(&first[1..])
    } else {
        String::from(&url[1..])
    };
    if file_path == ""{
        file_path = String::from("index.html");
    }
    //组装文件路径
    let file_path = crate::rrapp::get_rrapp_file_path(&String::from(app_cid), &file_path)
        .map_err(|err| anyhow::anyhow!("获取文件路径失败:{}", err))?;

    //获取文件大小，如果太大，提示
    //app_cid 即为内容ID，为下载应用的路径中的一部分
    tracing::debug!("文件路径:{:?}", file_path);

    //获取文件内容
    let buf = read_bytes_from_pathbuf(&file_path)?;

    //获取mimetype
    tauri::http::ResponseBuilder::new()
        .header("Origin", "*")
        .mimetype(
            mime_guess::from_path(file_path)
                .first_or(mime::TEXT_PLAIN)
                .as_ref(),
        )
        .header("Content-Length", buf.len())
        .status(200)
        .body(buf)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    //
    Builder::new("rrai-appbox")
        .invoke_handler(tauri::generate_handler![
            handlers::rrapp_download,
            handlers::rrapp_check,
        ])
        .setup(|app| Ok(()))
        .build()
}
