"""The HUD round for the 7.62 mm guns (Sniper and M4): a live rifle cartridge lying on its side,
pointing right - rim and extractor groove, a long brass case with a glint, the shoulder, the neck
and a pointed copper bullet, as bright as the pistol round. Drawn in tools/art/ammo_762.ase.
Same 32x32 layout as the other HUD rounds: the full round at the top left, then on the bottom
row the white flash and the grey empty one, outline kept.

    uv run -q --with pillow python tools/make_ammo_762_hud.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets/textures/ingame/ammo_762_hud.tga"
CELL = 16
TOP = 5

OUTLINE = (60, 34, 12)
COLOURS = {
    "k": OUTLINE,
    "R": (200, 132, 32),  # rim
    "Y": (255, 224, 138),  # rim, lit
    "W": (255, 244, 200),  # glint
    "L": (255, 214, 110),  # brass, lit
    "B": (240, 176, 40),  # brass
    "D": (184, 116, 24),  # brass, shade
    "N": (216, 154, 42),  # neck
    "T": (244, 135, 90),  # copper, lit
    "t": (196, 80, 44),  # copper
}
FLASH = (255, 243, 214)
EMPTY = (160, 160, 160)

ROUND = [
    "kk.kkkkkk.......",
    "kRkWWLLLLkkkkk..",
    "kYkLBBBBBLNTWTk.",
    "kRkBBBBBBBNTTTTk",
    "kRkDDDDDDDNtttk.",
    "kRkDDDDDDkkkkk..",
    "kk.kkkkkk.......",
]


def cell(fill=None):
    image = Image.new("RGBA", (CELL, CELL), (0, 0, 0, 0))
    px = image.load()
    for y, line in enumerate(ROUND):
        for x, key in enumerate(line):
            if key == ".":
                continue
            colour = COLOURS[key] if fill is None or key == "k" else fill
            px[x, TOP + y] = (*colour, 255)
    return image


def main():
    sheet = Image.new("RGBA", (CELL * 2, CELL * 2), (0, 0, 0, 0))
    sheet.alpha_composite(cell(), (0, 0))
    sheet.alpha_composite(cell(FLASH), (0, CELL))
    sheet.alpha_composite(cell(EMPTY), (CELL, CELL))
    sheet.save(OUT)
    if len(sys.argv) > 1:
        board = Image.new("RGBA", sheet.size, (30, 28, 36, 255))
        board.alpha_composite(sheet)
        board.resize((sheet.width * 12, sheet.height * 12), Image.NEAREST).save(sys.argv[1])


if __name__ == "__main__":
    main()
