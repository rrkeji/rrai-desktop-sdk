use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Mutex};
use tauri::{command, State};

use crate::models::AbilityEntity;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

/// 自动扫描
#[command]
pub async fn env_auto_scan() -> Result<bool> {
    let _ = crate::abilities::env_auto_scan().await?;
    Ok(true)
}

/// 执行任务
#[command]
pub async fn perform_task(ability: String, args: String) -> Result<String> {
    let res = crate::abilities::perform_task(&ability, &args).await?;
    Ok(res)
}

/// 任务的状态
#[command]
pub async fn perform_task_status(task_id: String) -> Result<HashMap<String, Value>> {
    let res = crate::abilities::perform_task_status(&task_id).await?;
    Ok(res)
}

#[command]
pub async fn list_abilities() -> Result<Vec<HashMap<String, Value>>> {
    let files = crate::abilities::list_abilities().await?;
    Ok(files)
}

#[command]
pub async fn insert_ability(ability: AbilityEntity) -> Result<usize> {
    let res = crate::abilities::insert_ability(ability).await?;
    Ok(res)
}

#[command]
pub async fn update_ability(
    id: u32,
    is_available: u32,
    ability: String,
    version: String,
    icon: String,
    dependencies: String,
    category: String,
    settings: String,
) -> Result<usize> {
    let res = crate::abilities::update_ability(
        id,
        is_available,
        ability,
        version,
        icon,
        dependencies,
        category,
        settings,
    )
    .await?;
    Ok(res)
}

#[command]
pub async fn update_ability_settings(ability: String, settings: String) -> Result<usize> {
    let res = crate::abilities::update_ability_settings(&ability, &settings).await?;
    Ok(res)
}

#[command]
pub async fn delete_ability(id: u32) -> Result<usize> {
    let res = crate::abilities::delete_ability(id).await?;
    Ok(res)
}

#[command]
pub async fn list_tasks(page: u32, page_size: u32) -> Result<(Value, Vec<HashMap<String, Value>>)> {
    let files = crate::tasks::list_tasks(page, page_size).await?;
    Ok(files)
}

///
#[command]
pub async fn query_task_by_task_id(task_id: String) -> Result<HashMap<String, Value>> {
    let res = crate::tasks::query_by_task_id(&task_id).await?;
    Ok(res)
}

///
#[command]
pub async fn query_task_status(task_id: String) -> Result<HashMap<String, Value>> {
    let res = crate::tasks::query_task_status(&task_id).await?;
    Ok(res)
}

#[command]
pub async fn delete_task(task_id: String) -> Result<usize> {
    let res = crate::tasks::delete_task(&task_id).await?;
    Ok(res)
}

/// 新建终端
#[command]
pub async fn new_terminal(command: String) -> Result<String> {
    let res = crate::terminal::async_execute_command(&command).await?;
    Ok(res)
}

/// 终端执行命令
#[command]
pub async fn terminal_execute_command(terminal_id: String, command: String) -> Result<String> {
    let res = crate::terminal::async_execute_command(&command).await?;
    Ok(res)
}

/// 终端的输出
#[command]
pub async fn terminal_stdout(terminal_id: String) -> Result<Vec<String>> {
    let res = crate::terminal::terminal_stdout(&terminal_id).await?;
    Ok(res)
}

/// 终端的错误输出
#[command]
pub async fn terminal_stderr(terminal_id: String) -> Result<Vec<String>> {
    let res = crate::terminal::terminal_stderr(&terminal_id).await?;
    Ok(res)
}

/// 终端当前状态
#[command]
pub async fn terminal_status(terminal_id: String) -> Result<Vec<String>> {
    let res = crate::terminal::terminal_stderr(&terminal_id).await?;
    Ok(res)
}
/// 终端当前命令中断
#[command]
pub async fn terminal_interrupt_command(terminal_id: String) -> Result<Vec<String>> {
    let res = crate::terminal::terminal_stderr(&terminal_id).await?;
    Ok(res)
}

/// 关闭终端
#[command]
pub async fn close_terminal(terminal_id: String) -> Result<Vec<String>> {
    let res = crate::terminal::terminal_stderr(&terminal_id).await?;
    Ok(res)
}
