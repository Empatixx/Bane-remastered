"""Things that hang from the ceiling on a chain or a thread: a candle chandelier, a cage with a
skeleton, a spider's cocoon, a lantern and a vine; and for the lava floor a brazier of glowing
coals, a crucible of molten metal, a charred skeleton hung on hooks and an obsidian stalactite
with glowing veins; and for the moss floor roots hung with glowing mushrooms, a wasps' nest on
a branch, a cage overgrown with moss and flowers, and a swarm of fireflies (no chain at all); and for
the ice floor a chandelier of icicles, a skeleton frozen in a block of ice and a great icicle.

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

EMBER = {"y": (255, 236, 150), "o": (255, 160, 50), "r": (214, 72, 26), "e": (120, 36, 20), "c": (40, 30, 30)}

BRAZIER = [
    [
        "................",
        "................",
        "......y.........",
        ".....yo..y......",
        "....yooy.oy.....",
        "...yoorooroy....",
        "..dmrooyyoormd..",
        "..dhhhhhhhhhhd..",
        "..dmeccrccecmd..",
        "...dmcecrcemd...",
        "...dm.d..d.md...",
        "....dm.dd.md....",
        ".....dmmmmd.....",
        "......dmmd......",
        ".......dd.......",
        ".......m........",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
    [
        "................",
        ".......y........",
        "....y..oy.......",
        "....oy.yo.y.....",
        "...yooyooyo.....",
        "...yorooyoroy...",
        "..dmrooyyorrmd..",
        "..dhhhhhhhhhhd..",
        "..dmcecrccecmd..",
        "...dmcrcecemd...",
        "...dm.d..d.md...",
        "....dm.dd.md....",
        ".....dmmmmd.....",
        "......dmmd......",
        ".......dd.......",
        ".......m........",
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

CRUCIBLE = [
    [
        ".......dd.......",
        "......dmmd......",
        "....dd.dd.dd....",
        "...d.......d....",
        "..d.........d...",
        ".dhhhhhhhhhhhd..",
        "dmmyyyyooyyymmd.",
        "dmmoyoooooyommd.",
        ".dmmmmmmmmmmmd..",
        ".dhmmmmmmmmmld..",
        "..dmmmmmmmmmd...",
        "..dmmmmmmmmmd...",
        "...dmmmmmmmd....",
        "....ddmmmdd.....",
        "......dod.......",
        ".......y........",
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
CRUCIBLE.append([row.replace("yyyy", "yoyy").replace("oooo", "yooo") for row in CRUCIBLE[0][:14]] + [
    "......ddd.......",
    "................",
] + CRUCIBLE[0][16:])

HOOKS = [
    [
        "...md......md...",
        "...md......md...",
        "...md......md...",
        "...hd......hd...",
        "...d.h....h.d...",
        "....bbbbbbbb....",
        "....bkbbbbkb....",
        ".....bkbbkb.....",
        "......bbbb......",
        "....bb.BB.bb....",
        "...b..bBBb..b...",
        "..e...bBBb...e..",
        "......bBBb......",
        ".......BB.......",
        "......B..B......",
        ".....B....B.....",
        ".....r....b.....",
        ".....b....o.....",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
    [
        "...md......md...",
        "...md......md...",
        "...md......md...",
        "...hd......hd...",
        "...d.h....h.d...",
        "....bbbbbbbb....",
        "....bkbbbbkb....",
        ".....bkbbkb.....",
        "......bbbb......",
        "....bb.BB.bb....",
        "...b..bBBb..b...",
        "..o...bBBb...e..",
        "......bBBb......",
        ".......BB.......",
        "......B..B......",
        ".....B....B.....",
        ".....o....b.....",
        ".....b....r.....",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
]
HOOK_CHAINS = [
    "...lh......lh...",
    "..l..m....l..m..",
    "..m..d....m..d..",
    "...md......md...",
] * 6
CHARRED = {"b": (150, 136, 120), "B": (92, 80, 72), "k": (24, 16, 16)}

ROCK = {"d": (14, 10, 12), "m": (34, 26, 28), "h": (70, 58, 60), "v": (214, 72, 26), "V": (255, 150, 50)}
STALACTITE = [
    [
        "....dmmmmmmd....",
        "....dmhmmvmd....",
        "....dmmmvmmd....",
        ".....dmmvmmd....",
        ".....dmhmvd.....",
        ".....dmmmvd.....",
        "......dmvmd.....",
        "......dmvd......",
        "......dhvd......",
        "......dmvd......",
        ".......dvd......",
        ".......dd.......",
        ".......d........",
        "................",
        "................",
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
STALACTITE.append([row.replace("v", "V") for row in STALACTITE[0]])
ROCK_ROOT = [
    "....dmmhmmmd....",
    "....dmmmmvmd....",
    "....dmhmmmmd....",
    "....dmmvmmhd....",
] * 6

ROOTS = {"r": (66, 48, 30), "R": (98, 74, 46), "k": (34, 24, 16), "g": (120, 230, 150), "G": (200, 255, 210), "c": (70, 140, 96)}
GLOW_ROOTS = [
    [
        "......rk........",
        ".....rRk........",
        "....rRk.rk......",
        "...rRk..Rrk.....",
        "...Rk...kRr.....",
        "..gGg....kR.....",
        "..cgc...gGg.....",
        "...k....cgc.....",
        "...r.....k......",
        "..rk.....rk.....",
        ".gGg......R.....",
        ".cgc.....gGg....",
        "..k......cgc....",
        "..........k.....",
        "................",
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
GLOW_ROOTS.append([row.replace("G", "W").replace("g", "G").replace("W", "g") for row in GLOW_ROOTS[0]])
ROOT_STRAND = [
    ".......rk.......",
    "......rRk.......",
    ".......Rk.......",
    ".......kR.......",
] * 6

NEST = {"b": (176, 150, 100), "B": (136, 112, 74), "d": (88, 70, 46), "h": (20, 16, 12), "r": (66, 48, 30), "R": (98, 74, 46)}
HIVE = [
    [
        ".......rR.......",
        "......dbbd......",
        ".....dbBBbd.....",
        "....dbbbbbbd....",
        "....BBBBBBBB....",
        "...dbbbbbbbbd...",
        "...dbbbhbbbbd...",
        "...BBBBhBBBBB...",
        "...dbbbbbbbbd...",
        "....dbbbbbbd....",
        "....BBBBBBBB....",
        ".....dbbbbd.....",
        "......dBBd......",
        ".......dd.......",
        "................",
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
HIVE.append(HIVE[0])
BRANCH = [
    ".......rR.......",
    ".......Rr.......",
    "......rRr.......",
    ".......rR.......",
] * 6

OVERGROWN = {**IRON, **BONE, "g": (84, 112, 52), "G": (120, 152, 70), "p": (232, 196, 214), "y": (240, 220, 110)}
MOSS_CAGE = [
    [
        ".......dd.......",
        "......dmmd......",
        ".......dd.......",
        ".....dGgGmd.....",
        "...dGgGGgGgd....",
        "..dhGmmgmmhhd...",
        "..dm.d..d.gGd...",
        "..dg.d..d..mg...",
        "..dm.dbbbd.md...",
        "..Gg.bkbkb.mG...",
        "..dm.dbbbd.gd...",
        "..dm.d.B.d.md...",
        "..gG.dbBbd.md...",
        "..dm.bdBdb.Gg...",
        "..dm.d.B.d.md...",
        "..gm.dB.Bd.mg...",
        "..dGpyg.gpgGd...",
        "..dhGhhhhhGhd...",
        "..dmmGgmmgmmd...",
        "...dddgddddd....",
        "......g.........",
        "................",
        "................",
        "................",
    ],
]
MOSS_CAGE.append(MOSS_CAGE[0])

FIREFLY = {"y": (230, 255, 140), "Y": (255, 255, 220), "o": (150, 200, 80, 150)}
FIREFLIES = [
    [
        "................",
        "................",
        ".....o..........",
        "....oYo.....o...",
        ".....o.....oyo..",
        "..o.........o...",
        ".oyo...o........",
        "..o...oYo.......",
        ".......o....o...",
        "...........oYo..",
        "....o.......o...",
        "...oyo..........",
        "....o...........",
        "................",
        "................",
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
    [
        "................",
        "................",
        ".....o..........",
        "....oyo.....o...",
        ".....o.....oYo..",
        "..o.........o...",
        ".oYo...o........",
        "..o...oyo.......",
        ".......o....o...",
        "...........oyo..",
        "....o.......o...",
        "...oYo..........",
        "....o...........",
        "................",
        "................",
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
NOTHING = ["................"] * 24

FROST = {"i": (118, 172, 214), "I": (190, 226, 246), "h": (226, 246, 255), "w": (255, 255, 255), "e": (90, 140, 190)}
ICE_CHANDELIER = [
    [
        ".......dd.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        ".......md.......",
        "..h....md....h..",
        ".hIh...md...hIh.",
        ".IiI...md...IiI.",
        "dddddddmmddddddd",
        "mhhhhhhhhhhhhhhm",
        "dmmmmmmmmmmmmmmd",
        ".IIiIIiIIiIIiII.",
        ".IiiIiiIiiIiiIi.",
        "..i..i.Ii..i..i.",
        "..i....i...i....",
        ".......i........",
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
ICE_CHANDELIER.append([row.replace("hIh", "wIw") for row in ICE_CHANDELIER[0][:8]] + [
    row.replace("IIi", "Iwi", 1) for row in ICE_CHANDELIER[0][8:]
])

FROZEN_BLOCK = [
    [
        ".......dd.......",
        "......dmmd......",
        "...eeeedde......",
        "..eIIIIIIIIe....",
        ".ehIIIIIIIIIe...",
        ".eIIbbbIIIIIe...",
        ".eIIbkbkbIIIe...",
        ".eIIIbbbIIIIe...",
        ".eIIIIBIIIIhe...",
        ".eIIbbBbbIIIe...",
        ".eIIIIBIIIIIe...",
        ".eIIIbIbIIIIe...",
        ".eIIbIIIbIIIe...",
        ".ehIIIIIIIIIe...",
        "..eiiiiiiiiie...",
        "...eeeeeeeee....",
        "....i.....i.....",
        "....i...........",
        "................",
        "................",
        "................",
        "................",
        "................",
        "................",
    ],
]
FROZEN_BLOCK.append(FROZEN_BLOCK[0])

BIG_ICICLE = [
    [
        "...eIIIIIIIe....",
        "...eIhIIIIIe....",
        "....eIhIIIe.....",
        "....eIhIIie.....",
        ".....eIhIe......",
        ".....eIhIe......",
        ".....eIhie......",
        "......eIe.......",
        "......eIe.......",
        "......ehe.......",
        ".......e........",
        ".......i........",
        "................",
        "................",
        "................",
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
BIG_ICICLE.append([row.replace("eIhI", "eIwI") for row in BIG_ICICLE[0]])
ICE_ROOT = [
    "...eIIhIIIIe....",
    "...eIIIIhIIe....",
    "...eIhIIIIIe....",
    "...eIIIIIhIe....",
] * 6

KINDS = [
    (CHANDELIER, CHAIN, {**IRON, **FLAME}),
    (CAGE, CHAIN, {**IRON, **BONE}),
    (COCOON, THREAD, {**SILK, "T": (150, 146, 126)}),
    (LANTERN, CHAIN, {**IRON, **GLASS, **FLAME}),
    (VINE, VINE_STRAND, LEAF),
    (BRAZIER, CHAIN, {**IRON, **EMBER}),
    (CRUCIBLE, CHAIN, {**IRON, **EMBER}),
    (HOOKS, HOOK_CHAINS, {**IRON, **CHARRED, "e": (120, 36, 20), "o": (255, 160, 50), "r": (214, 72, 26)}),
    (STALACTITE, ROCK_ROOT, ROCK),
    (GLOW_ROOTS, ROOT_STRAND, ROOTS),
    (HIVE, BRANCH, NEST),
    (MOSS_CAGE, CHAIN, OVERGROWN),
    (FIREFLIES, NOTHING, FIREFLY),
    (ICE_CHANDELIER, CHAIN, {**IRON, **FROST}),
    (FROZEN_BLOCK, CHAIN, {**IRON, **BONE, **FROST}),
    (BIG_ICICLE, ICE_ROOT, FROST),
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
