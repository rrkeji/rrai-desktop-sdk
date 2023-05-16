use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io::{BufRead, Read};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex, RwLock};

lazy_static::lazy_static! {
    pub(crate) static ref CURRENT_RUNNING_TASKS: RwLock<HashMap<String, Arc<Mutex<Child>>>> =
        RwLock::new(HashMap::new());
}

pub async fn execute_command(command: &String) -> Result<(Option<i32>, String)> {
    //
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .output()
            .map_err(|err| anyhow!("调用命令失败:{}", err))?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|err| anyhow!("调用命令失败:{}", err))?
    };

    let status = output.status;
    let stdout = output.stdout;
    let stderr = output.stderr;

    if status.success() {
        Ok((status.code(), String::from_utf8_lossy(&stdout).to_string()))
    } else {
        Ok((status.code(), String::from_utf8_lossy(&stderr).to_string()))
    }
}

pub async fn async_execute_command(command: &String) -> Result<String> {
    //
    let child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| anyhow!("调用命令失败:{}", err))?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|err| anyhow!("调用命令失败:{}", err))?
    };

    //生成一个UUID
    let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

    let arc_child = Arc::new(Mutex::new(child));

    let mut cache = CURRENT_RUNNING_TASKS
        .write()
        .map_err(|err| anyhow!("{}", err))?;
    cache.insert(uuid.clone(), arc_child.clone());

    Ok(uuid)
}

pub async fn running_command_stdout(
    running_id: &String,
    max_line_size: u16,
) -> Result<(bool, i32, String)> {
    tracing::debug!(
        "running_command_stdout方法调用:{},{}",
        running_id,
        max_line_size
    );

    //判断是否已经打开
    let exist = CURRENT_RUNNING_TASKS
        .read()
        .map_err(|err| anyhow!("{}", err))?
        .contains_key(running_id);

    tracing::debug!("是否存在:{},{}", running_id, exist);

    if exist {
        if let Some(arc_child) = CURRENT_RUNNING_TASKS
            .read()
            .map_err(|err| anyhow!("{}", err))?
            .get(running_id)
        {
            let mut child = arc_child
                .lock()
                .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;
            //

            let mut res = String::new();
            let mut line = String::new();
            let mut completed: bool = false;
            let mut exit_status: i32 = -1;
            let mut line_size: u16 = 0;

            tracing::debug!("判断是否退出:{}", running_id);

            if let Ok(Some(exit_status_obj)) = child.try_wait() {
                //判断是否已经完成
                tracing::debug!("判断退出:{} {}", running_id, exit_status_obj);

                completed = true;
                exit_status = exit_status_obj.code().map_or(0, |i| i);
            } else {
                tracing::debug!("读取:{}", running_id);
                let out = child.stdout.take().unwrap();
                // let mut out = std::io::BufReader::new(out);

                // while let Ok(_) = out.read_line(&mut line) {
                //     res.push_str(line.as_str());

                //     line_size = line_size + 1_u16;

                //     // 进程退出后结束循环
                //     if let Ok(Some(exit_status_obj)) = child.try_wait() {
                //         completed = true;
                //         exit_status = exit_status_obj.code().map_or(0, |i| i);
                //         break;
                //     }
                //     if line_size > max_line_size {
                //         break;
                //     }
                // }
                // if out.is_read_vectored() {}
            }
            tracing::debug!("返回:{}", running_id);
            Ok((completed, exit_status, res))
        } else {
            tracing::debug!("获取该任务失败:{}", running_id);
            Err(anyhow!("获取该任务失败"))
        }
    } else {
        tracing::debug!("不存在该任务:{}", running_id);
        //
        Err(anyhow!("不存在该任务"))
    }
}

pub async fn running_command_status(running_id: &String, exit_remove: bool) -> Result<(bool, i32)> {
    tracing::debug!(
        "running_command_status方法调用:{},{}",
        running_id,
        exit_remove
    );

    //判断是否已经打开
    let exist = CURRENT_RUNNING_TASKS
        .read()
        .map_err(|err| anyhow!("{}", err))?
        .contains_key(running_id);

    tracing::debug!("是否存在:{},{}", running_id, exist);

    if exist {
        if let Some(arc_child) = CURRENT_RUNNING_TASKS
            .read()
            .map_err(|err| anyhow!("{}", err))?
            .get(running_id)
        {
            let mut child = arc_child
                .lock()
                .map_err(|err| anyhow!("lock数据库连接失败:{}", err))?;
            //

            let mut completed: bool = false;
            let mut exit_status: i32 = -1;

            tracing::debug!("判断是否退出:{}", running_id);

            if let Ok(Some(exit_status_obj)) = child.try_wait() {
                tracing::debug!("判断退出:{} {}", running_id, exit_status_obj);
                //判断是否已经完成
                completed = true;
                exit_status = exit_status_obj.code().map_or(0, |i| i);

                if exit_remove {
                    let mut cache = CURRENT_RUNNING_TASKS
                        .write()
                        .map_err(|err| anyhow!("获取锁失败:{}", err))?;

                    cache.remove(running_id);
                }
            }
            tracing::debug!("返回:{}", running_id);
            Ok((completed, exit_status))
        } else {
            tracing::debug!("获取该任务失败:{}", running_id);
            Err(anyhow!("获取该任务失败"))
        }
    } else {
        tracing::debug!("不存在该任务:{}", running_id);
        //
        Err(anyhow!("不存在该任务"))
    }
}
