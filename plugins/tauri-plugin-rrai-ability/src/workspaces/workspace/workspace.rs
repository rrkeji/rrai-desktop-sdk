use anyhow::{anyhow, Result};
use std::io::Write;

use walkdir::{DirEntry, WalkDir};

pub struct Workspace {
    pub id: String,
}

impl Workspace {
    /// 通过ID进行创建
    pub fn create_from_id(id: &str) -> Result<Self> {
        let workspace_path = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(id);

        //判断目录是否存在
        if !workspace_path.as_path().exists() {
            //不存在，创建目录
            std::fs::create_dir_all(workspace_path).map_err(|err| anyhow::anyhow!(err))?;
        }

        Ok(Self {
            id: String::from(id),
        })
    }

    ///创建工程
    pub fn create() -> Result<Self> {
        //生成UUID,作为名称
        let uuid = uuid::Uuid::new_v4().to_string().replace("-", "");

        //获取主目录
        let workspace_path = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(uuid.clone());
        //创建目录
        std::fs::create_dir_all(workspace_path).map_err(|err| anyhow::anyhow!(err))?;

        Ok(Self { id: uuid })
    }
}

impl Workspace {
    ///添加文件
    pub fn add_file(&self, file_name: &str, content: &String) -> Result<bool> {
        //创建工程
        let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(self.id.clone())
            .join(file_name);

        //创建文件
        let mut output = std::fs::File::create(filename.as_path())?;
        output.write_all(content.as_bytes())?;

        Ok(true)
    }

    ///添加文件
    pub fn mkdirs(&self, file_name: &str) -> Result<bool> {
        //拼接路径
        let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(self.id.clone())
            .join(file_name);

        //创建目录
        std::fs::create_dir_all(filename).map_err(|err| anyhow::anyhow!(err))?;

        Ok(true)
    }

    /// list files
    pub fn list_files(&self, file_name: &str) -> Result<Vec<String>> {
        //拼接路径
        let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(self.id.clone())
            .join(file_name);

        Ok(WalkDir::new(filename)
            .into_iter()
            .filter_map(|v| v.ok().map_or(None, |entry|{
                if entry.file_type().is_dir(){None}else{Some(entry)}
            }))
            .map(|x| format!("{}", x.path().display()))
            .collect())
    }

    /// 获取工程的路径
    pub fn path(&self) -> Result<String> {
        let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(self.id.clone());

        Ok(filename.to_str().unwrap().to_string())
    }

    /// 获取工程的路径
    pub fn delete(&mut self) -> Result<bool> {
        let filename = rrai_desktop_sdk_common::utils::rrai_home_path()?
            .join(crate::constants::WORKSPACES_ROOT_PATH)
            .join(self.id.clone());

        if !filename.as_path().exists() {
            return Ok(true);
        }
        std::fs::remove_file(filename)?;
        Ok(true)
    }
}
