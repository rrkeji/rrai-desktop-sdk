use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

use crate::models::AbilityEntity;


//----------------------------------------------
const TABLE_NAME: &str = "rrai_abilities";

const ALL_FIELDS: &str= " id, is_available, ability, version, icon, dependencies, category, settings, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_abilities() -> Result<Vec<HashMap<String, Value>>> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("SELECT {} FROM {} order by id", ALL_FIELDS, TABLE_NAME),
        &json!({}),
    )
    .await?;

    Ok(abilities)
}

pub async fn insert_ability(ability: AbilityEntity) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("INSERT INTO {} (is_available, ability, version, icon, dependencies, category, settings) VALUES(:is_available, :ability, :version, :icon, :dependencies, :category, :settings)", TABLE_NAME),
        &json!({
            ":is_available": ability.is_available,
            ":ability": ability.ability.clone(),
            ":version":   ability.version.clone(),
            ":icon": ability.icon.clone(),
            ":dependencies": ability.dependencies.clone(),
            ":category": ability.category.clone(),
            ":settings": ability.settings.clone(),
        }),
    )
    .await?;
    Ok(res)
}

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
    let res  =  execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("UPDATE {} SET is_available = :is_available, ability = :ability, version = :version, icon = :icon, dependencies = :dependencies, category = :category, settings = :settings where id = :id ", TABLE_NAME),
        &json!({
            ":is_available": is_available,
            ":ability": ability,
            ":version": version,
            ":icon": icon,
            ":dependencies": dependencies,
            ":category": category,
            ":settings":settings,
            ":id":id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn delete_ability(id: u32) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("DELETE FROM {} where id = :id ", TABLE_NAME),
        &json!({
            ":id":id,
        }),
    )
    .await?;
    Ok(res)
}
