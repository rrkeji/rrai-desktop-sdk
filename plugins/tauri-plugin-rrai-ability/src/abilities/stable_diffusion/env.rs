use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn is_available() -> Result<(bool, String)> {
    //读取配置
    let ability = crate::abilities::abilities_service::query_by_ability(&String::from(
        crate::constants::STABLE_DIFFUSION_ABILITY_NAME,
    ))
    .await?;

    //
    if let Some(Value::String(settings)) = ability.get(&String::from("settings")) {
        //json 解析
        if let Value::Object(settings_values) = serde_json::from_str(settings.as_str())? {
            //获取到
            if let Value::String(path_str) = settings_values
                .get(&String::from("model_path"))
                .map_or(Err(anyhow!("")), |path| Ok(path))?
            {
                // path_str
                tracing::debug!("model_path:{}", path_str)
            }
            //
            // if let (Some(code), message) = execute_command(&String::from("docker version")).await? {
            //     if code == 0 {
            //         Ok((true, message))
            //     } else {
            //         Ok((false, message))
            //     }
            // }
        }
    }
    Ok((true, String::new()))
}

pub async fn scan_and_insert() -> Result<()> {
    let (available, version) = is_available().await?;

    crate::abilities::abilities_service::insert_or_update_ability(
        &String::from("StableDiffusion"),
        if available { 1 } else { 0 },
        &String::from(""),
        &version,
        &String::from(""),
    )
    .await?;

    Ok(())
}
