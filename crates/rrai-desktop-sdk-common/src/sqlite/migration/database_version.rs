use crate::sqlite::{execute, execute_batch, query_with_args};
use anyhow::{anyhow, Result};
use serde_json::{json, Value};

#[async_trait::async_trait]
pub trait DatabaseVersionSql {
    async fn version(&self) -> u32;
    async fn before(&self);
    async fn ddl(&self) -> Vec<String>;
    async fn after(&self);
}

pub async fn get_database_version(path: &String) -> Result<u32> {
    if !check_version_table(path).await? {
        Err(anyhow!("系统错误，没有数据库版本表!"))
    } else {
        //查询
        let versions = query_with_args(
            path,
            &"SELECT version FROM databases_version WHERE name = :name".to_string(),
            &json!({
                ":name":path.clone()
            }),
        )
        .await?;
        if versions.len() == 0 {
            Ok(0)
        } else {
            //
            let version = versions.get(0).map_or(0, |row| {
                if let Some(Value::Number(v)) = row.get("version") {
                    let v: u32 = v.as_u64().unwrap().try_into().unwrap();
                    v
                } else {
                    0
                }
            });
            Ok(version)
        }
    }
}

pub async fn set_database_version(path: &String, version: u32) -> Result<bool> {
    if !check_version_table(path).await? {
        Err(anyhow!("系统错误，没有数据库版本表!"))
    } else {
        //查询
        execute(path, &"INSERT INTO databases_version (name, version)VALUES (:name, :version) ON CONFLICT(name) DO UPDATE SET version = :version".to_string(), &json!({
            ":name":path.clone(),
            ":version":version
        })).await?;
        Ok(true)
    }
}

pub async fn merge_database_version<T>(path: &String, list: Vec<T>) -> Result<bool>
where
    T: DatabaseVersionSql,
{
    //当前的版本
    let version = get_database_version(path).await?;

    tracing::debug!("当前数据库版本:{}", version);
    //循环脚本
    for item in list {
        let item_version = item.version().await;

        if item_version > version {
            //before
            item.before().await;
            //ddl
            let ddls = item.ddl().await;

            for ddl_sql in ddls {
                execute_batch(path, &ddl_sql).await?;
            }
            //after
            item.after().await;

            set_database_version(path, item_version).await?;
        }
    }
    Ok(true)
}

#[derive(Debug)]
pub struct NormalDdlDatabaseVersionSql {
    version: u32,
    ddl: Vec<String>,
}
impl NormalDdlDatabaseVersionSql {
    pub fn new(version: u32, ddl: Vec<String>) -> Self {
        Self { version, ddl }
    }
}

#[async_trait::async_trait]
impl DatabaseVersionSql for NormalDdlDatabaseVersionSql {
    async fn version(&self) -> u32 {
        self.version
    }
    async fn before(&self) {
        tracing::debug!("nothing!");
    }
    async fn ddl(&self) -> Vec<String> {
        self.ddl.clone()
    }
    async fn after(&self) {
        tracing::debug!("nothing!");
    }
}

async fn check_version_table(path: &String) -> Result<bool> {
    let sql = String::from(
        "SELECT count(1) count FROM sqlite_master WHERE type='table' and name = :name ",
    );
    //
    let rows = query_with_args(
        path,
        &sql,
        &json!({
            ":name":"databases_version"
        }),
    )
    .await?;

    if rows.len() == 0 {
        //没有版本表
        execute_batch(
            path,
            &"CREATE TABLE databases_version (name TEXT, version INTEGER, unique(name));"
                .to_string(),
        )
        .await?;
        //
        Ok(true)
    } else {
        //
        let count = rows.get(0).map_or(0, |row| {
            if let Some(Value::Number(v)) = row.get("count") {
                let v: u32 = v.as_u64().unwrap().try_into().unwrap();
                v
            } else {
                0
            }
        });
        if count == 0 {
            execute_batch(
                path,
                &"CREATE TABLE databases_version (name TEXT, version INTEGER, unique(name));"
                    .to_string(),
            )
            .await?;
        }
        Ok(true)
    }
}
