use anyhow::Result;
use serde_json::json;

pub async fn dataset_create_by_model_id(token: &String, model_id: &String) -> Result<String> {
    let url = format!("/dataset/datasets/create_by_model_id/{}", model_id);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}

pub async fn dataset_rows(
    token: &String,
    dataset_id: &String,
    parts: Option<String>,
    tags: Option<String>,
    page_size: u32,
    page: u32,
) -> Result<String> {
    let url = format!("/dataset/rows/search");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "page_size": page_size,
        "page": page,
        "conditions": {
            "dataset_id": dataset_id.clone(),
            "parts": parts,
            "tags": tags,
        }
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn dataset_rows_search_owned(
    token: &String,
    dataset_id: &String,
    parts: Option<String>,
    tags: Option<String>,
    page_size: u32,
    page: u32,
) -> Result<String> {
    let url = format!("/dataset/rows/search_owned");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "page_size": page_size,
        "page": page,
        "conditions": {
            "dataset_id": dataset_id.clone(),
            "parts": parts,
            "tags": tags,
        }
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn dataset_rows_search_by_model(
    token: &String,
    model_id: &String,
    parts: Option<String>,
    tags: Option<String>,
    page_size: u32,
    page: u32,
) -> Result<String> {
    let url = format!("/dataset/rows/search_by_model");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "page_size": page_size,
        "page": page,
        "conditions": {
            "model_id": model_id,
            "parts": parts,
            "tags": tags,
        }
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn query_dataset_row(token: &String, dataset_id: &String, id: u32) -> Result<String> {
    let url = format!("/dataset/rows/get/{}", id);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}

pub async fn dataset_create_row(
    token: &String,
    dataset_id: &String,
    row_cid: &String,
    parts: &String,
    tags: &String,
) -> Result<String> {
    let url = format!("/dataset/rows/create");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "dataset_id": dataset_id.clone(),
        "row_cid": row_cid.clone(),
        "parts":  parts.clone(),
        "tags":tags.clone(),
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn update_dataset_row(
    token: &String,
    id: u32,
    row_cid: &String,
    parts: &String,
    tags: &String,
) -> Result<String> {
    let url = format!("/dataset/rows/update/{}", id);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "row_cid": row_cid.clone(),
        "parts":  parts.clone(),
        "tags":tags.clone(),
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn remove_dataset_row(token: &String, id: u32) -> Result<String> {
    let url = format!("/dataset/rows/remove/{}", id);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}
