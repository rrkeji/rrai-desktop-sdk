// {
//     "enable_hr": false,
//     "denoising_strength": 0,
//     "firstphase_width": 0,
//     "firstphase_height": 0,
//     "hr_scale": 2,
//     "hr_upscaler": "string",
//     "hr_second_pass_steps": 0,
//     "hr_resize_x": 0,
//     "hr_resize_y": 0,
//     "hr_sampler_name": "string",
//     "hr_prompt": "",
//     "hr_negative_prompt": "",
//     "prompt": "",
//     "styles": [
//       "string"
//     ],
//     "seed": -1,
//     "subseed": -1,
//     "subseed_strength": 0,
//     "seed_resize_from_h": -1,
//     "seed_resize_from_w": -1,
//     "sampler_name": "string",
//     "batch_size": 1,
//     "n_iter": 1,
//     "steps": 50,
//     "cfg_scale": 7,
//     "width": 512,
//     "height": 512,
//     "restore_faces": false,
//     "tiling": false,
//     "do_not_save_samples": false,
//     "do_not_save_grid": false,
//     "negative_prompt": "string",
//     "eta": 0,
//     "s_min_uncond": 0,
//     "s_churn": 0,
//     "s_tmax": 0,
//     "s_tmin": 0,
//     "s_noise": 1,
//     "override_settings": {},
//     "override_settings_restore_afterwards": true,
//     "script_args": [],
//     "sampler_index": "Euler",
//     "script_name": "string",
//     "send_images": true,
//     "save_images": false,
//     "alwayson_scripts": {}
//   }

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
