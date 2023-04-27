use crate::models::FileEntity;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri::command;

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

#[command]
pub async fn list_files(parent_id: u32) -> Result<Vec<HashMap<String, Value>>> {
    let files = crate::files::list_files_by_category(parent_id, &String::new()).await?;
    Ok(files)
}

#[command]
pub async fn list_files_by_category(
    parent_id: u32,
    category: String,
) -> Result<Vec<HashMap<String, Value>>> {
    let files = crate::files::list_files_by_category(parent_id, &category).await?;
    Ok(files)
}

#[command]
pub async fn insert_file(file: FileEntity) -> Result<usize> {
    let res = crate::files::insert_file(file).await?;
    Ok(res)
}

#[command]
pub async fn update_file(
    id: u32,
    parent_id: u32,
    file_name: String,
    category: String,
    avatar: String,
) -> Result<usize> {
    let res = crate::files::update_file(id, parent_id, file_name, category, avatar).await?;
    Ok(res)
}

#[command]
pub async fn delete_file(id: u32) -> Result<usize> {
    let res = crate::files::delete_file(id).await?;
    Ok(res)
}

#[command]
pub async fn create_dir(parent_id: u32, file_name: String) -> Result<usize> {
    let res = crate::files::insert_file(FileEntity {
        id: 0,
        parent_id: parent_id,
        cid: String::new(),
        is_pin: false,
        file_name: file_name,
        file_hash: String::new(),
        file_type: String::new(),
        category: String::new(),
        avatar: String::new(),
        is_dir: true,
        created_at: 0,
        updated_at: 0,
    })
    .await?;
    Ok(res)
}

#[command]
pub async fn ipfs_add_content(data: Vec<u8>) -> Result<String> {
    //添加内容
    let res = rrai_desktop_sdk_common::ipfs::ipfs_add_content(data).await?;
    Ok(res)
}

#[command]
pub async fn ipfs_get_content(cid: String) -> Result<Vec<u8>> {
    //添加内容
    let res = rrai_desktop_sdk_common::ipfs::ipfs_get_content(&cid).await?;
    Ok(res)
}
