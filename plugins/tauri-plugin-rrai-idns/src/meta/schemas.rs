use anyhow::Result;

pub async fn schema_by_model(token: &String, model_id: &String, version: u32) -> Result<String> {
    let url = format!("/schemas/{}/{}", model_id, version);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}
