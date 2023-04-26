pub mod migration;
mod rusqlite_utils;

use anyhow::{anyhow, Result};
use rusqlite::{types::Value as SqliteValue, Connection, OpenFlags, ToSql};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    pub(crate) static ref CONNECTIONS: RwLock<HashMap<String, Arc<Mutex<Connection>>>> =
        RwLock::new(HashMap::new());
}

pub async fn open(path: &String) -> Result<Arc<Mutex<Connection>>> {
    open_with_flags(path, OpenFlags::default()).await
}

pub async fn open_with_flags(path: &String, flags: OpenFlags) -> Result<Arc<Mutex<Connection>>> {
    //判断是否已经打开
    let exist = CONNECTIONS.read().unwrap().contains_key(path);

    if exist {
        if let Some(arc_conn) = CONNECTIONS.read().unwrap().get(path) {
            return Ok(arc_conn.clone());
        } else {
            Err(anyhow!("获取失败"))
        }
    } else {
        //
        let mut storage_path = crate::utils::rrai_home_path()?.join("sqlite");
        storage_path.push(path.clone());

        let prefix = storage_path.parent().unwrap_or(storage_path.as_path());
        std::fs::create_dir_all(prefix).map_err(|err| anyhow::anyhow!(err))?;

        let arc_conn = Arc::new(Mutex::new(Connection::open_with_flags(
            &storage_path,
            flags,
        )?));

        let mut cache = CONNECTIONS.write().unwrap();
        cache.insert(path.clone(), arc_conn.clone());

        Ok(arc_conn)
    }
}

pub async fn execute_sql(path: &String, sql: &String) -> Result<usize> {
    let arc_conn = open(path).await?;

    let conn = arc_conn
        .lock()
        .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

    let res = conn.execute(sql.as_str(), [])?;
    Ok(res)
}

pub async fn close(path: &String) -> Result<bool> {
    let arc_conn = open(path).await?;

    let conn = arc_conn
        .lock()
        .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

    drop(conn);
    //移除
    let mut cache = CONNECTIONS
        .write()
        .map_err(|err| anyhow!("获取锁失败:{}", err))?;

    cache.remove(path);
    Ok(true)
}

pub async fn execute_batch(path: &String, sql: &String) -> Result<bool> {
    let arc_conn = open(path).await?;

    let connection = arc_conn
        .lock()
        .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

    let res = connection.execute_batch(sql.as_str())?;
    Ok(true)
}

pub async fn execute(path: &String, sql: &String, args: &JsonValue) -> Result<usize> {
    let arc_conn = open(path).await?;

    let conn = arc_conn
        .lock()
        .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

    let mut args_sqlite_values = HashMap::<String, SqliteValue>::new();
    let mut named_args: Vec<(&str, &dyn ToSql)> = vec![];

    if let JsonValue::Object(json_value) = args {
        for (k, v) in json_value {
            args_sqlite_values.insert(k.clone(), rusqlite_utils::value_to_rusqlite_value(v)?);
            //
        }
    }

    for (k, v) in &args_sqlite_values {
        named_args.push((k, v as &dyn ToSql));
    }

    let res = conn.execute(sql.as_str(), &*named_args)?;
    return Ok(res);
}

pub async fn query_with_args(
    path: &String,
    sql: &String,
    args: &JsonValue,
) -> Result<Vec<HashMap<String, JsonValue>>> {
    let arc_conn = open(path).await?;

    let conn = arc_conn
        .lock()
        .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

    let mut stmt = conn.prepare(sql.as_str())?;

    let mut names: Vec<String> = Vec::new();
    for name in stmt.column_names() {
        names.push(name.to_string());
    }

    let mut args_sqlite_values = HashMap::<String, SqliteValue>::new();
    let mut named_args: Vec<(&str, &dyn ToSql)> = vec![];

    if let JsonValue::Object(json_value) = args {
        for (k, v) in json_value {
            args_sqlite_values.insert(k.clone(), rusqlite_utils::value_to_rusqlite_value(v)?);
        }
    }

    for (k, v) in &args_sqlite_values {
        named_args.push((k, v as &dyn ToSql));
    }

    let schema_iter = stmt.query_map(&*named_args, |row| {
        rusqlite_utils::rusqlite_row_to_map(row, &names)
            .map_err(|_e| rusqlite::Error::ExecuteReturnedResults)
    })?;

    let mut result = Vec::<HashMap<String, JsonValue>>::new();

    for table_result in schema_iter {
        if let Ok(row_value) = table_result {
            //
            result.push(row_value);
        }
    }
    Ok(result)
}
