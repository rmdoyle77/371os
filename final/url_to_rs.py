import sys
import numpy as np
from PIL import Image
import requests
from io import BytesIO


IMG_URL = "https://cd-public.github.io/ai101/images/photo-cat.jpg"
VGA_W, VGA_H = 80, 25
OUT_FILE = "src/colors/img.rs"


VGA_COLORS = np.array([
    [0,   0,   0  ],  #  0 Black
    [0,   0,   170],  #  1 Blue
    [0,   170, 0  ],  #  2 Green
    [0,   170, 170],  #  3 Cyan
    [170, 0,   0  ],  #  4 Red
    [170, 0,   170],  #  5 Magenta
    [170, 85,  0  ],  #  6 Brown
    [170, 170, 170],  #  7 Light Gray
    [85,  85,  85 ],  #  8 Dark Gray
    [85,  85,  255],  #  9 Light Blue
    [85,  255, 85 ],  # 10 Light Green
    [85,  255, 255],  # 11 Light Cyan
    [255, 85,  85 ],  # 12 Light Red
    [255, 85,  255],  # 13 Light Magenta
    [255, 255, 85 ],  # 14 Yellow
    [255, 255, 255],  # 15 White
])


url = sys.argv[1] if len(sys.argv) > 1 else IMG_URL
img = np.array(Image.open(BytesIO(requests.get(url).content)).convert("RGB"))


pil_img = Image.fromarray(img).resize((VGA_W, VGA_H), Image.LANCZOS)
scaled = np.array(pil_img)  # shape: (25, 80, 3)

def nearest_color(pixel):
    distances = np.linalg.norm(VGA_COLORS - pixel, axis=1)
    return np.argmin(distances)

mapped = [nearest_color(scaled[r][c]) for r in range(VGA_H) for c in range(VGA_W)]

rust_array = ", ".join(str(v) for v in mapped)
with open(OUT_FILE, "w") as f:
    f.write(f"pub const IMAGE: [u8; {VGA_W * VGA_H}] = [{rust_array}];\n")

print(f"Written to {OUT_FILE}  ({VGA_W * VGA_H} entries)")
