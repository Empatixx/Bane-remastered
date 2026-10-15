"""A mine track across a room: a piece of rail that repeats, the buffer stop at each end, and the
mine cart in two pictures (its wheels turned a little between them).

rails.tga is four 16x16 cells in a row: rail, buffer stop, cart, cart rolling.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
W = H = 16
CLEAR = (0, 0, 0, 0)

PALETTE = {
    # Sleepers.
    "w": (92, 62, 40), "W": (128, 88, 56), "q": (58, 38, 26),
    # Rails.
    "r": (150, 146, 156), "R": (196, 192, 200), "e": (70, 66, 78),
    # Cart body: rusty iron with a lighter rim.
    "d": (34, 28, 30), "m": (96, 64, 52), "l": (132, 88, 64), "h": (176, 124, 84), "u": (150, 74, 46),
    # Wheels.
    "k": (40, 38, 46), "g": (110, 108, 120),
    # Buffer stop: a timber with an iron plate.
    "b": (120, 82, 50), "B": (160, 114, 70), "p": (84, 82, 96), "P": (140, 138, 150),
    "o": (28, 22, 20),
}

RAIL = [
    "................",
    "................",
    "..WW......WW....",
    "RRRRRRRRRRRRRRRR",
    "rrrrrrrrrrrrrrrr",
    "eewweeeeeewweeee",
    "..ww......ww....",
    "..ww......ww....",
    "..ww......ww....",
    "..ww......ww....",
    "RRRRRRRRRRRRRRRR",
    "rrrrrrrrrrrrrrrr",
    "eewweeeeeewweeee",
    "..qq......qq....",
    "................",
    "................",
]

BUFFER = [
    "................",
    "....oooooooo....",
    "...oBBBBBBBBo...",
    "...obbbbbbbbo...",
    "RRRobPPPPPPbo...",
    "rrrobpPPPPpbo...",
    "eeeobbbbbbbbo...",
    "...obbbbbbbbo...",
    "...obbbbbbbbo...",
    "...obpPPPPpbo...",
    "RRRobPPPPPPbo...",
    "rrrobbbbbbbbo...",
    "eeeoqqqqqqqqo...",
    "....oooooooo....",
    "................",
    "................",
]

CART = [
    "................",
    "..dddddddddddd..",
    ".dhhhhhhhhhhhhd.",
    ".dhllllllllllhd.",
    ".dlmdddddddmmld.",
    ".dlmdooooodmmld.",
    ".dlmdooooodmuld.",
    ".dlmdddddddmmld.",
    ".dlmmmmmmmmmmld.",
    ".dlllulllllllld.",
    ".dmmmmmmmmmmmmd.",
    "..dddddddddddd..",
    "..kgk......kgk..",
    "..kkk......kkk..",
    "................",
    "................",
]
CART_ROLLING = CART[:12] + [
    "..gkk......gkk..",
    "..kkg......kkg..",
    "................",
    "................",
]


def cell(rows):
    image = Image.new("RGBA", (W, H), CLEAR)
    for y, row in enumerate(rows):
        assert len(row) == W, row
        for x, key in enumerate(row):
            if key in PALETTE:
                image.putpixel((x, y), PALETTE[key] + (255,))
    return image


def main():
    sheet = Image.new("RGBA", (4 * W, H), CLEAR)
    for column, rows in enumerate([RAIL, BUFFER, CART, CART_ROLLING]):
        sheet.paste(cell(rows), (column * W, 0))
    sheet.save(OUT / "rails.tga")


if __name__ == "__main__":
    main()
