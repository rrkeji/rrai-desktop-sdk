use std::collections::HashMap;

use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::sqlite::{execute, execute_batch, query_with_args};
use serde_json::{json, Value};

//----------------------------------------------
const TABLE_NAME: &str = "rrai_local_tasks";

const ALL_FIELDS: &str= " id, task_id, task_type, ability, args, request_task_id, request_task_process_id, request_server, result_code, stdout, stderr, result, STRFTIME(created_at), STRFTIME(updated_at) ";

pub async fn list_tasks(page: u32, page_size: u32) -> Result<(Value, Vec<HashMap<String, Value>>)> {
    let cnt = {
        let mut count_map = query_with_args(
            &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
            &format!("SELECT count(1) cnt FROM {} order by id", TABLE_NAME),
            &json!({}),
        )
        .await?;

        if let Some(item) = count_map.pop() {
            Ok(item.get("cnt").map_or(json!(0), |i| i.clone()))
        } else {
            Err(anyhow!("没有找到"))
        }
    }?;

    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} order by id limit {}, {}",
            ALL_FIELDS,
            TABLE_NAME,
            (page - 1) * page_size,
            page_size
        ),
        &json!({}),
    )
    .await?;

    Ok((cnt, abilities))
}

pub async fn list_tasks_by_task_type(
    task_type: &String,
    ability: &String,
    page: u32,
    page_size: u32,
) -> Result<(Value, Vec<HashMap<String, Value>>)> {
    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} WHERE task_type = :task_type and ability = :ability order by id",
            ALL_FIELDS, TABLE_NAME
        ),
        &json!({ ":task_type": task_type ,"ability":ability}),
    )
    .await?;

    let cnt = {
        let mut count_map = query_with_args(
            &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
            &format!("SELECT count(1) cnt FROM {} WHERE task_type = :task_type and ability = :ability order by id", TABLE_NAME),
            &json!({ ":task_type": task_type ,"ability":ability}),
        )
        .await?;

        if let Some(item) = count_map.pop() {
            Ok(item.get("cnt").map_or(json!(0), |i| i.clone()))
        } else {
            Err(anyhow!("没有找到"))
        }
    }?;

    let abilities = query_with_args(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "SELECT {} FROM {} order by id limit {}, {}",
            ALL_FIELDS,
            TABLE_NAME,
            (page - 1) * page_size,
            page_size
        ),
        &json!({ ":task_type": task_type }),
    )
    .await?;

    Ok((cnt, abilities))
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

pub async fn insert_local_task(
    task_id: &String,
    task_type: &String,
    ability: &String,
    args: &String,
    request_task_id: u32,
    request_task_process_id: u32,
) -> Result<usize> {
    let res = execute(
        &crate::constants::ABILITIES_DATABASE_NAME.to_string(),
        &format!(
            "INSERT INTO {} (task_id, task_type, ability, args,request_task_id,request_task_process_id) VALUES(:task_id,:task_type, :ability, :args,:request_task_id,:request_task_process_id)",
            TABLE_NAME
        ),
        &json!({
            ":task_id": task_id,
            ":task_type": task_type,
            ":ability": ability,
            ":args":   args,
            ":request_task_id":   request_task_id,
            ":request_task_process_id":   request_task_process_id,
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
