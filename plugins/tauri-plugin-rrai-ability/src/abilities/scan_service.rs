use anyhow::{anyhow, Result};
/// 自动扫描
pub async fn auto_scan() -> Result<()> {
    //依次调用查看能力

    //python
    crate::abilities::python::scan_and_insert().await?;
    //docker
    crate::abilities::docker::scan_and_insert().await?;
    //stable diffusion
    crate::abilities::stable_diffusion::scan_and_insert().await?;
    //stable diffusion webui
    crate::abilities::stable_diffusion_webui::scan_and_insert().await?;

    Ok(())
}
