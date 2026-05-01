import numpy as np
from PIL import Image

IMG_NAME = "dump.ppm"
img = np.array(Image.open(IMG_NAME))

colors_hex = []
for i in range(16):
    x = i * 45 + 22      
    y = 200              
    r, g, b = img[y][x]
    hex_val = f"#{r:02X}{g:02X}{b:02X}"
    colors_hex.append((i, hex_val, r, g, b))
    print(f"Color {i:2d}: {hex_val}  RGB({r}, {g}, {b})")
