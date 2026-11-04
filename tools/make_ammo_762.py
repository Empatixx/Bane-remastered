"""The 7.62 mm ammo item: four long bottle-necked rounds held at the base by a steel stripper clip,
in the brass, copper and outline colours of the other ammo boxes, with the same light glint
sliding across them. Eight 24x24 frames in a row, like the other boxes.

    uv run -q --with pillow python tools/make_ammo_762.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets/textures/ingame/ammobox_rifle.tga"
SIZE, FRAMES = 24, 8

COLOURS = {
    "k": (60, 34, 12),  # outline
    "L": (255, 214, 110),  # brass, lit
    "B": (232, 168, 40),  # brass
    "D": (176, 112, 24),  # brass, shade
    "T": (236, 140, 90),  # copper tip, lit
    "t": (196, 96, 52),  # copper tip
    "o": (34, 34, 42),  # clip outline
    "S": (176, 182, 192),  # clip, lit
    "s": (124, 128, 140),  # clip
    "d": (84, 86, 98),  # clip, shade
}

# One round, 5 wide; neighbours share their outline, so the four sit tight in the clip and part at
# the bottle necks.
ROUND = [
    "..k..",
    ".kTk.",
    ".kTk.",
    ".ktk.",
    ".kLk.",
    "kLBDk",
] + ["kLBDk"] * 11 + ["kDDDk"]

CLIP = [
    "ooooooooooooooooooo",
    "oSSSSSSSSSSSSSSSSSo",
    "osssssssssssssssssso"[:18] + "o",
    "odddddddddddddddddo",
    ".ooooooooooooooooo.",
]

ROUND_XS = (3, 7, 11, 15)
TOP = 1
CLIP_X = 2

# The glint: a diagonal band of light that crosses the brass in frames 3 to 5.
GLINT = (255, 240, 200)
GLINT_BANDS = {3: 8, 4: 16, 5: 24}
GLINT_WIDTH = 3


def frame(index):
    image = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    px = image.load()
    for x0 in ROUND_XS:
        for dy, line in enumerate(ROUND):
            for dx, key in enumerate(line):
                if key != ".":
                    px[x0 + dx, TOP + dy] = (*COLOURS[key], 255)
    clip_top = TOP + len(ROUND)
    for dy, line in enumerate(CLIP):
        for dx, key in enumerate(line):
            if key != ".":
                px[CLIP_X + dx, clip_top + dy] = (*COLOURS[key], 255)
    band = GLINT_BANDS.get(index)
    if band is not None:
        brass = {COLOURS[k] for k in "LBD"}
        for y in range(SIZE):
            for x in range(SIZE):
                if px[x, y][3] and px[x, y][:3] in brass and 0 <= (x + (SIZE - y)) - band < GLINT_WIDTH:
                    px[x, y] = (*GLINT, 255)
    return image


def main():
    sheet = Image.new("RGBA", (SIZE * FRAMES, SIZE), (0, 0, 0, 0))
    for index in range(FRAMES):
        sheet.alpha_composite(frame(index), (index * SIZE, 0))
    sheet.save(OUT)
    if len(sys.argv) > 1:
        board = Image.new("RGBA", sheet.size, (30, 28, 36, 255))
        board.alpha_composite(sheet)
        board.resize((sheet.width * 8, sheet.height * 8), Image.NEAREST).save(sys.argv[1])


if __name__ == "__main__":
    main()
