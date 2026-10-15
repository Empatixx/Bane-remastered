"""Things that hang from the ceiling on a chain or a thread: a candle chandelier, a cage with a
skeleton, a spider's cocoon, a lantern and a vine.

hanging.tga has one row per kind in that order and four 16x24 cells per row: the thing, the
thing with its flames or glow flickering, a piece of its chain (or thread, or vine) that repeats
upwards, and the topmost piece fading out into the dark. Hand-drawn at the tiles' pixel scale.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
W, H = 16, 24
CLEAR = (0, 0, 0, 0)

IRON = {"d": (34, 30, 40), "m": (78, 74, 90), "l": (112, 108, 124), "h": (146, 140, 160)}
FLAME = {"y": (255, 236, 150), "o": (255, 168, 60), "r": (220, 92, 34), "w": (232, 222, 196), "c": (184, 170, 140)}
BONE = {"b": (214, 206, 186), "B": (150, 142, 124), "k": (40, 30, 30)}
SILK = {"s": (222, 220, 196), "S": (176, 172, 150), "t": (126, 122, 106), "g": (110, 130, 70)}
GLASS = {"G": (255, 214, 120), "g": (226, 150, 60), "a": (120, 80, 40)}


def cell(rows, palette):
    image = Image.new("RGBA", (W, H), CLEAR)
    for y, row in enumerate(rows):
        assert len(row) == W, (row, len(row))
        for x, key in enumerate(row):
            if key in palette:
                colour = palette[key]
                image.putpixel((x, y), colour if len(colour) == 4 else colour + (255,))
    return image


CHANDELIER = [
    [
        ".......dd.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        "..y....md....y..",
        ".yoy...md...yoy.",
        "..o.y..md..y.o..",
        "..w.oy.md.yo.w..",
        "..w..w.md.w..w..",
        "..c..w.md.w..c..",
        "dddddcdmmdcddddd",
        "mhhhhhhhhhhhhhhm",
        "dmmmmmmmmmmmmmmd",
        ".ddddddmmdddddd.",
        ".......dd.......",
        ".......m........",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
    [
        ".......dd.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        "...y...md...y...",
        "..yoy..md..yoy..",
        "..o.y..md.oy.o..",
        "..w.oy.md.y..w..",
        "..w..w.md.w..w..",
        "..c..w.md.w..c..",
        "dddddcdmmdcddddd",
        "mhhhhhhhhhhhhhhm",
        "dmmmmmmmmmmmmmmd",
        ".ddddddmmdddddd.",
        ".......dd.......",
        ".......m........",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
]

CAGE = [
    [
        ".......dd.......",
        "......dmmd......",
        ".......dd.......",
        ".....dmmmmd.....",
        "...dmhhhhhhmd...",
        "..dhhmmmmmmhhd..",
        "..dm.d..d..md...",
        "..dm.d..d..md...",
        "..dm.dbbbd.md...",
        "..dm.bkbkb.md...",
        "..dm.dbbbd.md...",
        "..dm.d.B.d.md...",
        "..dm.dbBbd.md...",
        "..dm.bdBdb.md...",
        "..dm.d.B.d.md...",
        "..dm.dB.Bd.md...",
        "..dm.Bd.dB.md...",
        "..dhhhhhhhhhhd..",
        "..dmmmmmmmmmmd..",
        "...dddddddddd...",
        "................",
        "................",
        "................",
        "................",
    ],
]
CAGE.append(CAGE[0])

COCOON = [
    [
        ".......t........",
        ".......S........",
        ".......t........",
        "......sSs.......",
        ".....sssSs......",
        "....sSsssSt.....",
        "....tsSssss.....",
        "...sssssSsst....",
        "...sSttsssss....",
        "...ssssttsSs....",
        "...ssSssssts....",
        "...tssssSsss....",
        "...ssttssssS....",
        "...sssssttss....",
        "....sSssssst....",
        "....tsssSss.....",
        ".....ssttss.....",
        ".....tsssS......",
        "......ssst......",
        ".......tS.......",
        "........t.......",
        "................",
        "................",
        "................",
    ],
]
COCOON.append([row.replace("t", "T") for row in COCOON[0]])

LANTERN = [
    [
        ".......dd.......",
        "......d..d......",
        ".......dd.......",
        "......dmmd......",
        ".....dhhhhd.....",
        "....dmmmmmmd....",
        "....dmGGGGmd....",
        "....dGGyyGGd....",
        "....dGyooyGd....",
        "....dGyooyGd....",
        "....dGGyyGGd....",
        "....dmGGGGmd....",
        "....dmmmmmmd....",
        ".....dhhhhd.....",
        "......dddd......",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
]
LANTERN.append([row.replace("y", "G").replace("oo", "yy") for row in LANTERN[0]])

CHAIN = [
    ".......lh.......",
    "......l..m......",
    "......m..d......",
    ".......md.......",
] * 6
THREAD = [
    ".......t........",
    ".......S........",
] * 12

LEAF = {"v": (70, 112, 44), "V": (112, 158, 64), "L": (150, 196, 84), "e": (40, 66, 30)}
VINE = [
    [
        ".......ve.......",
        ".......vV.......",
        "......LVe.......",
        ".....LVve.......",
        "......eVv.......",
        ".......vVL......",
        ".......vVLL.....",
        ".......eVe......",
        "......vV........",
        ".....LLVe.......",
        "....LLVve.......",
        "......eVv.......",
        ".......vV.......",
        ".......eVLL.....",
        "........VLLL....",
        "........vVe.....",
        ".......vV.......",
        "......LVe.......",
        ".......e........",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
]
VINE.append([row[1:] + "." for row in VINE[0][:10]] + VINE[0][10:])
VINE_STRAND = [
    ".......ve.......",
    ".......vV.......",
    "......LVe.......",
    ".......eV.......",
    ".......vVL......",
    ".......eV.......",
] * 4

KINDS = [
    (CHANDELIER, CHAIN, {**IRON, **FLAME}),
    (CAGE, CHAIN, {**IRON, **BONE}),
    (COCOON, THREAD, {**SILK, "T": (150, 146, 126)}),
    (LANTERN, CHAIN, {**IRON, **GLASS, **FLAME}),
    (VINE, VINE_STRAND, LEAF),
]


def fading(image):
    """The topmost piece of a chain: fully there at the bottom, gone at the top, with a smooth
    ramp in between so the chain disappears into the dark instead of ending on an edge."""
    faded = image.copy()
    pixels = faded.load()
    for y in range(H):
        ramp = ((y + 0.5) / H) ** 1.4
        for x in range(W):
            r, g, b, a = pixels[x, y]
            pixels[x, y] = (r, g, b, int(a * ramp))
    return faded


def main():
    sheet = Image.new("RGBA", (4 * W, len(KINDS) * H), CLEAR)
    for row, (frames, chain, palette) in enumerate(KINDS):
        pieces = [cell(frames[0], palette), cell(frames[1], palette), cell(chain, palette)]
        pieces.append(fading(pieces[2]))
        for column, image in enumerate(pieces):
            sheet.paste(image, (column * W, row * H))
    sheet.save(OUT / "hanging.tga")


if __name__ == "__main__":
    main()
