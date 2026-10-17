"""The stone bier the hero lies on in the crypt, for the opening cinematic: two frames side by
side, the hero laid on it with his eyes shut (his own front frame from player.tga, turned on
its side), and the bier empty once he has risen.

    uv run -q --with pillow python tools/make_bier.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
W, H = 48, 24

OUTLINE = (24, 18, 26)
STONE = [(54, 50, 66), (78, 74, 94), (104, 100, 122), (136, 132, 154)]
SKIN = (196, 132, 96)


def bier():
    """The bier from above and a little in front: its long top face, its front face below."""
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = image.load()
    face = 18
    for y in range(0, H):
        for x in range(1, W - 1):
            edge = y in (0, H - 1) or x in (1, W - 2)
            if edge:
                colour = OUTLINE
            elif y == face:
                colour = OUTLINE
            elif y < face:
                # The top: lit at its back edge, darker towards the front.
                colour = STONE[3] if y == 1 else STONE[2] if y < face - 4 else STONE[1]
            else:
                colour = STONE[0] if y > face + 3 else STONE[1]
            px[x, y] = (*colour, 255)
    for x in range(5, W - 5, 6):
        px[x, face + 3] = (*OUTLINE, 255)
        px[x + 1, face + 3] = (*OUTLINE, 255)
    for x, y in ((9, 4), (10, 5), (33, 13), (34, 14), (35, 14)):
        px[x, y] = (*STONE[1], 255)
    return image


def laid(bier_image):
    sheet = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
    hero = sheet.crop((0, 0, 16, 38))
    # Eyes shut: the blue of his eyes and their whites go under the lids.
    hpx = hero.load()
    for y in range(38):
        for x in range(16):
            r, g, b, a = hpx[x, y]
            if a and (b > r + 20 or (r > 200 and g > 200 and b > 200)):
                hpx[x, y] = (*SKIN, 255)
    lying = hero.rotate(90, expand=True)  # head to the left
    out = bier_image.copy()
    out.alpha_composite(lying, (5, 1))
    return out


def main():
    slab = bier()
    sheet = Image.new("RGBA", (W * 2, H), (0, 0, 0, 0))
    sheet.alpha_composite(laid(slab), (0, 0))
    sheet.alpha_composite(slab, (W, 0))
    if len(sys.argv) > 1:
        preview = Image.new("RGBA", sheet.size, (30, 28, 36, 255))
        preview.alpha_composite(sheet)
        preview.resize((sheet.width * 8, sheet.height * 8), Image.NEAREST).save(sys.argv[1])
        return
    sheet.save(ROOT / "assets/textures/ingame/bier.tga")


if __name__ == "__main__":
    main()
