use anyhow::{anyhow, Result};
use std::io::Write;

///创建工程
pub async fn create() -> Result<String> {
    //生成UUID,作为名称
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

    //获取主目录
    let workspace_path = rrai_desktop_sdk_common::utils::rrai_home_path()?
        .join(crate::constants::WORKSPACES_ROOT_PATH)
        .join(uuid.clone());
    //创建目录
    std::fs::create_dir_all(workspace_path).map_err(|err| anyhow::anyhow!(err))?;

    Ok(uuid)
}

pub async fn create_by_file(file_name: &str, content: &String) -> Result<String> {
    //创建工程
    let workspace_id = create().await?;

    let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
        .join(crate::constants::WORKSPACES_ROOT_PATH)
        .join(workspace_id.clone())
        .join(file_name);

    //创建文件
    let mut output = std::fs::File::create(filename.as_path())?;
    output.write_all(content.as_bytes())?;

    Ok(workspace_id)
}

/// 获取工程的路径
pub async fn workspace_path(workspace_id: &String) -> Result<String> {
    let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
        .join(crate::constants::WORKSPACES_ROOT_PATH)
        .join(workspace_id);

    Ok(filename.to_str().unwrap().to_string())
}
