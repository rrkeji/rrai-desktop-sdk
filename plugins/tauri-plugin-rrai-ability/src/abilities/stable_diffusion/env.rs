use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
    //读取配置
    let ability = crate::abilities::abilities_service::query_by_ability(&String::from(
        crate::constants::STABLE_DIFFUSION_ABILITY_NAME,
    ))
    .await?;

    //
    let settings = ability.get(&String::from("settings"));

    //json 解析

    //获取到model_path
    
    //
    if let (Some(code), message) = execute_command(&String::from("docker version")).await? {
        if code == 0 {
            Ok((true, message))
        } else {
            Ok((false, message))
        }
    } else {
        Err(anyhow!(""))
    }
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
