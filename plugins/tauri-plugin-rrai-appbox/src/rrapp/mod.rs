mod downloader;
mod manager;

pub use downloader::*;
pub use manager::*;

use anyhow::Result;
use rrai_desktop_sdk_common::utils::rrai_home_path;
use std::path::PathBuf;

pub fn get_rrapp_path(application_cid: &String) -> Result<PathBuf> {
    //静态文件路径
    let storage_path = rrai_home_path()?
        .join(crate::constants::APP_ROOT_PATH)
        .join(application_cid.replace(":", "_"));
    Ok(storage_path)
}

pub fn get_rrapp_file_path(application_cid: &String, file_relative: &String) -> Result<PathBuf> {
    //静态文件路径
    let file_path = rrai_home_path()?
        .join(crate::constants::APP_ROOT_PATH)
        .join(application_cid.replace(":", "_"))
        .join(file_relative);
    Ok(file_path)
}
