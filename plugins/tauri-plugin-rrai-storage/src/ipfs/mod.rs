use anyhow::{anyhow, Result};
use rrai_desktop_sdk_common::utils::rrai_home_path;
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

pub fn get_rrai_ipfs_home() -> Result<String> {
    Ok(format!(
        "{:?}",
        rrai_home_path()?.join("ipfs").as_path().as_os_str()
    ))
}

///
pub fn check_installed() -> Result<bool> {
    //
    Ok(rrai_home_path()?
        .join("ipfs")
        .join("config")
        .as_path()
        .exists())
}

///
pub fn ipfs_init() -> Result<bool> {
    //
    std::fs::create_dir_all(rrai_home_path()?.join("ipfs").join("ipfs2222").as_path())?;

    let mut envs = HashMap::new();

    envs.insert("IPFS_PATH".into(), get_rrai_ipfs_home()?);

    let init_output = tauri::api::process::Command::new_sidecar("ipfs")
        .expect("failed to create `ipfs` binary command")
        .envs(envs)
        .args(["init"])
        .output()?;

    tracing::info!("ipfs init status:{}", init_output.status.success());
    tracing::info!("output:{}", init_output.stdout);
    tracing::info!("stderr:{}", init_output.stderr);

    let _ = update_ipfs_config();

    Ok(init_output.status.success())
}

///
pub fn update_ipfs_config() -> Result<bool> {
    //修改配置文件
    let config_path = rrai_home_path()?.join("ipfs").join("config");
    let file = File::open(config_path.as_path())?;
    let mut buf_reader = BufReader::new(file);
    // 解析配置文件
    let mut buffer = Vec::new();
    buf_reader.read_to_end(&mut buffer)?;

    //
    let mut config_value: Value = serde_json::from_slice(&buffer)?;
    //Bootstrap
    //"/ip4/49.232.102.140/tcp/4001/p2p/12D3KooWRJuU3Z78e3hJR2mn59eaaPrLzb23rRcnyZfwLhjqdydv"

    match config_value {
        Value::Object(mut config_obj) => {
            *config_obj.get_mut("Bootstrap").map_or(Err(anyhow!("")), |item|Ok(item))? = Value::Array(vec![json!("/ip4/49.232.102.140/tcp/4001/p2p/12D3KooWRJuU3Z78e3hJR2mn59eaaPrLzb23rRcnyZfwLhjqdydv")]);

            //保存文件
            let mut output = File::create(config_path.as_path())?;
            let content = Value::Object(config_obj).to_string();

            output.write_all(&content.as_bytes())?;
        }
        _ => {}
    }

    //集群文件
    let swarm_path = rrai_home_path()?.join("ipfs").join("swarm.key");
    let mut output = File::create(swarm_path.as_path())?;
    let content = String::from("/key/swarm/psk/1.0.0/\n/base16/\n8bc74d715ec9a06a734fac68e51ed91178b7e2861497eaa5ac98b9a319966015");

    output.write_all(&content.as_bytes())?;

    Ok(true)
}
