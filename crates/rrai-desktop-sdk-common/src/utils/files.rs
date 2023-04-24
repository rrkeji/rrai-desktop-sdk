use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

pub fn write_to_file(path: &str, filename: &str, content: &Vec<u8>) -> Result<()> {
    //
    let storage_path = crate::utils::rrai_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;

    let filename = storage_path.join(filename);
    //创建文件
    let mut output = File::create(filename.as_path())?;
    output.write_all(content)?;
    Ok(())
}

pub fn read_string_from_file(path: &str, filename: &str) -> Result<String> {
    //
    let storage_path = crate::utils::rrai_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;
    let filename = storage_path.join(filename);
    if !filename.as_path().exists() {
        return Err(anyhow!("文件不存在!"));
    }
    let file = File::open(filename.as_path())?;
    let mut buf_reader = BufReader::new(file);
    // 解析配置文件
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

pub fn file_exists(path: &str, filename: &str) -> Result<bool> {
    //
    let storage_path = crate::utils::rrai_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;
    let filename = storage_path.join(filename);
    Ok(filename.as_path().exists())
}

pub fn file_delete(path: &str, filename: &str) -> Result<bool> {
    //
    let storage_path = crate::utils::rrai_home_path()?.join(path);
    std::fs::create_dir_all(storage_path.as_path())?;
    let filename = storage_path.join(filename);
    if !filename.as_path().exists() {
        return Ok(true);
    }
    std::fs::remove_file(filename)?;
    Ok(true)
}
