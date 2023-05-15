use anyhow::{anyhow, Result};

/// 自动扫描
pub async fn perform_task(ability: &String) -> Result<()> {
    //依次调用查看能力
    if ability == crate::constants::STABLE_DIFFUSION_ABILITY_NAME {
        //Stable Diffusion
        crate::abilities::stable_diffusion::perform_task().await?;
    }

    Ok(())
}
