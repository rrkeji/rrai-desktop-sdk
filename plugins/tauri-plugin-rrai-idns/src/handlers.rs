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

#[derive(Default)]
pub struct ContextState(Mutex<HashMap<String, String>>);

#[command]
pub async fn set_context_value(
    state: State<'_, ContextState>,
    key: String,
    value: String,
) -> Result<bool> {
    state
        .0
        .lock()
        .map_err(|err| anyhow::anyhow!("获取锁失败！"))?
        .insert(key.clone(), value.clone());
    Ok(true)
}

#[command]
pub async fn get_context_value(state: State<'_, ContextState>, key: String) -> Result<String> {
    let map = state
        .0
        .lock()
        .map_err(|err| anyhow::anyhow!("获取锁失败！"))?;

    map.get(&key).map_or(
        Err(Error::Anyhow(anyhow::anyhow!("没有找到:{}", key))),
        |v| Ok(v.clone()),
    )
}

/// 通过模型和版本号获取schema
#[command]
pub async fn schema_by_model(
    state: State<'_, ContextState>,
    model_id: String,
    version: u32,
) -> Result<String> {
    let token = {
        let context = state
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!("获取锁失败:{}", err))?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };

    //获取token
    let res = crate::meta::schema_by_model(&token, &model_id, version).await?;
    Ok(res)
}

/// 获取数据集中的数据
#[command]
pub async fn dataset_rows(
    state: State<'_, ContextState>,
    dataset_id: String,
    parts: Option<String>,
    page_size: u32,
    page: u32,
) -> Result<String> {
    let token = {
        let context = state
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!("获取锁失败:{}", err))?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };
    //
    let res = crate::dataset::dataset_rows(&token, &dataset_id, parts, page_size, page).await?;
    Ok(res)
}

/// 插入数据集中的数据
#[command]
pub async fn insert_dataset_row(
    state: State<'_, ContextState>,
    dataset_id: String,
    row_cid: String,
    parts: String,
) -> Result<String> {
    let token = {
        let context = state
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!("获取锁失败:{}", err))?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };
    //
    let res = crate::dataset::insert_dataset_row(&token, &dataset_id, &row_cid, &parts).await?;
    Ok(res)
}

/// 插入数据集中的数据
#[command]
pub async fn update_dataset_row(
    state: State<'_, ContextState>,
    id: u32,
    row_cid: String,
    parts: String,
) -> Result<String> {
    let token = {
        let context = state.0.lock().map_err(
            |err: std::sync::PoisonError<std::sync::MutexGuard<HashMap<String, String>>>| {
                anyhow::anyhow!("获取锁失败:{}", err)
            },
        )?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };
    //
    let res = crate::dataset::update_dataset_row(&token, id, &row_cid, &parts).await?;
    Ok(res)
}
/// 插入数据集中的数据
#[command]
pub async fn remove_dataset_row(state: State<'_, ContextState>, id: u32) -> Result<String> {
    let token = {
        let context = state
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!("获取锁失败:{}", err))?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };
    //
    let res = crate::dataset::remove_dataset_row(&token, id).await?;
    Ok(res)
}

/// 插入数据集中的数据
#[command]
pub async fn query_dataset_row(
    state: State<'_, ContextState>,
    dataset_id: String,
    id: u32,
) -> Result<String> {
    let token = {
        let context = state
            .0
            .lock()
            .map_err(|err| anyhow::anyhow!("获取锁失败:{}", err))?;
        let token = context
            .get(&String::from(crate::constants::TOKEN_KEY))
            .map_or(
                Err(Error::Anyhow(anyhow::anyhow!("没有找到Token"))),
                |v| Ok(v.clone()),
            )?;
        token
    };
    //
    let res = crate::dataset::query_dataset_row(&token, &dataset_id, id).await?;
    Ok(res)
}
