use anyhow::{anyhow, Result};
use hyper::{client::Client, Body, Request};
use hyper_tls::HttpsConnector;

/// 该接口不用token
pub async fn model_schema(model_id: &String, version: u32) -> Result<String> {
    tracing::debug!("请求schema:{}[{}]", model_id, version);

    //请求上传文件
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let request = Request::builder()
        .uri(
            format!(
                "{}schemas/{}/{}",
                crate::constants::RRAI_CLOUD_URI,
                model_id,
                version,
            )
            .as_str(),
        )
        .method("GET")
        .body(Body::from(""))
        .map_err(|err| {
            tracing::error!("调用schema接口封装请求失败:{:?}", err);
            anyhow!(err)
        })?;

    //
    let response = client.request(request).await?;

    //读入数据
    let bytes = hyper::body::to_bytes(response.into_body())
        .await
        .map_err(|err| anyhow!(err))?;

    Ok(String::from_utf8(bytes.to_vec()).map_err(|err| {
        tracing::error!("转换String失败:{:?}", bytes);
        anyhow!(err)
    })?)
}
