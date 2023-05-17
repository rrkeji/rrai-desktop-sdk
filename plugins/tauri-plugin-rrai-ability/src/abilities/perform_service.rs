use crate::tasks::{running_command_status, running_command_stderr, running_command_stdout};
use anyhow::{anyhow, Result};
/// 自动扫描
pub async fn perform_task(ability: &String, args: &String) -> Result<String> {
    //依次调用查看能力
    if ability == crate::constants::STABLE_DIFFUSION_ABILITY_NAME {
        //Stable Diffusion
        let res = crate::abilities::stable_diffusion::perform_task(args).await?;

        return Ok(res);
    }
    Err(anyhow!("不支持的能力:{}", ability))
}

pub async fn perform_task_stdout(
    _ability: &String,
    running_task_id: &String,
) -> Result<Vec<String>> {
    running_command_stdout(running_task_id).await
}

pub async fn perform_task_stderr(
    _ability: &String,
    running_task_id: &String,
) -> Result<Vec<String>> {
    running_command_stderr(running_task_id).await
}

pub async fn perform_task_status(
    _ability: &String,
    running_task_id: &String,
    exit_remove: bool,
) -> Result<(bool, i32)> {
    running_command_status(running_task_id, exit_remove).await
}
