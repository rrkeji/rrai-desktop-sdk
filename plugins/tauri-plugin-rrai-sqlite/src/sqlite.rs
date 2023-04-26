use rusqlite::OpenFlags;
use std::collections::HashMap;
use tauri::command;

use serde::{Serialize, Serializer};
use serde_json::Value as JsonValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    #[error("database {0} not opened")]
    DatabaseNotOpened(String),
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

// #[derive(Default)]
// pub struct SqliteMap(Mutex<HashMap<String, Connection>>);
// #[command]
// pub async fn open(state: State<'_, SqliteMap>, path: String) -> Result<bool> {
// }

#[command]
pub async fn open(path: String) -> Result<bool> {
    rrai_desktop_sdk_common::sqlite::open(&path).await?;
    Ok(true)
}

#[command]
pub async fn open_with_flags(path: String, iflags: i32) -> Result<bool> {
    let flags = OpenFlags::default();

    rrai_desktop_sdk_common::sqlite::open_with_flags(&path, flags).await?;
    Ok(true)
}

#[command]
pub async fn close(path: String) -> Result<bool> {
    rrai_desktop_sdk_common::sqlite::close(&path).await?;
    Ok(true)
}

#[command]
pub async fn execute_sql(path: String, sql: String) -> Result<usize> {
    let res = rrai_desktop_sdk_common::sqlite::execute_sql(&path, &sql).await?;
    Ok(res)
}

#[command]
pub async fn execute_batch(path: String, sql: String) -> Result<bool> {
    let res = rrai_desktop_sdk_common::sqlite::execute_batch(&path, &sql).await?;
    Ok(res)
}

#[command]
pub async fn execute(path: String, sql: String, args: JsonValue) -> Result<usize> {
    let res = rrai_desktop_sdk_common::sqlite::execute(&path, &sql, &args).await?;
    Ok(res)
}

#[command]
pub async fn query_with_args(
    path: String,
    sql: String,
    args: JsonValue,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let res = rrai_desktop_sdk_common::sqlite::query_with_args(&path, &sql, &args).await?;
    Ok(res)
}
