use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};

use super::webui_get;

pub fn get_webui_url() -> String {
    //
    return String::from("http://100.64.166.104:17860");
}

pub async fn is_available() -> Result<(bool, String)> {
    //调用/internal/sysinfo接口
    if let Ok(res_string) = webui_get("/internal/sysinfo").await {
        tracing::debug!("调用接口返回:{}!", res_string);
        Ok((true, res_string))
    } else {
        tracing::debug!("调用接口失败!");
        Err(anyhow!(""))
    }
}

pub async fn scan_and_insert() -> Result<()> {
    let (available, info_string) = is_available().await?;

    crate::abilities::abilities_env_service::insert_or_update_ability_env(
        &String::from(crate::constants::STABLE_DIFFUSION_WEBUI_ABILITY_NAME),
        if available { 1 } else { 0 },
        &String::from(""),
        &info_string,
    )
    .await?;
    Ok(())
}
