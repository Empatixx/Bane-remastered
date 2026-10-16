"""The lava floor's counterpart to the sewer's waterworks (see make_sewer.py), in the same
layout so the game draws both alike (16x16 cells, shown four times over):

- lava_crack.tga: two glowing cracks in a wall's face, in the wall's own stone colours, the
  molten rock welling out at their lower end.
- lava_fall.tga: four frames of lava running down the wall from the crack (top row) and four of
  it splashing into the pool at the wall's foot, sparks flying (bottom row).
- lava_debris.tga: what floats on the lava: three crusts of cooled black rock with glowing
  seams, three empty cells, and the glowing ring round anything afloat (the seventh column).
- lava_bridge.tga: a path of stepping stones over the lava, across the screen (top row: left
  end, middle, right end) and down it (bottom row: top end, middle, bottom end): two flat
  basalt stones to a tile, one at the bank's edge at each end, the lava showing between them.

Palettes from the lava tileset and the lava surface.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

# The lava floor's wall stone.
WALL0 = (20, 8, 8)
WALL1 = (60, 36, 36)
WALL2 = (80, 48, 44)
WALL3 = (110, 72, 64)
WALL4 = (140, 100, 90)
# Molten rock.
GLOW_H = (255, 236, 160)
GLOW = (255, 214, 96)
LAVA_L = (255, 140, 40)
LAVA = (222, 82, 28)
LAVA_D = (170, 44, 22)
EMBER = (140, 48, 22)
# Cooled rock.
ROCK0 = (14, 10, 12)
ROCK1 = (34, 26, 28)
ROCK2 = (56, 44, 44)
ROCK3 = (80, 66, 62)
# Basalt for the stepping stones.
BAS0 = (18, 14, 16)
BAS1 = (44, 38, 40)
BAS2 = (68, 60, 60)
BAS3 = (96, 86, 82)
BAS4 = (124, 112, 104)


class Cell:
    def __init__(self):
        self.image = Image.new("RGBA", (S, S), CLEAR)

    def put(self, x, y, colour, alpha=255):
        if 0 <= x < S and 0 <= y < S:
            self.image.putpixel((x, y), colour + (alpha,))

    def rect(self, x0, y0, x1, y1, colour, alpha=255):
        for y in range(y0, y1 + 1):
            for x in range(x0, x1 + 1):
                self.put(x, y, colour, alpha)

    def pattern(self, x, y, rows, palette):
        for dy, row in enumerate(rows):
            for dx, key in enumerate(row):
                if key in palette:
                    colour = palette[key]
                    if len(colour) == 4:
                        self.put(x + dx, y + dy, colour[:3], colour[3])
                    else:
                        self.put(x + dx, y + dy, colour)


def sheet(rows):
    out = Image.new("RGBA", (S * max(len(row) for row in rows), S * len(rows)), CLEAR)
    for r, row in enumerate(rows):
        for c, cell in enumerate(row):
            out.paste(cell.image, (c * S, r * S))
    return out


CRACK_PALETTE = {"d": WALL0, "m": WALL2, "l": WALL3, "h": WALL4, "e": EMBER, "r": LAVA_D, "o": LAVA, "y": LAVA_L, "g": GLOW}


def crack(variant):
    c = Cell()
    shapes = [
        [
            "......dd........",
            ".....dred.......",
            "....dreod.......",
            "...dreoyd.......",
            "....droyed......",
            ".....doyred.....",
            "....dmdoygod....",
            "...dlmdoyyod....",
            "....dddoggod....",
            "......doggod....",
            ".....dhdyyd.....",
            "......ddood.....",
        ],
        [
            ".........dd.....",
            "........dred....",
            ".......dreod....",
            "......droyed....",
            ".....dreoyd.....",
            "....dreoyd......",
            "...dmdoygd......",
            "...dldoyyod.....",
            "....ddogggd.....",
            "......doggod....",
            ".....dhdyyd.....",
            "......ddood.....",
        ],
    ]
    c.pattern(0, 1, shapes[variant], CRACK_PALETTE)
    # The rock round the crack, scorched.
    for x, y in ((5, 13), (10, 13), (4, 9), (11, 8)):
        c.put(x, y, EMBER, 160)
    return c


def fall(frame):
    c = Cell()
    for y in range(S):
        band = (y - frame * 4) % 8
        for x in range(5, 11):
            colour = LAVA
            if x in (5, 10):
                colour = LAVA_D
            elif band < 2 and x in (6, 7, 8):
                colour = GLOW
            elif band in (2, 3) and x in (7, 8):
                colour = LAVA_L
            elif band == 5 and x == 9:
                colour = LAVA_L
            c.put(x, y, colour)
    return c


def splash(frame):
    c = Cell()
    shapes = [
        ["...s...s..s...", "..ooyyyyyyoo..", ".oyyggggggyyo.", "..oyyggggyyo..", "....oooooo...."],
        [".s..s....s..s.", ".ooyyyyyyyyoo.", "oyyggggggggyyo", ".oyyggggggyyo.", "...oooooooo..."],
        ["..s..s..s..s..", "..ooyyyyyyoo..", ".oyyggggggyyo.", "..oyyggggyyo..", "....oooooo...."],
        ["s...s..s....s.", ".ooyyyyyyyyoo.", "oyyggggggggyyo", ".oyyggggggyyo.", "...oooooooo..."],
    ]
    c.pattern(1, 1, shapes[frame], {"s": GLOW_H, "o": LAVA_L, "y": GLOW, "g": GLOW_H})
    return c


def crust(variant):
    c = Cell()
    shapes = [
        [
            "...dddddd...",
            "..dmmhmmmd..",
            ".dmhhmmgmmd.",
            "dmmmgmmmmmmd",
            "dmmmmggmmhmd",
            ".dmmmmmmmmd.",
            "..dodddodd..",
        ],
        [
            ".ddddd....",
            "dmhmmmdd..",
            "dmmgmmhmd.",
            ".dmmgmmmmd",
            "..ddmmmod.",
            "....ddd...",
        ],
        [
            "..dddd..",
            ".dmhhmd.",
            "dmmgmmmd",
            "dmmmgmmd",
            ".dodmmd.",
            "..ddd...",
        ],
    ]
    palette = {"d": ROCK0, "m": ROCK1, "h": ROCK3, "g": LAVA_L, "o": LAVA}
    rows = shapes[variant]
    c.pattern((S - len(rows[0])) // 2, (S - len(rows)) // 2, rows, palette)
    return c


def ring():
    c = Cell()
    c.pattern(0, 9, [
        "....gggggggg....",
        "..gg........gg..",
        ".g............g.",
        "..gg........gg..",
        "....gggggggg....",
    ], {"g": GLOW + (170,)})
    return c


STONE_BIG = [
    "....ddddddd....",
    "..ddhhhhhhhdd..",
    ".dhhllllllllhd.",
    "dhllllllmllllhd",
    "dllllmlllllllld",
    "dlllllllllmllmd",
    "dllllllllllllmd",
    "dmllllmllllllmd",
    ".dmlllllllllmd.",
    "..dmmmmmmmmmd..",
    "...oddddddddo..",
]
STONE_SMALL = [
    "..dddddd..",
    ".dhhhhhhd.",
    "dhllllllmd",
    "dllllmllmd",
    "dlmllllmmd",
    ".dmmmmmmd.",
    "..oddddo..",
]


def stone(c, x, y, rows):
    """A flat stone sticking out of the lava, lit on top, glowing where the lava licks it."""
    c.pattern(x, y, rows, {"d": BAS0, "h": BAS4, "l": BAS3, "m": BAS2, "o": LAVA_L})


def stepping(part, across):
    """Stepping stones over the lava: a big flat stone to a tile in the middle, a smaller one at
    the bank's edge at each end, the lava showing between them."""
    c = Cell()
    if part == 1:
        stone(c, 0, 2, STONE_BIG)
    else:
        near_end = part == 0
        if across:
            stone(c, 6 if near_end else 0, 4, STONE_SMALL)
        else:
            stone(c, 3, 9 if near_end else 0, STONE_SMALL)
    return c


def main():
    sheet([[crack(0), crack(1)]]).save(OUT / "lava_crack.tga")
    sheet([[fall(f) for f in range(4)], [splash(f) for f in range(4)]]).save(OUT / "lava_fall.tga")
    sheet([[crust(0), crust(1), crust(2), Cell(), Cell(), Cell(), ring()]]).save(OUT / "lava_debris.tga")
    sheet([[stepping(p, True) for p in range(3)], [stepping(p, False) for p in range(3)]]).save(OUT / "lava_bridge.tga")


if __name__ == "__main__":
    main()
