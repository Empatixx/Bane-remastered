"""The temple's acolytes for the opening cinematic: the hero's own sheet (player.tga, 6x4 frames of
16x38) redressed in the temple's colours, so they are drawn exactly as he is: a white robe for
his red shirt, royal-blue trousers, his gold strap kept as a sash, and dark hair.

    uv run -q --with pillow python tools/make_acolyte.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent

SWAP = {
    # Shirt, red to temple white.
    (141, 41, 28): (214, 206, 192),
    (109, 24, 16): (168, 158, 146),
    (166, 57, 38): (232, 226, 214),
    (161, 45, 28): (222, 214, 200),
    (180, 69, 46): (244, 240, 230),
    # Trousers, slate to royal blue.
    (46, 50, 62): (38, 50, 128),
    (32, 34, 44): (26, 32, 92),
    (62, 68, 82): (58, 76, 168),
    # Hair, auburn to near black.
    (78, 36, 29): (50, 42, 56),
    (115, 58, 48): (78, 68, 86),
    (56, 24, 22): (36, 30, 42),
    (66, 42, 30): (56, 48, 62),
    (58, 40, 36): (46, 40, 52),
    (58, 34, 24): (46, 38, 52),
    (50, 28, 22): (40, 34, 46),
    (104, 64, 44): (92, 82, 100),
    (146, 80, 62): (110, 100, 118),
    (40, 20, 16): (30, 24, 34),
}


def main():
    sheet = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
    px = sheet.load()
    for y in range(sheet.height):
        for x in range(sheet.width):
            r, g, b, a = px[x, y]
            if a and (r, g, b) in SWAP:
                px[x, y] = (*SWAP[(r, g, b)], a)
    if len(sys.argv) > 1:
        hero = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
        preview = Image.new("RGBA", (sheet.width * 2 + 8, sheet.height), (30, 28, 36, 255))
        preview.alpha_composite(hero, (0, 0))
        preview.alpha_composite(sheet, (sheet.width + 8, 0))
        preview.resize((preview.width * 4, preview.height * 4), Image.NEAREST).save(sys.argv[1])
        return
    sheet.save(ROOT / "assets/textures/ingame/acolyte.tga")


if __name__ == "__main__":
    main()
