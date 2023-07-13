use crate::{models::AbilityEntity, utils::execute_command};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tera::{Context, Tera};

pub async fn perform_test() -> Result<String> {
    //获取配置信息
    let settings_values = crate::abilities::abilities_service::query_ability_settings(
        &String::from(crate::constants::STABLE_DIFFUSION_WEBUI_ABILITY_NAME),
    )
    .await?;

    Err(anyhow!("没有配置信息"))
}

pub async fn perform_task(task_id: &String, args: &String) -> Result<String> {
    //获取配置信息
    let settings_values = crate::abilities::abilities_service::query_ability_settings(
        &String::from(crate::constants::STABLE_DIFFUSION_WEBUI_ABILITY_NAME),
    )
    .await?;

    //TODO 获取到请求地址
    //调用/sdapi/v1/txt2img接口
    if let Ok(res_string) = super::webui_post("/sdapi/v1/txt2img", args.clone()).await {
        tracing::debug!("调用接口返回:{}!", res_string);
        Ok(res_string)
    } else {
        tracing::debug!("调用接口失败!");
        Err(anyhow!(""))
    }
}

pub async fn perform_task_and_block<F, R>(
    task_id: &String,
    args: &String,
    explorer: F,
) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    //获取配置信息
    //TODO 获取到请求地址
    //调用/sdapi/v1/txt2img接口
    tracing::debug!("开始请求/sdapi/v1/txt2img....");
    if let Ok(res_string) = super::webui_post("/sdapi/v1/txt2img", args.clone()).await {
        tracing::debug!("调用接口返回:{}!", res_string);
        let res_value: Value = serde_json::from_str(res_string.as_str())?;
        //
        // {
        //     "images": [
        //       "string"
        //     ],
        //     "parameters": {},
        //     "info": "string"
        //   }
        //获取到图片
        if let Some(Value::Array(images)) = res_value
            .as_object()
            .map_or(Err(anyhow!("返回的结果格式错误")), |obj| Ok(obj))?
            .get("images")
        {
            tracing::debug!("解析所有图片:{}", images.len());
            let mut images_array: Vec<String> = vec![];
            //上传图片
            for image_value in images {
                if let Some(image_base64) = image_value.as_str() {
                    tracing::debug!("解码前字符串:{}", image_base64);
                    //base64 转码
                    let content = general_purpose::STANDARD.decode(image_base64)?;

                    tracing::debug!("解码后数据长度:{}", content.len());

                    //上传文件获取到CID
                    let cid = explorer(content).await?;
                    images_array.push(cid);
                }
            }

            return Ok(json!(images_array).to_string());
        }
        tracing::debug!("数据格式不合法!");
        Err(anyhow!(""))
    } else {
        tracing::debug!("调用接口失败!");
        Err(anyhow!(""))
    }
}
