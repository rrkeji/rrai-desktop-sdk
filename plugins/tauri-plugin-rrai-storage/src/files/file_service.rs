use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

use crate::models::FileEntity;

const TABLE_NAME: &str = "rrai_files";

const ALL_FIELDS: &str= " id, parent_id, cid, is_pin, file_name, file_hash, file_type, category, avatar, is_dir, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_files_by_category(
    parent_id: u32,
    category: &String,
) -> Result<Vec<HashMap<String, Value>>> {
    let files = if category == "" {
        query_with_args(
            &crate::constants::STORAGE_DATABASE_NAME.to_string(),
            &format!(
                "SELECT {} FROM {} WHERE parent_id = :parent_id",
                ALL_FIELDS, TABLE_NAME
            ),
            &json!({
                ":parent_id": parent_id,
            }),
        )
        .await?
    } else {
        //
        query_with_args(
            &crate::constants::STORAGE_DATABASE_NAME.to_string(),
            &format!(
                "SELECT {} FROM {} WHERE parent_id = :parent_id and category = :category",
                ALL_FIELDS, TABLE_NAME
            ),
            &json!({
                ":parent_id": parent_id,
                ":category": category.clone()
            }),
        )
        .await?
    };

    Ok(files)
}

pub async fn insert_file(file: FileEntity) -> Result<usize> {
    let res = execute(
        &crate::constants::STORAGE_DATABASE_NAME.to_string(),
        &format!("INSERT INTO {} (parent_id, cid, is_pin, file_name, file_hash, file_type, category, avatar, is_dir) VALUES(:parent_id, :cid, :is_pin, :file_name, :file_hash, :file_type, :category, :avatar, :is_dir)", TABLE_NAME),
        &json!({
            ":parent_id": file.parent_id,
            ":cid": file.cid.clone(),
            ":is_pin": if file.is_pin {1}else{0},
            ":file_name": file.file_name.clone(),
            ":file_hash": file.file_hash.clone(),
            ":file_type": file.file_type.clone(),
            ":category": file.category.clone(),
            ":avatar":file.avatar.clone(),
            ":is_dir": if file.is_dir {1}else{0},
        }),
    )
    .await?;
    Ok(res)
}

pub async fn update_file(
    id: u32,
    parent_id: u32,
    file_name: String,
    category: String,
    avatar: String,
) -> Result<usize> {
    let res  =  execute(
        &crate::constants::STORAGE_DATABASE_NAME.to_string(),
        &format!("UPDATE {} SET parent_id = :parent_id, file_name = :file_name, category = :category, avatar = :avatar where id = :id ", TABLE_NAME),
        &json!({
            ":parent_id": parent_id,
            ":file_name": file_name,
            ":category": category,
            ":avatar":avatar,
            ":id":id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn delete_file(id: u32) -> Result<usize> {
    let res = execute(
        &crate::constants::STORAGE_DATABASE_NAME.to_string(),
        &format!("DELETE FROM {} where id = :id ", TABLE_NAME),
        &json!({
            ":id":id,
        }),
    )
    .await?;
    Ok(res)
}
