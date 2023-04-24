use actix_files as fs;
use actix_web::web::Data;
use actix_web::{get, http::header, web, App, HttpServer, Responder};
use anyhow::{anyhow, Context, Result};

use rrai_desktop_sdk_common::ipfs::ipfs_get_content;
use rrai_desktop_sdk_common::utils::rrai_home_path;

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }
}

impl Server {
    ///获取url的根路径
    pub fn get_root_uri() -> Result<String> {
        Ok(String::from("http://127.0.0.1:35080"))
    }
    /// 下载应用
    pub async fn download_file(cid: &String, extension: Option<String>) -> Result<()> {
        //下载cid对应的zip文件
        let conent_vec = ipfs_get_content(&cid).await.context("ipfs下载文件")?;
        //静态文件路径
        //TODO 文件数超出
        let storage_path = rrai_home_path()?.join("webroot/files");

        let file_name = if let Some(extension_str) = extension {
            let mut file_name = cid.clone();
            file_name.push_str(&extension_str);
            file_name
        } else {
            cid.clone()
        };

        rrai_desktop_sdk_common::utils::zip::write_file(
            &String::from(storage_path.to_str().ok_or(anyhow!("目录to_str失败"))?),
            file_name.as_str(),
            &conent_vec,
        )?;
        Ok(())
    }

    ///启动服务
    pub fn start(&self) -> Result<()> {
        //资源web服务
        tracing::info!("启动资源web服务");
        tokio::spawn(
            HttpServer::new(|| {
                //静态文件路径
                let storage_path = rrai_home_path().unwrap().join("webroot");
                std::fs::create_dir_all(storage_path.as_path());

                tracing::info!("server web root:{:?}.", storage_path.to_str());

                App::new()
                    .wrap(actix_web::middleware::Compress::default())
                    .wrap(
                        actix_web::middleware::DefaultHeaders::new()
                            .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff")),
                    )
                    .wrap(actix_web::middleware::NormalizePath::trim())
                    .wrap(actix_web::middleware::Logger::default())
                    .service(
                        fs::Files::new("/", storage_path.to_str().unwrap()).show_files_listing(),
                    )
            })
            .bind(("0.0.0.0", 35080))?
            .run(),
        );
        //代理服务
        tracing::info!("启动代理服务");
        tokio::spawn(
            HttpServer::new(|| {
                //
                let http_client = awc::Client::default();

                App::new()
                    .app_data(Data::new(http_client))
                    .wrap(actix_web::middleware::NormalizePath::trim())
                    .wrap(actix_web::middleware::Logger::default())
                    .service(crate::proxy::server_proxy)
            })
            .bind(("0.0.0.0", 35081))?
            .run(),
        );
        Ok(())
    }
}
