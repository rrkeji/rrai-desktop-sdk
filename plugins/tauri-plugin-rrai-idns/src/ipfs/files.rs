use anyhow::Result;
use serde_json::json;

pub async fn ipfs_files_search(token: &String, page_size: u32, page: u32) -> Result<String> {
    let url = format!("/ipfs/files/search");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "page_size": page_size,
        "page": page,
        "conditions": {
        }
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn ipfs_files_create(
    token: &String,
    parent_id: u64,
    cid: &String,
    is_pin: u16,
    is_dir: u16,
    file_name: &String,
    file_size: u32,
    file_type: &String,
    avatar: &String,
    category: &String,
) -> Result<String> {
    let url = format!("/ipfs/files/search");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "parent_id": parent_id,
        "cid": cid.clone(),
        "is_pin":  is_pin,
        "is_dir":  is_dir,
        "file_name":  file_name.clone(),
        "file_size":  file_size,
        "file_type":  file_type.clone(),
        "avatar":  avatar.clone(),
        "category":  category.clone(),
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn ipfs_files_update(
    token: &String,
    id: u32,
    file_name: &String,
    avatar: &String,
    category: &String,
) -> Result<String> {
    let url = format!("/ipfs/files/update/{}", id);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "file_name": file_name.clone(),
        "avatar":  avatar.clone(),
        "category":  category.clone()
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn create_with_string_content(
    token: &String,
    paths: &Vec<String>,
    content: &String,
    file_type: &String,
    file_name: &String,
    category: &String,
) -> Result<String> {
    let url = format!("/ipfs/files/create_with_string_content");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "path": paths,
        "content": content,
        "file_type": file_type,
        "file_name": file_name.clone(),
        "category":  category.clone()
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn mkdirs(token: &String, paths: &Vec<String>) -> Result<String> {
    let url = format!("/ipfs/files/mkdirs");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!(paths).to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

pub async fn ipfs_files_remove(token: &String, id: u32) -> Result<String> {
    let url = format!("/ipfs/files/remove/{}", id);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}