use crate::ipfs::create_with_bytes_content;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex, RwLock};
use tauri_plugin_rrai_ability::abilities::perform_task_and_block;

/// 发布任务
pub async fn tasks_task_publish(
    token: &String,
    name: &String,
    task_type: &String,
    model: &String,
    action: &String,
    model_args: &String,
    description: &String,
    assign_strategy: &String,
    reward: u32,
) -> Result<String> {
    let url = format!("/tasks/task/publish");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "name" : name,
        "task_type" : task_type,
        "model" : model,
        "action" : action,
        "model_args" : model_args,
        "description" : description,
        "reward" :reward,
        "assign_strategy": assign_strategy
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

/// 领取任务
pub async fn tasks_task_take(
    token: &String,
    peer_id: &String,
    env: &String,
    task_type: &String,
    abilities: &String,
) -> Result<String> {
    let url = format!("/tasks/task/take");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "peer_id" : peer_id,
        "env" : env,
        "task_type" : task_type,
        "abilities" : abilities,
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

/// 保存任务结果和进度
pub async fn tasks_task_process_result(
    token: &String,
    task_id: u32,
    process_id: u32,
    progress: u16,
    result_code: u16,
    result: &String,
    logs: &String,
) -> Result<String> {
    let url = format!("/tasks/task/process/result");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "task_id" : task_id,
        "process_id" : process_id,
        "progress" : progress,
        "result_code" : result_code,
        "result" : result,
        "logs" : logs,
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}

/// 查询任务
pub async fn tasks_task_query_by_id(token: &String, task_id: u32) -> Result<String> {
    let url = format!("/tasks/task/get/{}", task_id);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);
    let res = crate::request::rrai_cloud_get(&url, token).await?;
    Ok(res)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AiTaskProcessEntity {
    pub id: u32,
    pub task_id: u32,
    pub task_type: String,
    pub model: String,
    pub action: String,
    pub model_args: String,
    pub reward: u32,
    pub deadline: Option<u32>,
    pub acceptor: String,
    pub acceptor_type: String,
    pub acceptor_args: String,
    pub process_status: u16,
    pub progress: u16,
    pub result_code: u16,
    pub result: String,
    pub created_at: u32,
    pub updated_at: u32,
    pub status: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AiTaskProcessEntityResponse {
    pub data: AiTaskProcessEntity,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpfsCidResult {
    pub data: String,
}

lazy_static::lazy_static! {
    static ref TASK_LOCK: RwLock<Option<bool>> = RwLock::new(None);
}

//
pub async fn tasks_worker_wakeup(token: &String) -> Result<()> {
    //
    let running = TASK_LOCK
        .read()
        .map_err(|_err| anyhow!("获取锁失败"))?
        .is_some();

    if running {
        tracing::debug!("正在执行任务....");
        return Ok(());
    }
    //设置锁状态
    {
        let mut cache = TASK_LOCK.write().map_err(|_err| anyhow!("获取锁失败"))?;
        *cache = Some(true);
    }

    // let action = String::from("TXT2IMG");
    let accepts = vec![
        ("AI_STABLE_DIFFUSION", "AI_STABLE_DIFFUSION_WEBUI"),
        ("PYTHON", "PYTHON"),
        ("FFMPEG", "FFMPEG"),
    ];

    for accept in accepts {
        //
        let _res =
            _tasks_worker_wakeup(token, &String::from(accept.0), &String::from(accept.1)).await;
    }

    //设置锁状态
    {
        let mut cache = TASK_LOCK.write().map_err(|_err| anyhow!("获取锁失败"))?;
        *cache = None;
    }
    Ok(())
}

//
pub async fn _tasks_worker_wakeup(
    token: &String,
    task_type: &String,
    abilities: &String,
) -> Result<()> {
    let peer_id = String::from("");
    let env = String::from("");

    if let Ok(process_str) = tasks_task_take(token, &peer_id, &env, task_type, abilities).await {
        tracing::debug!("{}", process_str);
        //
        let process_data: AiTaskProcessEntityResponse =
            serde_json::from_str(process_str.as_str()).map_err(|err| anyhow!("{}", err))?;

        let process = process_data.data;
        let action = &process.action;
        let args = &process.model_args;
        let task_id = process.task_id;

        //调用本地能力，进行执行
        let result = perform_task_and_block(
            &task_type,
            &abilities,
            action,
            args,
            task_id,
            process.id,
            |content| async move {
                let paths = vec![
                    String::from(".rrai"),
                    String::from(".worker"),
                    format!("{}", task_id),
                ];
                let file_type = String::from("image/png");
                let file_name = String::from("image.png");
                let category = String::from("AI_STABLE_DIFFUSION_WEBUI");
                tracing::debug!("传递的保存的内容长度:{}", content.len());
                //保存文件
                let ipfs_res = create_with_bytes_content(
                    token, &paths, &content, &file_type, &file_name, &category,
                )
                .await?;
                //ipfs_res :{data:xxx}
                let cid_data: IpfsCidResult =
                    serde_json::from_str(ipfs_res.as_str()).map_err(|err| anyhow!("{}", err))?;
                return Ok(cid_data.data);
            },
        )
        .await;

        // match result {
        //     Ok(result_string) => {
        //         tracing::debug!("{}", result_string);
        //         //发送成功
        //         let _update_cnt = tasks_task_process_result(
        //             token,
        //             task_id,
        //             process.id,
        //             100,
        //             0,
        //             &result_string,
        //             &String::new(),
        //         )
        //         .await?;
        //     }
        //     Err(err) => {
        //         tracing::error!("{}", err);
        //         //执行失败，
        //         let _update_cnt = tasks_task_process_result(
        //             token,
        //             task_id,
        //             process.id,
        //             0,
        //             3,
        //             &String::from(""),
        //             &String::new(),
        //         )
        //         .await?;
        //     }
        // }
    } else {
        tracing::error!("获取任务失败:{}-{}!", task_type, abilities);
    }
    Ok(())
}
