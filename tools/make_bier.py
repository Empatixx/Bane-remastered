"""The sarcophagus the hero sleeps in, for the opening cinematic, seen from above and a little in
front like the rest of the dungeon.

sarcophagus.tga: two frames side by side, the open box with the hero laid inside (his own front
frame from player.tga, turned on its side, eyes shut), and the open box empty once he has risen.
sarcophagus_lid.tga: its carved stone lid, slid off the box in the game.

    uv run -q --with pillow python tools/make_bier.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
W, H = 50, 26
LID = (46, 20)

OUTLINE = (24, 18, 26)
STONE = [(46, 42, 58), (66, 62, 82), (92, 88, 110), (124, 120, 142), (158, 154, 176)]
INSIDE = [(14, 10, 18), (24, 18, 30)]
GOLD = [(120, 84, 26), (186, 140, 52), (236, 196, 98)]
SKIN = (196, 132, 96)


def box():
    """The open box: its rim round a dark hollow, its front face below with a carved band."""
    image = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = image.load()
    face = 19
    for y in range(H):
        for x in range(W):
            edge = y in (0, H - 1) or x in (0, W - 1) or y == face
            if edge:
                colour = OUTLINE
            elif y < face:
                rim = y < 3 or y > face - 3 or x < 3 or x > W - 4
                if rim:
                    colour = STONE[4] if y == 1 else STONE[3] if y < 3 else STONE[2]
                else:
                    colour = INSIDE[0] if y < 6 else INSIDE[1]
            else:
                colour = STONE[1] if y < face + 3 else STONE[0]
            px[x, y] = (*colour, 255)
    # The carved band on its front, gold studs at the corners.
    for x in range(4, W - 4, 5):
        px[x, face + 3] = (*OUTLINE, 255)
    for x in (2, W - 3):
        px[x, face + 2] = (*GOLD[2], 255)
        px[x, face + 3] = (*GOLD[1], 255)
    return image


def lid():
    """The lid: a slab with a raised rim, a carved figure with folded hands, gold at its ends."""
    w, h = LID
    image = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    px = image.load()
    for y in range(h):
        for x in range(w):
            edge = y in (0, h - 1) or x in (0, w - 1)
            if edge:
                colour = OUTLINE
            elif y == 1 or x == 1:
                colour = STONE[4]
            elif y == h - 2 or x == w - 2:
                colour = STONE[1]
            elif y < 4 or y > h - 5 or x < 4 or x > w - 5:
                colour = STONE[3]
            else:
                colour = STONE[2]
            px[x, y] = (*colour, 255)
    # The carved sleeper: a head at the left, the body down the slab, hands folded over a sword.
    for x, y in ((8, 8), (9, 7), (10, 7), (11, 8), (11, 9), (10, 10), (9, 10), (8, 9)):
        px[x, y] = (*STONE[1], 255)
    for x in range(13, 38):
        px[x, 7] = (*STONE[1], 255)
        px[x, 12] = (*STONE[1], 255)
    for x in range(14, 37):
        px[x, 8] = (*STONE[4], 255)
    for x in range(18, 34):
        px[x, 10] = (*GOLD[1], 255)
    px[17, 10] = (*GOLD[2], 255)
    for y in (9, 11):
        px[17, y] = (*GOLD[1], 255)
    for x in (3, w - 4):
        for y in (3, h - 4):
            px[x, y] = (*GOLD[2], 255)
    return image


def laid(open_box):
    sheet = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
    cell = (sheet.width // 6, sheet.height // 4)
    hero = sheet.crop((0, 0, *cell))
    hpx = hero.load()
    for y in range(cell[1]):
        for x in range(cell[0]):
            r, g, b, a = hpx[x, y]
            if a and (b > r + 20 or (r > 200 and g > 200 and b > 200)):
                hpx[x, y] = (*SKIN, 255)
    lying = hero.rotate(90, expand=True)
    out = open_box.copy()
    out.alpha_composite(lying, (6, 2))
    # The rim stays over him at the front, so he lies in the box, not on it.
    rim = open_box.crop((0, 15, W, 19))
    out.alpha_composite(rim, (0, 15))
    return out


def main():
    empty = box()
    sheet = Image.new("RGBA", (W * 2, H), (0, 0, 0, 0))
    sheet.alpha_composite(laid(empty), (0, 0))
    sheet.alpha_composite(empty, (W, 0))
    cover = lid()
    if len(sys.argv) > 1:
        preview = Image.new("RGBA", (W * 3 + 8, H + 4), (30, 28, 36, 255))
        preview.alpha_composite(sheet, (0, 2))
        closed = empty.copy()
        closed.alpha_composite(cover, (2, 0))
        preview.alpha_composite(closed, (W * 2 + 8, 2))
        preview.resize((preview.width * 8, preview.height * 8), Image.NEAREST).save(sys.argv[1])
        return
    sheet.save(ROOT / "assets/textures/ingame/sarcophagus.tga")
    cover.save(ROOT / "assets/textures/ingame/sarcophagus_lid.tga")


if __name__ == "__main__":
    main()
