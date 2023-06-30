use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

use crate::models::AbilityEnvEntity;

//----------------------------------------------
const TABLE_NAME: &str = "rrai_ability_env";

const ALL_FIELDS: &str= " id, is_available, env_code, env_name, version, version_infor, icon, category, settings_schema, install_guide, settings, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_ability_envs() -> Result<Vec<HashMap<String, Value>>> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("SELECT {} FROM {} order by id", ALL_FIELDS, TABLE_NAME),
        &json!({}),
    )
    .await?;

    Ok(abilities)
}

pub async fn query_by_ability_env(env_code: &String) -> Result<HashMap<String, Value>> {
    let mut abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE `env_code` = :env_code order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({
            ":env_code" : env_code.clone()
        }),
    )
    .await?;

    if let Some(item) = abilities.pop() {
        Ok(item)
    } else {
        Err(anyhow!("没有找到"))
    }
}
 
pub async fn insert_or_update_ability_env(
    env_code: &String,
    is_available: u32,
    version: &String,
    version_infor: &String,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("UPDATE {} SET is_available = :is_available, version=:version, version_infor=:version_infor WHERE env_code = :env_code", TABLE_NAME),
        &json!({
            ":is_available": is_available,
            ":env_code":  env_code.clone(),
            ":version":    version.clone(),
            ":version_infor": version_infor.clone(),
        }),
    )
    .await?;
    Ok(res)
}

pub async fn update_ability_env_settings(env_code: &String, settings: &String) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "UPDATE {} SET settings = :settings WHERE env_code = :env_code",
            TABLE_NAME
        ),
        &json!({
            ":env_code":  env_code.clone(),
            ":settings":  settings.clone(),
        }),
    )
    .await?;
    Ok(res)
}

pub async fn update_ability_env(
    id: u32,
    is_available: u32,
    env_code: String,
    version: String,
    icon: String,
    category: String,
    settings: String,
) -> Result<usize> {
    let res  =  execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("UPDATE {} SET is_available = :is_available, env_code = :env_code, version = :version, icon = :icon, category = :category, settings = :settings where id = :id ", TABLE_NAME),
        &json!({
            ":is_available": is_available,
            ":env_code": env_code,
            ":version": version,
            ":icon": icon,
            ":category": category,
            ":settings":settings,
            ":id":id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn delete_ability_env(id: u32) -> Result<usize> {
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

pub async fn query_ability_env_settings(ability: &String) -> Result<HashMap<String, Value>> {
    //查询该能力
    let ability = crate::abilities::abilities_service::query_by_ability(ability).await?;

    //
    if let Some(Value::String(settings)) = ability.get(&String::from("settings")) {
        //json 解析
        if let Value::Object(settings_values) = serde_json::from_str(settings.as_str())? {
            let mut res = HashMap::new();

            for (key, value) in settings_values {
                res.insert(key.clone(), value.clone());
            }
            return Ok(res);
        }
    }
    return Ok(HashMap::new());
}
