use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

//----------------------------------------------
const TABLE_NAME: &str = "rrai_local_tasks";

const ALL_FIELDS: &str= " id, task_id, ability, args, remote, remote_task_id, remote_server, result_code, stdout, stderr, result, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_tasks() -> Result<Vec<HashMap<String, Value>>> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({}),
    )
    .await?;

    Ok(abilities)
}

pub async fn list_local_tasks(is_local: bool) -> Result<Vec<HashMap<String, Value>>> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE remote = :remote order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({
            ":remote" : if is_local {0}else{1}
        }),
    )
    .await?;

    Ok(abilities)
}

pub async fn query_by_task_id(task_id: &String) -> Result<HashMap<String, Value>> {
    let mut abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE `task_id` = :task_id order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({
            ":task_id" : task_id.clone()
        }),
    )
    .await?;

    if let Some(item) = abilities.pop() {
        Ok(item)
    } else {
        Err(anyhow!("没有找到"))
    }
}

pub async fn insert_local_task(task_id: &String, ability: &String, args: &String) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "INSERT INTO {} (task_id, ability, args) VALUES(:task_id, :ability, :args)",
            TABLE_NAME
        ),
        &json!({
            ":task_id": task_id,
            ":ability": ability,
            ":args":   args,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn insert_remote_task(
    task_id: &String,
    ability: &String,
    args: &String,
    remote_task_id: &String,
    remote_server: &String,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("INSERT INTO {} (task_id, ability, args, remote, remote_task_id, remote_server) VALUES(:task_id, :ability, :args, :remote, :remote_task_id, :remote_server)", TABLE_NAME),
        &json!({
            ":task_id": task_id,
            ":ability": ability,
            ":args":   args,
            ":remote": 1_u16,
            ":remote_task_id": remote_task_id,
            ":remote_server": remote_server,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn append_task_stdout(
    task_id: &String,
    stdout: &String,
    stderr: &String,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "UPDATE {} SET stdout =  stdout || :stdout,stderr = stderr || :stderr WHERE task_id = :task_id",
            TABLE_NAME
        ),
        &json!({
            ":stdout":  stdout,
            ":stderr":  stderr,
            ":task_id":  task_id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn update_task_status(
    task_id: &String,
    result_code: u16,
    result: &String,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "UPDATE {} SET result_code = :result_code, result = :result WHERE task_id = :task_id",
            TABLE_NAME
        ),
        &json!({
            ":result_code":  result_code,
            ":result":  result,
            ":task_id":  task_id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn delete_task(task_id: &String) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!("DELETE FROM {} where task_id = :task_id ", TABLE_NAME),
        &json!({
            ":task_id":task_id,
        }),
    )
    .await?;
    Ok(res)
}

pub async fn query_task_status(task_id: &String) -> Result<HashMap<String, Value>> {
    let mut abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT result_code, stdout, stderr, result FROM {} WHERE `task_id` = :task_id order by id",
            TABLE_NAME
        ),
        &json!({
            ":task_id" : task_id.clone()
        }),
    )
    .await?;

    if let Some(item) = abilities.pop() {
        Ok(item)
    } else {
        Err(anyhow!("没有找到"))
    }
}
