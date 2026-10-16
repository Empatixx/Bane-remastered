"""The ice floor's counterpart to the sewer's waterworks (see make_sewer.py), in the same layout
so the game draws all of them alike (16x16 cells, shown four times over):

- ice_spout.tga: two cracks in a wall's face in the wall's own colours, icicles hanging from
  their lips, where the frozen falls come out.
- ice_fall.tga: four frames of a frozen fall running down the wall, a glint sliding down it
  (top row), and four of the mound of ice it has piled up at the wall's foot, glinting (bottom
  row).
- ice_debris.tga: what lies frozen under the ice: a fish, a skeleton's hand, a sword and a few
  coins; two cracks in the ice; and a drift of snow against a wall (the seventh column).

Palettes from the ice tileset and the ice surface.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

WALL0 = (14, 24, 40)
WALL1 = (50, 72, 108)
WALL2 = (80, 108, 146)
WALL3 = (150, 190, 214)
ICE0 = (90, 140, 190)
ICE1 = (118, 172, 214)
ICE2 = (150, 198, 232)
ICE3 = (190, 226, 246)
ICE4 = (226, 246, 255)
SHINE = (255, 255, 255)
SNOW = (236, 244, 250)
SNOW_D = (196, 214, 230)
FISH = (120, 140, 150)
FISH_D = (70, 86, 96)
BONE = (214, 206, 186)
BONE_D = (150, 142, 124)
STEEL = (170, 176, 190)
STEEL_D = (100, 106, 120)
HILT = (120, 80, 40)
GOLD = (230, 190, 80)
GOLD_D = (170, 130, 40)


class Cell:
    def __init__(self):
        self.image = Image.new("RGBA", (S, S), CLEAR)

    def put(self, x, y, colour, alpha=255):
        if 0 <= x < S and 0 <= y < S:
            self.image.putpixel((x, y), colour + (alpha,))

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


def spout(variant):
    c = Cell()
    shapes = [
        [
            "....wwwwwww.....",
            "...wsSSSSSsw....",
            "...wiIIIIIiw....",
            "....iIhhIIi.....",
            "....iI..Ii......",
            "....i....i......",
            ".........i......",
        ],
        [
            ".....wwwwwwww...",
            "....wsSSSSSSsw..",
            "....wiIIIIIIiw..",
            ".....iIIhhIIi...",
            "......iI..Ii....",
            "......i....I....",
            "...........i....",
        ],
    ]
    c.pattern(0, 3, shapes[variant], {"w": WALL0, "s": WALL2, "S": WALL3, "i": ICE1, "I": ICE3, "h": ICE4})
    return c


def fall(frame):
    c = Cell()
    for y in range(S):
        for x in range(5, 11):
            colour = ICE2
            if x in (5, 10):
                colour = ICE0
            elif x == 6:
                colour = ICE3
            elif (x + y) % 5 == 0:
                colour = ICE1
            c.put(x, y, colour, 235)
        # The glint sliding down.
    glint = (frame * 4) % S
    for y in range(glint, min(S, glint + 2)):
        c.put(7, y, SHINE)
        c.put(8, y, ICE4)
    return c


def mound(frame):
    c = Cell()
    c.pattern(1, 2, [
        "....IIIIII....",
        "..IIhhhhhhII..",
        ".IhhIIIIIIhhI.",
        "IhIIiiiiiiIIhI",
        "IIiiiiiiiiiiII",
        ".iiiiiiiiiiii.",
        "...iiiiiiii...",
    ], {"I": ICE3, "h": ICE4, "i": ICE1})
    sparkle = [(4, 3), (9, 4), (12, 6), (6, 6)][frame]
    c.put(sparkle[0], sparkle[1], SHINE)
    c.put(sparkle[0] + 1, sparkle[1], ICE4)
    return c


def fish():
    c = Cell()
    c.pattern(2, 5, [
        "....ddddd...",
        "d.ddffffFdd.",
        "dddfffFFffkd",
        "d.ddffffFdd.",
        "....ddddd...",
    ], {"d": FISH_D, "f": FISH, "F": (170, 186, 192), "k": (20, 20, 20)})
    return c


def hand():
    c = Cell()
    c.pattern(3, 3, [
        "..b.b.b...",
        "..b.b.b.b.",
        "..b.b.b.b.",
        "..bbbbbbb.",
        "b.bBBBBBb.",
        ".bbBBBBb..",
        "...bBBb...",
        "....bb....",
        "....BB....",
    ], {"b": BONE, "B": BONE_D})
    return c


def sword():
    c = Cell()
    c.pattern(1, 6, [
        "..........hh..",
        "ssssssssssghg.",
        "SSSSSSSSSSghhh",
        "..........hh..",
    ], {"s": STEEL, "S": STEEL_D, "h": HILT, "g": GOLD})
    return c


def coins():
    c = Cell()
    for x, y in ((4, 6), (8, 5), (6, 9), (10, 8), (3, 10)):
        c.pattern(x, y, ["gG", "Gg"], {"g": GOLD, "G": GOLD_D})
    return c


def crack(variant):
    c = Cell()
    lines = [
        [(2, 3), (3, 4), (4, 5), (5, 5), (6, 6), (7, 7), (8, 7), (9, 8), (10, 9), (11, 10), (12, 12), (7, 8), (6, 10), (6, 11), (10, 8), (11, 7)],
        [(12, 2), (11, 3), (10, 4), (10, 5), (9, 6), (8, 7), (7, 8), (6, 8), (5, 9), (4, 11), (3, 12), (8, 8), (9, 10), (9, 11), (7, 6), (6, 5)],
    ][variant]
    for x, y in lines:
        c.put(x, y, ICE0)
        c.put(x + 1, y, ICE4, 200)
    return c


def drift():
    c = Cell()
    c.pattern(0, 0, [
        "SSSSSSSSSSSSSSSS",
        "sSSSSsSSSSSsSSSS",
        ".sSSSSSSsSSSSSs.",
        "..ssSSSSSSSSss..",
        "....sssSSsss....",
        "......ssss......",
    ], {"S": SNOW, "s": SNOW_D})
    return c


def main():
    sheet([[spout(0), spout(1)]]).save(OUT / "ice_spout.tga")
    sheet([[fall(f) for f in range(4)], [mound(f) for f in range(4)]]).save(OUT / "ice_fall.tga")
    sheet([[fish(), hand(), sword(), coins(), crack(0), crack(1), drift()]]).save(OUT / "ice_debris.tga")


if __name__ == "__main__":
    main()
