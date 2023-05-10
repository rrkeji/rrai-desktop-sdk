use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
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

    crate::abilities::abilities_service::insert_ability(AbilityEntity {
        id: 0,
        is_available: if available { 1 } else { 0 },
        ability: String::from("StableDiffusion"),
        version: String::from("1.5"),
        icon: String::from(
            "https://www.docker.com/wp-content/uploads/2023/04/cropped-Docker-favicon-32x32.png",
        ),
        dependencies: String::from(""),
        category: String::from(""),
        settings: String::from("{}"),
        created_at: 0,
        updated_at: 0,
    })
    .await?;

    Ok(())
}
