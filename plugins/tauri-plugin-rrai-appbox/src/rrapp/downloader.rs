use anyhow::{anyhow, Context, Result};
use serde::{Serialize, Serializer};
use std::{collections::HashMap, sync::Mutex};

use rrai_desktop_sdk_common::ipfs::ipfs_get_content;

/// 下载应用
pub async fn rrapp_download(application_cid: &String) -> Result<bool> {
    //TODO 先查看文件信息，太大不下载

    //下载cid对应的zip文件
    let conent_vec = ipfs_get_content(application_cid)
        .await
        .context("ipfs下载文件")?;

    //静态文件路径
    let storage_path = crate::rrapp::get_rrapp_path(application_cid)?;

    std::fs::create_dir_all(storage_path.as_path())
        .map_err(|err| anyhow!("目录创建失败:{:?}-{}", storage_path, err))?;

    //解压zip文件
    rrai_desktop_sdk_common::utils::zip::extract_v8_to_fs(
        &conent_vec,
        storage_path.to_str().ok_or(anyhow!(""))?,
    )?;

    Ok(true)
}
