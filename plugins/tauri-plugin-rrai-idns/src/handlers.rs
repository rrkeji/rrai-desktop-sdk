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
pub async fn dataset_rows_search(
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

/// 获取数据集中的数据
#[command]
pub async fn dataset_rows_search_owned(
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
    let res =
        crate::dataset::dataset_rows_search_owned(&token, &dataset_id, parts, page_size, page)
            .await?;
    Ok(res)
}

/// 获取数据集中的数据
#[command]
pub async fn dataset_rows_search_by_model(
    state: State<'_, ContextState>,
    model_id: String,
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
    let res =
        crate::dataset::dataset_rows_search_by_model(&token, &model_id, parts, page_size, page)
            .await?;
    Ok(res)
}

/// 插入数据集中的数据
#[command]
pub async fn dataset_create_row(
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
    let res = crate::dataset::dataset_create_row(&token, &dataset_id, &row_cid, &parts).await?;
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

/// 插入数据集中的数据
#[command]
pub async fn dataset_create_by_model_id(
    state: State<'_, ContextState>,
    model_id: String,
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
    let res = crate::dataset::dataset_create_by_model_id(&token, &model_id).await?;
    Ok(res)
}

/// 发布任务
#[command]
pub async fn tasks_task_publish(
    state: State<'_, ContextState>,
    name: String,
    task_type: String,
    model: String,
    model_args: String,
    description: String,
    assign_strategy: String,
    reward: u32,
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
    let res = crate::task::tasks_task_publish(
        &token,
        &name,
        &task_type,
        &model,
        &model_args,
        &description,
        &assign_strategy,
        reward,
    )
    .await?;
    Ok(res)
}

/// 获取任务
#[command]
pub async fn tasks_task_take(
    state: State<'_, ContextState>,
    peer_id: String,
    env: String,
    abilities: Vec<String>,
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
    let res = crate::task::tasks_task_take(&token, &peer_id, &env, &abilities).await?;
    Ok(res)
}

/// 获取任务
#[command]
pub async fn tasks_task_process_result(
    state: State<'_, ContextState>,
    task_id: u32,
    process_id: u32,
    progress: u16,
    result_code: u16,
    result: String,
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
    let res = crate::task::tasks_task_process_result(
        &token,
        task_id,
        process_id,
        progress,
        result_code,
        &result,
    )
    .await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_search(
    state: State<'_, ContextState>,
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
    let res = crate::ipfs::ipfs_files_search(&token, page_size, page).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_create(
    state: State<'_, ContextState>,
    parent_id: u64,
    cid: String,
    is_pin: u16,
    is_dir: u16,
    file_name: String,
    file_size: u32,
    file_type: String,
    avatar: String,
    category: String,
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
    let res = crate::ipfs::ipfs_files_create(
        &token, parent_id, &cid, is_pin, is_dir, &file_name, file_size, &file_type, &avatar,
        &category,
    )
    .await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_create_with_string_content(
    state: State<'_, ContextState>,
    paths: Vec<String>,
    content: String,
    file_name: String,
    file_type: String,
    category: String,
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
    let res = crate::ipfs::create_with_string_content(
        &token, &paths, &content, &file_type, &file_name, &category,
    )
    .await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_mkdirs(
    state: State<'_, ContextState>,
    paths: Vec<String>,
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
    let res = crate::ipfs::mkdirs(&token, &paths).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_update(
    state: State<'_, ContextState>,
    id: u32,
    file_name: String,
    avatar: String,
    category: String,
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
    let res = crate::ipfs::ipfs_files_update(&token, id, &file_name, &avatar, &category).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_files_remove(state: State<'_, ContextState>, id: u32) -> Result<String> {
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
    let res = crate::ipfs::ipfs_files_remove(&token, id).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_pins_status(state: State<'_, ContextState>, cid: String) -> Result<String> {
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
    let res = crate::ipfs::ipfs_pins_status(&token, &cid).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_pins_unpin(state: State<'_, ContextState>, cid: String) -> Result<String> {
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
    let res = crate::ipfs::ipfs_pins_unpin(&token, &cid).await?;
    Ok(res)
}

///  
#[command]
pub async fn ipfs_pins_pin(state: State<'_, ContextState>, cid: String) -> Result<String> {
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
    let res = crate::ipfs::ipfs_pins_pin(&token, &cid).await?;
    Ok(res)
}
