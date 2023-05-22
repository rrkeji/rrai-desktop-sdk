use crate::tasks::{insert_local_task, query_task_status};
use anyhow::{anyhow, Result};
use serde_json::Value;
use std::collections::HashMap;

/// 自动扫描
pub async fn perform_task(ability: &String, args: &String) -> Result<String> {
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

    //插入一条任务信息
    insert_local_task(&uuid, ability, args).await?;

    //依次调用查看能力
    if ability == crate::constants::STABLE_DIFFUSION_ABILITY_NAME {
        //Stable Diffusion
        let res = crate::abilities::stable_diffusion::perform_task(&uuid, args).await?;

        return Ok(res);
    }
    Err(anyhow!("不支持的能力:{}", ability))
}

pub async fn perform_task_status(task_id: &String) -> Result<HashMap<String, Value>> {
    query_task_status(task_id).await
}
