use anyhow::{anyhow, Result};
use deadqueue::unlimited::Queue;
use std::collections::HashMap;
use std::io::Error as IoError;
use std::io::{BufRead, Read};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;
use tokio::task::JoinError;
use tokio::task::{spawn, JoinHandle};

lazy_static::lazy_static! {
    pub(crate) static ref CURRENT_RUNNING_TASKS: RwLock<HashMap<String, Arc<Mutex<LocalProcessTask>>>> =
        RwLock::new(HashMap::new());
}

#[derive(Error, Debug)]
pub enum StdoutChannelError {
    #[error("task join error")]
    JoinError(#[from] JoinError),
    #[error("io error")]
    IoError(#[from] IoError),
}

type StdoutQueue<T> = Queue<T>;
type StdoutTask = JoinHandle<Result<(), StdoutChannelError>>;

pub struct LocalProcessTask {
    pub id: String,
    pub process_child: Child,
    stdout_queue: Arc<StdoutQueue<String>>,
    stderr_queue: Arc<StdoutQueue<String>>,
    pub stdout_task: Arc<Mutex<Option<StdoutTask>>>,
    pub stderr_task: Arc<Mutex<Option<StdoutTask>>>,
}

impl LocalProcessTask {
    /// 新建
    pub fn new(command: &String) -> Result<Self> {
        let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

        //
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|err| anyhow!("调用命令失败:{}", err))?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|err| anyhow!("调用命令失败:{}", err))?
        };
        // stdout
        let stdout_queue = Queue::new().into();
        let out = child.stdout.take().unwrap();
        let stdout_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stdout_queue);

            async move {
                let mut out = std::io::BufReader::new(out);
                let mut line = String::new();

                while let Ok(_) = out.read_line(&mut line) {
                    queue.push(line.clone());
                }
                Ok(())
            }
        })))
        .into();

        // stderr
        let stderr_queue = Queue::new().into();
        let stderr = child.stderr.take().unwrap();
        let stderr_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stderr_queue);

            async move {
                let mut stderr = std::io::BufReader::new(stderr);
                let mut line = String::new();

                while let Ok(_) = stderr.read_line(&mut line) {
                    queue.push(line.clone());
                }
                Ok(())
            }
        })))
        .into();

        Ok(Self {
            id: uuid,
            process_child: child,
            stdout_queue: stdout_queue,
            stderr_queue: stderr_queue,
            stdout_task: stdout_task,
            stderr_task: stderr_task,
        })
    }
}

pub async fn async_execute_command(command: &String) -> Result<String> {
    tracing::debug!("async_execute_command方法调用:{} ", command);
    //新建
    let task = LocalProcessTask::new(command)?;
    let task_id = task.id.clone();

    //
    let arc_task = Arc::new(Mutex::new(task));

    let mut cache = CURRENT_RUNNING_TASKS
        .write()
        .map_err(|err| anyhow!("{}", err))?;
    cache.insert(task_id.clone(), arc_task);

    Ok(task_id)
}

pub async fn running_command_stdout(task_id: &String) -> Result<Vec<String>> {
    tracing::debug!("running_command_stdout方法调用:{}", task_id);

    //判断是否已经打开
    let exist = CURRENT_RUNNING_TASKS
        .read()
        .map_err(|err| anyhow!("{}", err))?
        .contains_key(task_id);

    tracing::debug!("是否存在:{},{}", task_id, exist);
    if exist {
        if let Some(arc_task) = CURRENT_RUNNING_TASKS
            .read()
            .map_err(|err| anyhow!("{}", err))?
            .get(task_id)
        {
            let mut res = Vec::<String>::new();
            {
                let stdout_queue = arc_task
                    .lock()
                    .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?
                    .stdout_queue
                    .clone();

                while let Some(line) = stdout_queue.try_pop() {
                    res.push(line);
                }
            }
            tracing::debug!("获取行数:{}", res.len());
            Ok(res)
        } else {
            tracing::debug!("获取该任务失败:{}", task_id);
            Err(anyhow!("获取该任务失败"))
        }
    } else {
        tracing::debug!("不存在该任务:{}", task_id);
        //
        Err(anyhow!("不存在该任务"))
    }
}

pub async fn running_command_stderr(task_id: &String) -> Result<Vec<String>> {
    tracing::debug!("running_command_stdout方法调用:{}", task_id);

    //判断是否已经打开
    let exist = CURRENT_RUNNING_TASKS
        .read()
        .map_err(|err| anyhow!("{}", err))?
        .contains_key(task_id);

    tracing::debug!("是否存在:{},{}", task_id, exist);
    if exist {
        if let Some(arc_task) = CURRENT_RUNNING_TASKS
            .read()
            .map_err(|err| anyhow!("{}", err))?
            .get(task_id)
        {
            let task = arc_task
                .lock()
                .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

            let mut res = Vec::<String>::new();

            while let Some(line) = task.stderr_queue.try_pop() {
                res.push(line);
            }
            tracing::debug!("获取行数:{}", res.len());
            Ok(res)
        } else {
            tracing::debug!("获取该任务失败:{}", task_id);
            Err(anyhow!("获取该任务失败"))
        }
    } else {
        tracing::debug!("不存在该任务:{}", task_id);
        //
        Err(anyhow!("不存在该任务"))
    }
}

pub async fn running_command_status(task_id: &String, exit_remove: bool) -> Result<(bool, i32)> {
    tracing::debug!("running_command_stdout方法调用:{}", task_id);

    //判断是否已经打开
    let exist = CURRENT_RUNNING_TASKS
        .read()
        .map_err(|err| anyhow!("{}", err))?
        .contains_key(task_id);

    tracing::debug!("是否存在:{},{}", task_id, exist);
    if exist {
        if let Some(arc_task) = CURRENT_RUNNING_TASKS
            .read()
            .map_err(|err| anyhow!("{}", err))?
            .get(task_id)
        {
            let mut task = arc_task
                .lock()
                .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;

            if let Ok(Some(exit_status_obj)) = task.process_child.try_wait() {
                tracing::debug!("任务退出:{}", exit_status_obj);
                if exit_remove {
                    let mut cache = CURRENT_RUNNING_TASKS
                        .write()
                        .map_err(|err| anyhow!("获取锁失败:{}", err))?;

                    cache.remove(task_id);
                }
                Ok((true, exit_status_obj.code().map_or(0, |i| i)))
            } else {
                tracing::debug!("任务正在进行");
                Ok((false, 0))
            }
        } else {
            tracing::debug!("获取该任务失败:{}", task_id);
            Err(anyhow!("获取该任务失败"))
        }
    } else {
        tracing::debug!("不存在该任务:{}", task_id);
        //
        Err(anyhow!("不存在该任务"))
    }
}
