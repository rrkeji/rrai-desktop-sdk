use anyhow::{anyhow, Result};
use serde_json::Value;
use std::io::BufRead;
use std::process::{Command, Stdio};
use tera::{Context, Tera};

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

pub async fn async_execute_command(command: &String) -> Result<(())> {
    //
    let mut child = if cfg!(target_os = "windows") {
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

    let out = child.stdout.take().unwrap();
    let mut out = std::io::BufReader::new(out);
    let mut s = String::new();
    while let Ok(_) = out.read_line(&mut s) {
        // 进程退出后结束循环
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
        println!("{}", s);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut context = Context::new();
    context.insert(
        "model_path",
        "/Users/suhs/works/diffusers/stable-diffusion-v1-5",
    );

    let code_str =
        Tera::one_off(include_str!("test_main.py"), &context, false).map_err(|err| anyhow!(err))?;

    println!("{}", code_str);

    // let (code, message) = execute_command(&String::from(
    //     "python3 /Users/suhs/.rrai/workspaces/test/main.py",
    // ))
    // .await?;
    // let (code, message) = execute_command(&String::from("top")).await?;
    // println!("{}:{}", code.unwrap(), message);

    let _ = async_execute_command(&String::from("top")).await?;

    Ok(())
}
