use anyhow::{anyhow, Result};
use futures::executor::block_on;
use std::collections::HashMap;
use std::future::Future;
use std::io::Error as IoError;
use std::io::{BufRead, Read};
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Arc, Mutex, RwLock};
use thiserror::Error;
use tokio::task::JoinError;
use tokio::task::{spawn, JoinHandle};

use super::{append_task_stdout, insert_local_task};

lazy_static::lazy_static! {
    pub(crate) static ref CURRENT_RUNNING_TASKS: RwLock<HashMap<String, Arc<Mutex<LocalProcessTask>>>> =
        RwLock::new(HashMap::new());
}

#[derive(Error, Debug)]
pub enum StdoutChannelError {
    #[error("anyhow")]
    AnyhowError(#[from] anyhow::Error),
    #[error("task join error")]
    JoinError(#[from] JoinError),
    #[error("io error")]
    IoError(#[from] IoError),
}

type StdoutTask = JoinHandle<Result<(), StdoutChannelError>>;

pub struct LocalProcessTask {
    pub id: String,
    pub process_child: Arc<Mutex<Child>>,
    pub stdout_task: Arc<Mutex<Option<StdoutTask>>>,
    pub stderr_task: Arc<Mutex<Option<StdoutTask>>>,
}

impl LocalProcessTask {
    /// 新建
    pub async fn new<T>(task_id: &String, command: &String, completed: T) -> Result<Self>
    where
        T: Future + Send + 'static,
    {
        //
        let child = if cfg!(target_os = "windows") {
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

        let child_arc = Arc::new(Mutex::new(child));

        // stderr
        let child_stderr = child_arc.clone();
        let stderr = child_stderr
            .lock()
            .map_err(|err| anyhow!(""))?
            .stderr
            .take()
            .unwrap();
        let task_id_temp = task_id.clone();
        let stderr_task = Mutex::new(Some(spawn({
            async move {
                let mut stderr = std::io::BufReader::new(stderr);
                let mut line = String::new();

                while let Ok(_) = stderr.read_line(&mut line) {
                    // 进程退出后结束循环
                    if let Ok(Some(exit_status_obj)) =
                        child_stderr.lock().map_err(|err| anyhow!(""))?.try_wait()
                    {
                        //
                        block_on(completed);
                        //
                        let mut cache = CURRENT_RUNNING_TASKS
                            .write()
                            .map_err(|err| anyhow!("获取锁失败:{}", err))?;

                        cache.remove(&task_id_temp);
                        break;
                    }
                    append_task_stdout(&task_id_temp, &String::new(), &line.clone()).await?;
                }
                Ok(())
            }
        })))
        .into();

        // stdout
        let child_stdout = child_arc.clone();
        let out = child_stdout
            .lock()
            .map_err(|err| anyhow!(""))?
            .stdout
            .take()
            .unwrap();
        let task_id_temp = task_id.clone();
        let stdout_task = Mutex::new(Some(spawn({
            async move {
                let mut out = std::io::BufReader::new(out);
                let mut line = String::new();

                while let Ok(_) = out.read_line(&mut line) {
                    // 进程退出后结束循环
                    if let Ok(Some(exit_status_obj)) =
                        child_stdout.lock().map_err(|err| anyhow!(""))?.try_wait()
                    {
                        //
                        let mut cache = CURRENT_RUNNING_TASKS
                            .write()
                            .map_err(|err| anyhow!("获取锁失败:{}", err))?;

                        cache.remove(&task_id_temp);
                        break;
                    }
                    append_task_stdout(&task_id_temp, &line.clone(), &String::new()).await?;
                }
                Ok(())
            }
        })))
        .into();

        Ok(Self {
            id: task_id.clone(),
            process_child: child_arc.clone(),
            stdout_task: stdout_task,
            stderr_task: stderr_task,
        })
    }
}

pub async fn async_execute_command<T>(
    task_id: &String,
    command: &String,
    completed: T,
) -> Result<String>
where
    T: Future<Output = Result<()>> + Send + 'static,
{
    tracing::debug!("async_execute_command方法调用:{} ", command);
    //新建
    let task = LocalProcessTask::new(task_id, command, completed).await?;
    let task_id = task.id.clone();

    //
    let arc_task = Arc::new(Mutex::new(task));

    let mut cache = CURRENT_RUNNING_TASKS
        .write()
        .map_err(|err| anyhow!("{}", err))?;
    cache.insert(task_id.clone(), arc_task);

    Ok(task_id)
}
