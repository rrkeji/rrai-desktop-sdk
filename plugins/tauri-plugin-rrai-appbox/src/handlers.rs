use serde::{Serialize, Serializer};
use std::{collections::HashMap, sync::Mutex};
use tauri::{command, State};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

type Result<T> = std::result::Result<T, Error>;

/// 下载应用
#[command]
pub async fn rrapp_download(application_cid: String) -> Result<bool> {
    let res = crate::rrapp::rrapp_download(&application_cid).await?;
    Ok(res)
}

/// 应用检查
#[command]
pub async fn rrapp_check(application_cid: String) -> Result<(u32, String)> {
    let res = crate::rrapp::rrapp_check(&application_cid).await?;
    Ok(res)
}
