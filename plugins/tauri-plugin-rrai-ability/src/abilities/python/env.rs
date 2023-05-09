use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
    //
    if let (Some(code), message) = execute_command(&String::from("python3 --version")).await? {
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
        ability: String::from("Python"),
        version: version,
        icon: String::from("https://docs.python.org/3/_static/py.svg"),
        dependencies: String::from(""),
        category: String::from(""),
        settings: String::from("{}"),
        created_at: 0,
        updated_at: 0,
    })
    .await?;

    Ok(())
}
