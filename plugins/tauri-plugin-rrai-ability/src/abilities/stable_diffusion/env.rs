use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
    //读取配置
    if let Ok(res) = crate::abilities::stable_diffusion::perform_test().await {
        tracing::debug!("stable_diffusion测试成功");
        Ok((true, res))
    } else {
        Ok((false, String::new()))
    }
}

pub async fn scan_and_insert() -> Result<()> {
    let (available, version) = is_available().await?;

    tracing::debug!("进行更新能力表:{},{}", available, version);

    crate::abilities::abilities_service::insert_or_update_ability(
        &String::from(crate::constants::STABLE_DIFFUSION_ABILITY_NAME),
        if available { 1 } else { 0 },
        &String::from(""),
        &version,
    )
    .await?;

    Ok(())
}
