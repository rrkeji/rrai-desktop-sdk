use anyhow::Result;
use serde_json::json;

/// 发布任务
pub async fn tasks_task_publish(
    token: &String,
    name: &String,
    task_type: &String,
    model: &String,
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
    abilities: &Vec<String>,
) -> Result<String> {
    let url = format!("/tasks/task/take");
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    let request_obj = json!({
        "peer_id" : peer_id,
        "env" : env,
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
    })
    .to_string();
    tracing::debug!("request:{:?}", request_obj);

    let res = crate::request::rrai_cloud_post(&url, token, request_obj).await?;
    Ok(res)
}
