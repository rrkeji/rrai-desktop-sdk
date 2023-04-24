use super::files::write_to_file;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::fs::File;
use std::io::{copy, Cursor, Read, Seek, Write};
use std::path::{Path, PathBuf};

///解压
/// test.zip文件解压到d:/test文件夹下
///     let zipfile = std::fs::File::open(&zip_file_path).context(format!("打开文件{:?}失败", zip_file_path))?;
///
pub fn extract_v8_to_fs(zip_file: &Vec<u8>, target_input: &str) -> Result<()> {
    let mut file = Cursor::new(zip_file);
    let mut zip = zip::ZipArchive::new(file).context(format!("打开文件失败"))?;

    let mut target = String::from(target_input);

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        tracing::debug!("Filename: {} {:?}", file.name(), file.sanitized_name());

        if file.is_dir() {
            tracing::debug!("file utf8 path {:?}", file.name_raw());
            //文件名编码,在windows下用winrar压缩的文件夹，中文文夹件会码(发现文件名是用操作系统本地编码编码的，我的电脑就是GBK),本例子中的压缩的文件再解压不会出现乱码
            // let mut file_name = String::from(target);
            // file_name.push_str(&file.name().replace("\\", ""));
            //创建目录
        } else {
            let mut data = Vec::new();
            //
            file.read_to_end(&mut data).map_err(|e| {
                tracing::error!("{}", e);
            });
            tracing::debug!("write_to_file {:?}|{:?}", target, file.name());
            write_file(&target, &file.name(), &data)?;
        }
    }
    Ok(())
}

pub fn write_file(path: &String, filename: &str, content: &Vec<u8>) -> Result<()> {
    //
    let mut file_path = PathBuf::from(path.as_str()).join(filename);
    let dir_path = file_path.parent().ok_or(anyhow!("获取父路径失败"))?;
    std::fs::create_dir_all(dir_path)?;
    //创建文件
    let mut output = File::create(file_path)?;
    output.write_all(content)?;
    Ok(())
}
