use anyhow::{anyhow, Result};
use hyper::{client::Client, Body, Request};
use hyper_tls::HttpsConnector;

pub async fn rrai_cloud_get(path: &String, token: &String) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("{}{}", crate::constants::RRAI_CLOUD_URL, path.clone(),);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("GET")
        .header("Authorization", token.clone())
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

pub async fn rrai_cloud_post(path: &String, token: &String, request_obj: String) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("{}{}", crate::constants::RRAI_CLOUD_URL, path.clone(),);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("POST")
        .header("Authorization", token.clone())
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
