use anyhow::{anyhow, Result};
use hyper::{client::Client, Body, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;

/// 文字生图 POST /sdapi/v1/txt2img
pub async fn text2image() -> Result<String> {
    let json_obj = json!({
     "denoising_strength": 0,
     "prompt": "puppy dogs", //提示词
     "negative_prompt": "", //反向提示词
     "seed": -1, //种子，随机数
     "batch_size": 2, //每次张数
     "n_iter": 1, //生成批次
     "steps": 50, //生成步数
     "cfg_scale": 7, //关键词相关性
     "width": 512, //宽度
     "height": 512, //高度
     "restore_faces": false, //脸部修复
     "tiling": false, //可平埔
     "override_settings": {
         "sd_model_checkpoint" :"wlop-any.ckpt [7331f3bc87]"
    }, // 一般用于修改本次的生成图片的stable diffusion 模型，用法需保持一致
       "script_args": [
          0,
          true,
          true,
          "LoRA",
          "dingzhenlora_v1(fa7c1732cc95)",
          1,
          1
      ], // 一般用于lora模型或其他插件参数，如示例，我放入了一个lora模型， 1，1为两个权重值，一般只用到前面的权重值1
     "sampler_index": "Euler" //采样方法
        });
    webui_post("/sdapi/v1/txt2img", json_obj.to_string()).await
}

/// 图片生图 POST /sdapi/v1/img2img
pub async fn image2image() -> Result<String> {
    let json_obj = json!({});
    webui_post("/sdapi/v1/txt2img", json_obj.to_string()).await
}

/// 获取设置 GET /sdapi/v1/options
pub async fn get_options() -> Result<String> {
    webui_get("/sdapi/v1/txt2img").await
}

/// 获取设置 POST（可用来更新远端的模型） /sdapi/v1/options
pub async fn update_options() -> Result<String> {
    let json_obj = json!({});
    webui_post("/sdapi/v1/txt2img", json_obj.to_string()).await
}

/// 获取所有的模型 GET /sdapi/v1/sd-models
pub async fn get_sd_models() -> Result<String> {
    webui_get("/sdapi/v1/txt2img").await
}

async fn webui_get(path: &str) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!("{}{}", "http://127.0.0.1:7860", path.clone(),);
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("GET")
        .body(Body::from(""))
        .map_err(|err| {
            tracing::error!("调用{}接口封装request:{:?}", url, err);
            anyhow!("{}", err)
        })?;

    //
    let response = client
        .request(request)
        .await
        .map_err(|err| anyhow!("{}", err))?;

    //读入数据
    let bytes = hyper::body::to_bytes(response.into_body())
        .await
        .map_err(|err| anyhow!("{}", err))?;

    Ok(String::from_utf8(bytes.to_vec())?)
}

async fn webui_post(path: &str, request_obj: String) -> Result<String> {
    //
    let https = HttpsConnector::new();
    //构建Https Client
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = format!(
        "{}{}",
        crate::abilities::stable_diffusion_webui::get_webui_url(),
        path.clone(),
    );
    //请求的URL
    tracing::debug!("请求的URL:{}", url);

    //构建请求
    let request = Request::builder()
        .uri(url.as_str())
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(request_obj))
        .map_err(|err| {
            tracing::error!("调用{}接口封装request:{:?}", url, err);
            anyhow!("{}", err)
        })?;

    //
    let response = client
        .request(request)
        .await
        .map_err(|err| anyhow!("{}", err))?;

    //读入数据
    let bytes = hyper::body::to_bytes(response.into_body())
        .await
        .map_err(|err| anyhow!("{}", err))?;

    Ok(String::from_utf8(bytes.to_vec())?)
}
