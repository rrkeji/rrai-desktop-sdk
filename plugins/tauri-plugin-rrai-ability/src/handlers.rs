use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Mutex};
use tauri::{command, State};

use crate::models::AbilityEntity;

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

/// 下载应用
#[command]
pub async fn rrapp_download(application_cid: String) -> Result<bool> {
    Ok(true)
}

#[command]
pub async fn list_abilities() -> Result<Vec<HashMap<String, Value>>> {
    let files = crate::abilities::list_abilities().await?;
    Ok(files)
}

#[command]
pub async fn insert_ability(ability: AbilityEntity) -> Result<usize> {
    let res = crate::abilities::insert_ability(ability).await?;
    Ok(res)
}

#[command]
pub async fn update_ability(
    id: u32,
    is_available: u32,
    ability: String,
    version: String,
    icon: String,
    dependencies: String,
    category: String,
    settings: String,
) -> Result<usize> {
    let res = crate::abilities::update_ability(
        id,
        is_available,
        ability,
        version,
        icon,
        dependencies,
        category,
        settings,
    )
    .await?;
    Ok(res)
}

#[command]
pub async fn delete_ability(id: u32) -> Result<usize> {
    let res = crate::abilities::delete_ability(id).await?;
    Ok(res)
}
