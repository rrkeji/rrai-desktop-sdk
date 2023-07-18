use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
    //
    if let (Some(code), message) = execute_command(&String::from("ffmpeg version")).await? {
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
        &String::from("Docker"),
        if available { 1 } else { 0 },
        &String::from(""),
        &version,
    )
    .await?;

    Ok(())
}
