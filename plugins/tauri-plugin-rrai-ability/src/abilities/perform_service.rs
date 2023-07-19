use crate::tasks::{insert_local_task, query_task_status, update_task_status};
use anyhow::{anyhow, Result};
use futures::future::Future;
use serde_json::Value;
use std::collections::HashMap;

/// 自动扫描
pub async fn perform_task(ability: &String, args: &String) -> Result<String> {
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

    //插入一条任务信息
    // insert_local_task(&uuid, ability, args).await?;

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

/// 执行任务到完成
pub async fn perform_task_and_block<F, R>(
    task_type: &String,
    ability: &String,
    action: &String,
    args: &String,
    request_task_id: u32,
    request_task_process_id: u32,
    explorer: F,
) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");
    //插入一条任务信息
    insert_local_task(
        &uuid,
        task_type,
        ability,
        args,
        request_task_id,
        request_task_process_id,
    )
    .await?;

    //依次调用查看能力
    let res = if ability == crate::constants::STABLE_DIFFUSION_WEBUI_ABILITY_NAME {
        //Stable Diffusion
        let res = crate::abilities::stable_diffusion_webui::perform_task_and_block(
            &uuid, action, args, explorer,
        )
        .await;
        res
    } else if ability == crate::constants::PYTHON_ABILITY_NAME {
        //PYTHON
        let res =
            crate::abilities::python::perform_task_and_block(&uuid, action, args, explorer).await;
        res
    } else {
        Err(anyhow!("不支持的能力:{}", ability))
    };

    match res {
        Ok(response) => {
            //任务完成
            update_task_status(&uuid, 0, &response).await?;
            return Ok(response);
        }
        Err(err) => {
            tracing::error!("{}", err);
            let error_message = format!("{}", err);
            update_task_status(&uuid, 3, &error_message).await?;

            return Err(anyhow!("执行失败:{}", ability));
        }
    }
}
