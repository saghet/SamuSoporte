import sys
from PIL import Image, ImageDraw, ImageFilter, ImageOps

src_path = r"C:/Users/saghe/.gemini/antigravity/brain/b1823b29-1cce-4e6b-b8b6-691e8f2af532/.user_uploaded/media_1794269483291_b6ef8678.jpg"
out_dir = r"c:/Users/saghe/Documents/SamuSoporte/src/OpenSupport.App/Assets"

img = Image.open(src_path)

# 1. Recortar área centrada en el rostro del bebé (cuadrado perfecto)
w, h = img.size
# El rostro está en la parte central-inferior, excluimos marca de agua inferior
face_box = (int(w * 0.05), int(h * 0.15), int(w * 0.95), int(h * 0.85))
cropped = img.crop(face_box)
# Redimensionar a cuadrado grande 1024x1024
sq_size = min(cropped.size)
left = (cropped.width - sq_size) // 2
top = (cropped.height - sq_size) // 2
square_img = cropped.crop((left, top, left + sq_size, top + sq_size)).resize((1024, 1024), Image.Resampling.LANCZOS)

# 2. Crear versión circular estilizada con borde sutil para AppIcon
mask = Image.new('L', (1024, 1024), 0)
draw = ImageDraw.Draw(mask)
draw.ellipse((10, 10, 1014, 1014), fill=255)

icon_img = Image.new('RGBA', (1024, 1024), (0, 0, 0, 0))
icon_img.paste(square_img, (0, 0), mask=mask)

# Añadir un anillo estético moderno color turquesa/esmeralda característico de SamuSoporte
draw_border = ImageDraw.Draw(icon_img)
draw_border.ellipse((10, 10, 1014, 1014), outline=(16, 185, 129, 230), width=16)

# 3. Exportar a todas las escalas de Windows App SDK / WinUI
targets = {
    "Square44x44Logo.scale-200.png": (88, 88),
    "Square44x44Logo.targetsize-24_altform-unplated.png": (24, 24),
    "Square44x44Logo.targetsize-48_altform-lightunplated.png": (48, 48),
    "Square150x150Logo.scale-200.png": (300, 300),
    "LockScreenLogo.scale-200.png": (48, 48),
    "StoreLogo.png": (50, 50),
    "Wide310x150Logo.scale-200.png": (620, 300),
    "SplashScreen.scale-200.png": (1240, 600),
}

for name, (tw, th) in targets.items():
    p = f"{out_dir}/{name}"
    if tw == th:
        resized = icon_img.resize((tw, th), Image.Resampling.LANCZOS)
    else:
        # Formato panorámico (Splash / Wide)
        bg = Image.new('RGBA', (tw, th), (15, 23, 42, 255)) # Dark slate
        # Centrar logo circular
        center_icon = icon_img.resize((int(th * 0.8), int(th * 0.8)), Image.Resampling.LANCZOS)
        x = (tw - center_icon.width) // 2
        y = (th - center_icon.height) // 2
        bg.paste(center_icon, (x, y), mask=center_icon)
        resized = bg
    resized.save(p)
    print(f"Generado: {name}")

# Generar AppIcon.ico multi-resolución
icon_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
icon_img.save(f"{out_dir}/AppIcon.ico", format="ICO", sizes=icon_sizes)
print("Generado: AppIcon.ico")
