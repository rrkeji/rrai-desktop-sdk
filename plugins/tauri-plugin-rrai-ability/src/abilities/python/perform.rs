use crate::{
    models::AbilityEntity,
    tasks::{async_execute_command, update_task_status},
    utils::execute_command,
};
use anyhow::{anyhow, Result};
use futures::future::Future;
use serde_json::{json, Value};
use tera::{Context, Tera};
use walkdir::{DirEntry, WalkDir};

pub async fn perform_test() -> Result<String> {
    Err(anyhow!("没有配置信息"))
}

pub async fn perform_task(task_id: &String, args: &String) -> Result<String> {
    //args 反序列化
    let args_value: Value = serde_json::from_str(args)?;

    //创建workspace
    let workspace = crate::workspaces::Workspace::create()?;
    let workspace_path = workspace.path()?;
    tracing::debug!("workspace_path:{}", workspace_path);

    let mut context = Context::from_value(args_value)?;
    context.insert("workspace_path", &workspace_path);

    //通过args中的_src加载代码
    let _src = context
        .get("_src")
        .map_or("./source/empty.py", |v| v.as_str().unwrap());

    //通过args中的_src_loader来判断代码的加载方式
    let _src_loader = context
        .get("_src_loader")
        .map_or("INLINE", |v| v.as_str().unwrap());

    let code = if _src_loader == "INLINE" {
        //
        if _src == "analyze_key_frames.py" {
            Tera::one_off(
                include_str!("./source/analyze_key_frames.py"),
                &context,
                false,
            )
            .map_err(|err| anyhow!(err))?
        } else {
            Tera::one_off(include_str!("./source/empty.py"), &context, false)
                .map_err(|err| anyhow!(err))?
        }
    } else {
        Tera::one_off(include_str!("./source/empty.py"), &context, false)
            .map_err(|err| anyhow!(err))?
    };

    //创建目录,并写入文件
    workspace.add_file("main.py", &code)?;
    workspace.mkdirs("outputs")?;

    let test_command = format!("python3 {}{}", workspace_path, "/main.py");
    tracing::debug!("test_command:{}", test_command);
    //
    let task_id_temp = task_id.clone();
    let workspace_id = workspace.id.clone();

    let running_id = async_execute_command(task_id, &test_command, async move {
        let workspace: crate::workspaces::Workspace =
            crate::workspaces::Workspace::create_from_id(workspace_id.as_str())?;

        //获取到outputs目录下的文件， 组装结果保存
        let files = workspace.list_files("outputs")?;

        let result = json!(files).to_string();
        tracing::debug!("任务完成:{}", result);
        //完成之后
        update_task_status(&task_id_temp, 2, &result).await;
        Ok(())
    })
    .await?;

    tracing::debug!("返回执行命令的id:{}", running_id);
    Ok(running_id)
}

pub async fn perform_task_and_block<F, R>(
    task_id: &String,
    args: &String,
    explorer: F,
) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    //args 反序列化
    let args_value: Value = serde_json::from_str(args)?;

    //创建workspace
    let workspace = crate::workspaces::Workspace::create()?;
    let workspace_path = workspace.path()?;
    tracing::debug!("workspace_path:{}", workspace_path);

    let mut context = Context::from_value(args_value)?;
    context.insert("workspace_path", &workspace_path);

    //通过args中的_src加载代码
    let _src = context
        .get("_src")
        .map_or("./source/empty.py", |v| v.as_str().unwrap());

    //通过args中的_src_loader来判断代码的加载方式
    let _src_loader = context
        .get("_src_loader")
        .map_or("INLINE", |v| v.as_str().unwrap());

    let code = if _src_loader == "INLINE" {
        //
        if _src == "analyze_key_frames.py" {
            Tera::one_off(
                include_str!("./source/analyze_key_frames.py"),
                &context,
                false,
            )
            .map_err(|err| anyhow!(err))?
        } else {
            Tera::one_off(include_str!("./source/empty.py"), &context, false)
                .map_err(|err| anyhow!(err))?
        }
    } else {
        Tera::one_off(include_str!("./source/empty.py"), &context, false)
            .map_err(|err| anyhow!(err))?
    };

    //创建目录,并写入文件
    workspace.add_file("main.py", &code)?;
    workspace.mkdirs("outputs")?;

    let test_command = format!("python3 {}{}", workspace_path, "/main.py");
    tracing::debug!("test_command:{}", test_command);
    //
    let task_id_temp = task_id.clone();
    let workspace_id = workspace.id.clone();

    let running_id = async_execute_command(task_id, &test_command, async move {
        let workspace: crate::workspaces::Workspace =
            crate::workspaces::Workspace::create_from_id(workspace_id.as_str())?;

        //获取到outputs目录下的文件， 组装结果保存
        let files = workspace.list_files("outputs")?;

        let result = json!(files).to_string();
        tracing::debug!("任务完成:{}", result);
        //完成之后
        update_task_status(&task_id_temp, 2, &result).await;
        Ok(())
    })
    .await?;

    tracing::debug!("返回执行命令的id:{}", running_id);
    Ok(running_id)
}
