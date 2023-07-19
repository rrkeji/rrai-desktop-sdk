use crate::{models::AbilityEntity, utils::execute_command_with_dir};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use futures::future::Future;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
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
        // tracing::debug!("调用接口返回:{}!", res_string);
        Ok(res_string)
    } else {
        tracing::debug!("调用接口失败!");
        Err(anyhow!(""))
    }
}

pub async fn perform_task_and_block<F, R>(
    task_id: &String,
    action: &String,
    args: &String,
    explorer: F,
) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    //action
    if action == "TXT2IMG" {
        //文生图
        _perform_task_and_block_txt2img(args, explorer).await
    } else if action == "IMG2IMG" {
        //文生图
        _perform_task_and_block_txt2img(args, explorer).await
    } else if action == "VIDEO2ANIMATION" {
        //文生图
        _perform_task_and_block_rrai_video2animation(args, explorer).await
    } else {
        Err(anyhow!("没有实现"))
    }
}

async fn _perform_task_and_block_txt2img<F, R>(args: &String, explorer: F) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    //获取配置信息
    //TODO 获取到请求地址
    //调用/sdapi/v1/txt2img接口
    tracing::debug!("开始请求/sdapi/v1/txt2img....");
    if let Ok(res_string) = super::webui_post("/sdapi/v1/txt2img", args.clone()).await {
        // tracing::debug!("调用接口返回:{}!", res_string);
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

//视频生成动画
async fn _perform_task_and_block_rrai_video2animation<F, R>(
    args: &String,
    explorer: F,
) -> Result<String>
where
    F: Fn(Vec<u8>) -> R,
    R: Future<Output = Result<String>>,
{
    //创建workspace
    // let workspace = crate::workspaces::Workspace::create()?;
    let workspace = crate::workspaces::Workspace::create_from_id("001")?;
    let workspace_path = workspace.path()?;
    tracing::debug!("workspace_path:{}", workspace_path);

    //初始创建一些文件夹
    workspace.mkdirs("video_frame")?;
    workspace.mkdirs("video_key")?;
    workspace.mkdirs("video_mask")?;
    workspace.mkdirs("img2img_key")?;
    workspace.mkdirs("img2img_upscale_key")?;

    workspace.mkdirs("src")?;

    //下载视频
    let video_path = "/Users/suhs/.rrai/workspaces/001/1.mp4";

    // 1. 创建关键帧
    // #ffmpeg.exe -ss 00:00:00  -y -i %1 -qscale 0 -f image2 -c:v png "%05d.png"
    // if let (Some(code), message) = if cfg!(target_os = "windows") {
    //     execute_command_with_dir(
    //         &workspace_path,
    //         &format!(
    //             "ffmpeg.exe -ss 00:00:00  -y -i {} -qscale 0 -f image2 -c:v png \"video_frame/%05d.png\"",
    //             video_path
    //         ),
    //     )
    //     .await?
    // } else {
    //     execute_command_with_dir(
    //         &workspace_path,
    //         &format!(
    //             "ffmpeg -ss 00:00:00  -y -i {} -qscale 0 -f image2 -c:v png \"video_frame/%05d.png\"",
    //             video_path
    //         ),
    //     )
    //     .await?
    // } {
    //     tracing::debug!("{}", message);

    //     if code == 0 {
    //         //
    //     } else {
    //         return Err(anyhow!("创建关键帧失败！"));
    //     }
    // } else {
    //     return Err(anyhow!("创建关键帧失败"));
    // }
    // 2. 关键帧分析
    // let mut context = Context::new();
    // context.insert("original_movie_path", "./1.mp4");
    // context.insert("frame_path", "./video_frame");
    // context.insert("frame_mask_path", "./video_mask");
    // context.insert("org_key_path", "./video_key");

    // context.insert("key_min_gap", "10");
    // context.insert("key_max_gap", "20");
    // context.insert("key_th", "1080");

    // let code = Tera::one_off(
    //     include_str!("../python/source/analyze_key_frames.py"),
    //     &context,
    //     false,
    // )
    // .map_err(|err| anyhow!(err))?;

    // //创建目录,并写入文件
    // workspace.add_file("src/analyze_key_frames.py", &code)?;

    // //执行
    // if let (Some(code), message) = execute_command_with_dir(
    //     &workspace_path,
    //     &format!("python3 src/analyze_key_frames.py",),
    // )
    // .await?
    // {
    //     tracing::debug!("{}", message);

    //     if code == 0 {
    //         //
    //     } else {
    //         return Err(anyhow!("关键帧分析失败！"));
    //     }
    // } else {
    //     return Err(anyhow!("关键帧分析失败"));
    // }

    // 3.创建关键帧的mask
    // if let Err(err) = _pbrem_predict_post(
    //     &format!("{}/video_key", workspace_path),
    //     &format!("{}/video_mask", workspace_path),
    // )
    // .await
    // {
    //     return Err(anyhow!("创建mask失败:{}", err));
    // }

    // 4.将关键帧生成tag
    // if let Err(err) = _tagger_interrogate(&format!("{}/video_key", workspace_path)).await {
    //     return Err(anyhow!("将关键帧生成tag失败:{}", err));
    // }

    // 5.将关键帧进行图生图
    // if let Err(err) = _video2animation_img2img(
    //     &format!("{}/video_key", workspace_path),
    //     &format!("{}/video_mask", workspace_path),
    //     &format!("{}/img2img_key", workspace_path),
    // )
    // .await
    // {
    //     return Err(anyhow!("将关键帧进行图生图失败:{}", err));
    // }
    // 6.放大图片到原始视频的尺寸

    // 7.重命名关键帧,生成 .ebs 文件
    // if let Err(err) = _video2animation_proj_file(&workspace).await {
    //     return Err(anyhow!("生成 .ebs 文件失败:{}", err));
    // }

    // 8.运行.ebs文件
    // if let Err(err) = _video2animation_run_proj(&workspace).await {
    //     return Err(anyhow!("生成 .ebs 文件失败:{}", err));
    // }

    Err(anyhow!(""))
}

async fn _pbrem_predict_post(key_path: &String, mask_path: &String) -> Result<()> {
    //
    for entry in walkdir::WalkDir::new(key_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name().to_str().unwrap().ends_with(".png"))
    {
        //
        let file = File::open(entry.path())?;
        let mut buf_reader = BufReader::new(file);
        // 解析配置文件
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;

        let content = general_purpose::STANDARD.encode(buffer);
        //
        let request = serde_json::json!({
          "img": content,
          "td_abg_enabled": false,
          "h_split": 256,
          "v_split": 256,
          "n_cluster": 500,
          "alpha": 50,
          "th_rate": 0.1,
          "cascadePSP_enabled": false,
          "fast": false,
          "psp_L": 900,
          "sa_enabled": false,
          "model_name": "sam_vit_h_4b8939.pth",
          "query": "",
          "predicted_iou_threshold": 0.9,
          "stability_score_threshold": 0.9,
          "clip_threshold": 0.1
        });

        if let Ok(res_string) = super::webui_post("/pbrem/predict", request.to_string()).await {
            tracing::debug!("调用接口返回:{}!", res_string);
            let res_value: Value = serde_json::from_str(res_string.as_str())?;
            //
            // {
            //     "img": "string",
            //     "mask": "string"
            //   }
            //获取到图片
            if let Some(Value::String(mask_str)) = res_value
                .as_object()
                .map_or(Err(anyhow!("返回的结果格式错误")), |obj| Ok(obj))?
                .get("mask")
            {
                //base64 转码
                let content = general_purpose::STANDARD.decode(mask_str)?;
                //保存文件
                tracing::debug!("解码后数据长度:{}", content.len());

                let filename = format!("{}/{}", mask_path, entry.file_name().to_str().unwrap());
                //创建文件
                let mut output = File::create(filename.as_str())?;
                output.write_all(&content)?;

                tracing::debug!("mask文件:{}", filename);
            }
        } else {
            tracing::debug!("调用接口失败!");
        }
    }
    Ok(())
}

async fn _tagger_interrogate(key_path: &String) -> Result<()> {
    for entry in walkdir::WalkDir::new(key_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name().to_str().unwrap().ends_with(".png"))
    {
        //
        //
        let file = File::open(entry.path())?;
        let mut buf_reader = BufReader::new(file);
        // 解析配置文件
        let mut buffer = Vec::new();
        buf_reader.read_to_end(&mut buffer)?;

        let content = general_purpose::STANDARD.encode(buffer);
        //
        let request = serde_json::json!({
          "image": content,
          "model": "wd14-swinv2-v2-git",
          "threshold": 0.35
        });
        if let Ok(res_string) =
            super::webui_post("/tagger/v1/interrogate", request.to_string()).await
        {
            tracing::debug!("调用接口返回:{}!", res_string);
            let res_value: Value = serde_json::from_str(res_string.as_str())?;
            //
            // {
            //     "caption": {
            //       "additionalProp1": 0,
            //       "additionalProp2": 0,
            //       "additionalProp3": 0
            //     }
            //   }
            //保存文件
            let filename = format!("{}/{}.txt", key_path, entry.file_name().to_str().unwrap());
            //创建文件
            let mut output = File::create(filename.as_str())?;
            output.write_all(res_string.as_bytes())?;

            tracing::debug!("prompts文件:{}", filename);
        } else {
            tracing::debug!("调用接口失败!");
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptFileEntity {
    pub caption: HashMap<String, f32>,
}

async fn _video2animation_img2img(
    key_path: &String,
    key_mask_path: &String,
    img2img_path: &String,
) -> Result<()> {
    //
    for entry in walkdir::WalkDir::new(key_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.file_name().to_str().unwrap().ends_with(".png"))
    {
        let prompts = {
            //
            let file = File::open(format!("{}.txt", entry.path().to_str().unwrap()).as_str())?;
            let mut buf_reader = BufReader::new(file);
            // 解析配置文件
            let mut buffer = Vec::new();
            buf_reader.read_to_end(&mut buffer)?;
            //序列号json
            let res_value: PromptFileEntity = serde_json::from_slice(&buffer)?;
            let mut res = String::new();

            for key in res_value.caption.keys() {
                res.push_str(key.as_str());
                res.push_str(",");
            }
            res
        };
        //
        let content = {
            //
            let file = File::open(entry.path())?;
            let mut buf_reader = BufReader::new(file);
            // 解析配置文件
            let mut buffer = Vec::new();
            buf_reader.read_to_end(&mut buffer)?;

            general_purpose::STANDARD.encode(buffer)
        };
        //
        let mask = {
            //
            let file = File::open(format!(
                "{}/{}",
                key_mask_path,
                entry.file_name().to_str().unwrap()
            ))?;
            let mut buf_reader = BufReader::new(file);
            // 解析配置文件
            let mut buffer = Vec::new();
            buf_reader.read_to_end(&mut buffer)?;

            general_purpose::STANDARD.encode(buffer)
        };
        //
        let request = serde_json::json!({
            "init_images": [
                content
            ],
            "prompt": format!("(best quality), ((masterpiece)), (highres), illustration, original, extremely detailed wallpaper, no background,{}",prompts),
            "negative_prompt": "(nsfw:2), (human face:1.3), badhandv4, easynegative, ng_deepnegative_v1_75t,sketches, (worst quality:2), (low quality:2), (normal quality:2),normal quality, ((monochrome)), ((grayscale)), see-through, skin spots, acnes, skin blemishes, bad anatomy,DeepNegative,(fat:1.2),facing away,effect,text",
            "override_settings": {
                "sd_model_checkpoint": "全网首发 WT anime_V1.0.safetensors [86618977ac]"
            },
            "seed": -1,
            "batch_size": 1,
            "n_iter": 1,
            "steps": 20,
            "cfg_scale": 7,
            "width": 512,
            "height": 768,
            "restore_faces": false,
            "tiling": false,
            "script_args": [],
            "sampler_index": "DPM++ SDE Karras",
            "mask":mask,
            "resize_mode": 1,
            "image_cfg_scale": 0,
            "denoising_strength": 0.8,
            "mask_blur": 10,
            "inpainting_fill": 0,
            "inpaint_full_res": true,
            "inpaint_full_res_padding": 0,
            "inpainting_mask_invert": 0,
            "initial_noise_multiplier": 0,
            "styles":[],
            "alwayson_scripts": {
                "ControlNet": {
                    "args": [
                        {
                            "enabled": true,
                            "module": "lineart_anime",
                            "model": "control_v11p_sd15s2_lineart_anime [3825e83e]",
                            "weight": 0.8,
                            "image":content,
                            "mask": "",
                            "invert_image": false,
                            "resize_mode": 0,
                            "rgbbgr_mode": false,
                            "lowvram": false,
                            "processor_res": 0,
                            "threshold_a": 64,
                            "threshold_b": 64,
                            "guidance_start": 0,
                            "guidance_end": 1,
                            "guessmode": false
                        },
                        {
                            "enabled": true,
                            "module": "tile_resample",
                            "model": "control_v11f1e_sd15_tile [a371b31b]",
                            "weight": 1,
                            "image":content,
                            "mask": "",
                            "invert_image": false,
                            "resize_mode": 0,
                            "rgbbgr_mode": false,
                            "lowvram": false,
                            "processor_res": 0,
                            "threshold_a": 64,
                            "threshold_b": 64,
                            "guidance_start": 0,
                            "guidance_end": 1,
                            "guessmode": false
                        }
                    ]
                }
            }
        });

        if let Ok(res_string) = super::webui_post("/sdapi/v1/img2img", request.to_string()).await {
            // tracing::debug!("调用接口返回:{}!", res_string);
            let res_value: Value = serde_json::from_str(res_string.as_str())?;
            //
            //获取到图片
            if let Some(Value::Array(images)) = res_value
                .as_object()
                .map_or(Err(anyhow!("返回的结果格式错误")), |obj| Ok(obj))?
                .get("images")
            {
                tracing::debug!("解析所有图片:{}", images.len());
                //上传图片
                let mut i = 0;
                for image_value in images {
                    i = i + 1;
                    if let Some(image_base64) = image_value.as_str() {
                        //base64 转码
                        let content = general_purpose::STANDARD.decode(image_base64)?;

                        tracing::debug!("解码后数据长度:{}", content.len());
                        let filename = format!(
                            "{}/{}{}",
                            img2img_path,
                            i,
                            entry.file_name().to_str().unwrap()
                        );
                        //创建文件
                        let mut output = File::create(filename.as_str())?;
                        output.write_all(&content)?;
                    }
                }
            }
        } else {
            tracing::debug!("调用接口失败!");
        }
    }
    Ok(())
}

async fn _video2animation_proj_file(workspace: &crate::workspaces::Workspace) -> Result<()> {
    let workspace_path = workspace.path()?;

    //
    let mut context = Context::new();
    context.insert("project_dir", "./1.mp4");
    context.insert("frame_path", "./video_frame");
    context.insert("frame_mask_path", "./video_mask");
    context.insert("img2img_key_path", "./img2img_key");
    context.insert("img2img_upscale_key_path", "./img2img_upscale_key");

    let code = Tera::one_off(
        include_str!("../python/source/ebsynth_stage5.py"),
        &context,
        false,
    )
    .map_err(|err| anyhow!(err))?;

    //创建目录,并写入文件
    workspace.add_file("src/ebsynth_stage5.py", &code)?;

    //执行
    if let (Some(code), message) =
        execute_command_with_dir(&workspace_path, &format!("python3 src/ebsynth_stage5.py",))
            .await?
    {
        tracing::debug!("{}", message);

        if code == 0 {
            //
        } else {
            return Err(anyhow!("工程文件创建失败！"));
        }
    } else {
        return Err(anyhow!("工程文件创建失败"));
    }

    Ok(())
}


async fn _video2animation_run_proj()->Result<()>{

    Ok(())
}