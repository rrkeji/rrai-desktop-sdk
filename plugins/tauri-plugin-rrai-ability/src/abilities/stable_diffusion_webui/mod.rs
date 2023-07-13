mod env;
mod perform;

pub use env::*;
pub use perform::*;

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use hyper::{client::Client, Body, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;

/// GET TOKEN
fn get_basic_authorization() -> Result<String> {
    //获取到用户名和密码
    let username = "rrkeji";
    let password = "rr9999";

    let encoded = general_purpose::STANDARD.encode(&format!("{}:{}", username, password));
    Ok(encoded)
}

pub async fn webui_get(path: &str) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("{}{}", get_webui_url(), path.clone(),);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("GET")
        .body(Body::from(""))
        .map_err(|err| {
            tracing::error!("调用{}接口封装request:{:?}", url, err);
            anyhow!("{}", err)
        })?;

    //
    let response = client
        .request(request)
        .await
        .map_err(|err| anyhow!("{}", err))?;

    //读入数据
    let bytes = hyper::body::to_bytes(response.into_body())
        .await
        .map_err(|err| anyhow!("{}", err))?;

    Ok(String::from_utf8(bytes.to_vec())?)
}

pub async fn webui_post(path: &str, request_obj: String) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("{}{}", get_webui_url(), path.clone(),);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("POST")
        .header(
            "Authorization",
            format!("Basic {}", get_basic_authorization()?),
        )
        .header("Content-Type", "application/json")
        .body(Body::from(request_obj))
        .map_err(|err| {
            tracing::error!("调用{}接口封装request:{:?}", url, err);
            anyhow!("{}", err)
        })?;

    //
    let response = client
        .request(request)
        .await
        .map_err(|err| anyhow!("{}", err))?;

    //读入数据
    let bytes = hyper::body::to_bytes(response.into_body())
        .await
        .map_err(|err| anyhow!("{}", err))?;

    Ok(String::from_utf8(bytes.to_vec())?)
}
