use anyhow::Result;
use serde_json::json;

pub async fn ipfs_pins_status(token: &String, cid: &String) -> Result<String> {
    let url = format!("/ipfs/pins/status/{}", cid);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}

pub async fn ipfs_pins_unpin(token: &String, cid: &String) -> Result<String> {
    let url = format!("/ipfs/pins/unpin/{}", cid);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({}).to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn ipfs_pins_pin(token: &String, cid: &String) -> Result<String> {
    let url = format!("/ipfs/pins/pin/{}", cid);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({}).to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}
