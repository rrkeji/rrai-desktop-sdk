use crate::{models::AbilityEntity, tasks::async_execute_command, utils::execute_command};
use anyhow::{anyhow, Result};
use serde_json::Value;
use tera::{Context, Tera};

pub async fn perform_test() -> Result<String> {
    //获取配置信息
    let settings_values = crate::abilities::abilities_service::query_ability_settings(
        &String::from(crate::constants::STABLE_DIFFUSION_ABILITY_NAME),
    )
    .await?;

    //获取到
    if let Value::String(path_str) = settings_values
        .get(&String::from("model_path"))
        .map_or(Err(anyhow!("")), |path| Ok(path))?
    {
        // path_str
        tracing::debug!("model_path:{}", path_str);

        let mut context = Context::new();
        context.insert("model_path", &path_str);

        let code = Tera::one_off(include_str!("test_main.py"), &context, false)
            .map_err(|err| anyhow!(err))?;

        //创建目录,并写入文件
        let workspace = crate::workspaces::Workspace::create()?;

        workspace.add_file("main.py", &code)?;

        let workspace_path = workspace.path()?;
        tracing::debug!("workspace_path:{}", workspace_path);

        let test_command = format!("python3 {}{}", workspace_path, "/main.py");

        tracing::debug!("test_command:{}", test_command);
        //
        if let (Some(code), message) = execute_command(&test_command).await? {
            if code == 0 {
                tracing::debug!("测试成功");

                Ok(message)
            } else {
                Err(anyhow!("测试执行返回非0:{}", code))
            }
        } else {
            Err(anyhow!("测试执行异常!"))
        }
    } else {
        Err(anyhow!("没有配置信息"))
    }
}

pub async fn perform_task(args: &String) -> Result<String> {
    //获取配置信息
    let settings_values = crate::abilities::abilities_service::query_ability_settings(
        &String::from(crate::constants::STABLE_DIFFUSION_ABILITY_NAME),
    )
    .await?;

    //获取到
    if let Value::String(path_str) = settings_values
        .get(&String::from("model_path"))
        .map_or(Err(anyhow!("")), |path| Ok(path))?
    {
        // path_str
        tracing::debug!("model_path:{}", path_str);
        //args 反序列化
        let args_value: Value = serde_json::from_str(args)?;

        let workspace = crate::workspaces::Workspace::create()?;
        let workspace_path = workspace.path()?;
        tracing::debug!("workspace_path:{}", workspace_path);

        let mut context = Context::from_value(args_value)?;
        context.insert("model_path", &path_str);
        context.insert("workspace_path", &workspace_path);

        let code =
            Tera::one_off(include_str!("main.py"), &context, false).map_err(|err| anyhow!(err))?;

        //创建目录,并写入文件
        workspace.add_file("main.py", &code)?;
        workspace.mkdirs("outputs")?;

        let test_command = format!("python3 {}{}", workspace_path, "/main.py");
        tracing::debug!("test_command:{}", test_command);
        //
        let running_id = async_execute_command(&test_command).await?;
        tracing::debug!("返回执行命令的id:{}", running_id);
        Ok(running_id)
    } else {
        Err(anyhow!("没有配置信息"))
    }
}
