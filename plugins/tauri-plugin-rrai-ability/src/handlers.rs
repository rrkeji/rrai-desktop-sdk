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

/// 自动扫描
#[command]
pub async fn auto_scan() -> Result<bool> {
    let _ = crate::abilities::auto_scan().await?;
    Ok(true)
}

/// 能力扫描
#[command]
pub async fn ability_scan(ability: String) -> Result<bool> {
    let _ = crate::abilities::ability_scan(&ability).await?;
    Ok(true)
}

/// 执行任务
#[command]
pub async fn perform_task(ability: String, args: String) -> Result<bool> {
    let _ = crate::abilities::perform_task(&ability).await?;
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
pub async fn update_ability_settings(ability: String, settings: String) -> Result<usize> {
    let res = crate::abilities::update_ability_settings(&ability, &settings).await?;
    Ok(res)
}

#[command]
pub async fn delete_ability(id: u32) -> Result<usize> {
    let res = crate::abilities::delete_ability(id).await?;
    Ok(res)
}
