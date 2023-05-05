use anyhow::{anyhow, Context, Result};
use serde::{Serialize, Serializer};
use std::{collections::HashMap, sync::Mutex};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};

use rrai_desktop_sdk_common::ipfs::ipfs_get_content;
use rrai_desktop_sdk_common::utils::rrai_home_path;

/// 下载应用
#[command]
pub async fn download(application_id: String, application_cid: String) -> crate::Result<()> {
    //
    //下载cid对应的zip文件
    let conent_vec = ipfs_get_content(&application_cid)
        .await
        .context("ipfs下载文件")?;

    //静态文件路径
    let storage_path = rrai_home_path()?
        .join("webroot/apps")
        .join(application_id.replace(":", "_"));

    std::fs::create_dir_all(storage_path.as_path())
        .map_err(|err| anyhow!("目录创建失败:{:?}-{}", storage_path, err))?;

    //解压zip文件
    rrai_desktop_sdk_common::utils::zip::extract_v8_to_fs(
        &conent_vec,
        storage_path.to_str().ok_or(anyhow!(""))?,
    )?;

    Ok(())
}
