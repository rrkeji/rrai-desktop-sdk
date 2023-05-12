from diffusers import DiffusionPipeline

pipeline = DiffusionPipeline.from_pretrained("{{model_path}}")

image = pipeline("{{prompts}}").images[0]

image.save("./test2.png")