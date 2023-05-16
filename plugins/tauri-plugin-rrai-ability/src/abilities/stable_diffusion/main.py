from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained("{{model_path}}")

#    self,
#     prompt: Union[str, List[str]] = None,
#     image: Union[torch.FloatTensor, PIL.Image.Image] = None,
#     mask_image: Union[torch.FloatTensor, PIL.Image.Image] = None,
#     strength: float = 0.8,
#     num_inference_steps: Optional[int] = 50,
#     guidance_scale: Optional[float] = 7.5,
#     negative_prompt: Optional[Union[str, List[str]]] = None,
#     num_images_per_prompt: Optional[int] = 1,
#     add_predicted_noise: Optional[bool] = False,
#     eta: Optional[float] = 0.0,
#     generator: Optional[Union[torch.Generator, List[torch.Generator]]] = None,
#     prompt_embeds: Optional[torch.FloatTensor] = None,
#     negative_prompt_embeds: Optional[torch.FloatTensor] = None,
#     output_type: Optional[str] = "pil",
#     return_dict: bool = True,
#     callback: Optional[Callable[[int, int, torch.FloatTensor], None]] = None,
#     callback_steps: int = 1,


images = pipeline(
    prompt="{{prompts}}",
    negative_prompt="{{negative_prompt}}",
    num_inference_steps={{steps}},
    num_images_per_prompt={{batch_size}},
).images

for i in range(len(images)):
    images[i].save(f"./iamge_{i}.png")
