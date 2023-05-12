use anyhow::{anyhow, Result};

/// 获取工程的路径
pub async fn delete_workspace(workspace_id: &String) -> Result<bool> {
    let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
        .join(crate::constants::WORKSPACES_ROOT_PATH)
        .join(workspace_id);

    Ok(true)
}
