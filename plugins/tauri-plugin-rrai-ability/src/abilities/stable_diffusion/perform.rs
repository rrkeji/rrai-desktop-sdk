use crate::{models::AbilityEntity, utils::execute_command};
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
        let workspace_id = crate::workspaces::create_by_file("main.py", &code).await?;

        let workspace_path = crate::workspaces::workspace_path(&workspace_id).await?;
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

pub async fn perform_task() -> Result<String> {
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
        let workspace_id = crate::workspaces::create_by_file("main.py", &code).await?;

        let workspace_path = crate::workspaces::workspace_path(&workspace_id).await?;
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
