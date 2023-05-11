use std::{collections::HashMap, f32::consts::E};

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

use crate::models::AbilityEntity;

//----------------------------------------------
const TABLE_NAME: &str = "rrai_abilities";

const ALL_FIELDS: &str= " id, is_available, ability, ability_name, version, version_infor, icon, dependencies, category, settings_schema, install_guide, settings, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_abilities() -> Result<Vec<HashMap<String, Value>>> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("SELECT {} FROM {} order by id", ALL_FIELDS, TABLE_NAME),
        &json!({}),
    )
    .await?;

    Ok(abilities)
}

pub async fn query_by_ability(ability: &String) -> Result<HashMap<String, Value>> {
    let mut abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE `ability` = :ability order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({
            ":ability" : ability.clone()
        }),
    )
    .await?;

    if let Some(item) = abilities.pop() {
        Ok(item)
    } else {
        Err(anyhow!("没有找到"))
    }
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

pub async fn insert_or_update_ability(
    ability: &String,
    is_available: u32,
    version: &String,
    version_infor: &String,
    settings: &String,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("UPDATE {} SET is_available = :is_available, version=:version, version_infor=:version_infor, settings=:settings WHERE ability = :ability", TABLE_NAME),
        &json!({
            ":is_available": is_available,
            ":ability":  ability.clone(),
            ":version":    version.clone(),
            ":version_infor": version_infor.clone(),
            ":settings":  settings.clone(),
        }),
    )
    .await?;
    Ok(res)
}

pub async fn update_ability_settings(ability: &String, settings: &String) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "UPDATE {} SET settings = :settings WHERE ability = :ability",
            TABLE_NAME
        ),
        &json!({
            ":ability":  ability.clone(),
            ":settings":  settings.clone(),
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
