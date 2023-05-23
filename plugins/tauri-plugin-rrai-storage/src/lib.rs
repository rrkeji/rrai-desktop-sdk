mod constants;
mod files;
mod handlers;
mod ipfs;
mod migration;
mod models;

use anyhow::anyhow;
use rrai_desktop_sdk_common::utils::read_bytes_from_pathbuf;
use tauri::{
    http::{Request as HttpRequest, Response as HttpResponse},
    plugin::{Builder, TauriPlugin},
    AppHandle, Runtime,
};
use url::Url;

pub fn rrfile_protocol<R: Runtime>(
    _app_handle: &AppHandle<R>,
    request: &HttpRequest,
) -> std::result::Result<HttpResponse, Box<dyn std::error::Error>> {
    let url: Url = request.uri().parse()?;

    //host localhost/ipfs
    let host = url
        .host()
        .map_or(Err(anyhow!("host没有找到")), |host| Ok(host.to_string()))?;

    let mut file_name_query = String::new();

    let _query_pairs = url
        .query_pairs()
        .into_iter()
        .map(|pair| {
            if pair.0 == "filename" {
                file_name_query = String::from(pair.1.clone());
            }
            (String::from(pair.0), String::from(pair.1))
        })
        .collect::<Vec<(String, String)>>();

    tracing::debug!("请求地址:{}-{}", url, url.path());

    let mut mime = mime::TEXT_PLAIN;

    let buf = if host == "localhost" {
        //local
        let url = url.path().to_string();
        let file_path = url.replace("rrfile://localhost", "");
        //获取文件大小，如果太大，提示
        tracing::debug!("文件路径:{:?}", file_path);

        mime = mime_guess::from_path(file_path.clone()).first_or(mime::TEXT_PLAIN);
        //获取文件内容
        let buf = read_bytes_from_pathbuf(&file_path.parse()?)?;
        buf
    } else {
        // rrfile://ipfs/QmdLuUuPRHpnQSV8iC4HVdpjjVF3Gsb3c3CVSR84fGXp7S?filename=3f298d37-9a69-459a-8f4e-ac7e0c9e9a3b.jpeg
        if file_name_query != "" {
            mime = mime_guess::from_path(file_name_query.clone()).first_or(mime::TEXT_PLAIN);
        }
        //ipfs
        tracing::debug!("请求地址:{}-{}", url, url.path());
        let cid = url.path().replace("/", "");

        let res =
            tauri::async_runtime::block_on(rrai_desktop_sdk_common::ipfs::ipfs_get_content(&cid))?;

        res
    };

    //获取mimetype
    tauri::http::ResponseBuilder::new()
        .header("Origin", "*")
        .mimetype(mime.as_ref())
        .header("Content-Length", buf.len())
        .status(200)
        .body(buf)
}

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
    });
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
