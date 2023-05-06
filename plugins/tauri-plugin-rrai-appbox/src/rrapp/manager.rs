use anyhow::{anyhow, Context, Result};
use serde::{Serialize, Serializer};
use std::{collections::HashMap, sync::Mutex};

/// 检查应用
pub async fn rrapp_check(application_cid: &String) -> Result<(u32, String)> {
    //检查是否存在

    //检查是否本地下载

    //检查所有文件是否被修改

    //返回
    Ok((0, String::new()))
}
