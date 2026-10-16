"""What lies about the spider floor, drawn at the tiles' pixel scale (16x16 cells, shown four
times over):

- spider_eggs.tga: two clutches of eggs stuck in silk against a wall (one per row), each in two
  frames as the eggs swell and settle, something stirring inside.
- spider_wrapped.tga: two victims wrapped in webs on the floor (one per row), a skeleton and a
  heap of bones with a helmet.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

SILK = (222, 220, 196)
SILK_D = (176, 172, 150)
SILK_S = (126, 122, 106)
EGG = (236, 232, 206)
EGG_D = (196, 188, 160)
EGG_H = (255, 255, 240)
VEIN = (150, 170, 110)
BONE = (214, 206, 186)
BONE_D = (150, 142, 124)
DARK = (40, 30, 30)
IRON = (110, 106, 122)
IRON_D = (60, 58, 70)


def cell(rows, palette):
    image = Image.new("RGBA", (S, S), CLEAR)
    top = S - len(rows)
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key in palette:
                colour = palette[key]
                image.putpixel((x, top + y), colour if len(colour) == 4 else colour + (255,))
    return image


EGGS = {"e": EGG, "E": EGG_D, "h": EGG_H, "v": VEIN, "s": SILK + (200,), "S": SILK_S + (180,)}

CLUTCHES = [
    [
        "..s..........s..",
        "...s.eee....s...",
        "....ehheEeee....",
        "..eeehEeehheE...",
        ".ehheeEehEeeEe..",
        ".eEeeEeeeeEeEE..",
        "..EEeheEEheEE...",
        "...SEEEESEEES...",
        "..S..SSSS...S...",
    ],
    [
        "s.............s.",
        ".s...eee.eee.s..",
        "..seheEehheEs...",
        "...eEeeEeEeeE...",
        "..ehheEehheeEe..",
        "..eEeEeeEeEeE...",
        "...EEvEEEEvE....",
        "..S.SSSSSSS.S...",
    ],
]


def clutch(variant, swollen):
    rows = CLUTCHES[variant]
    if swollen:
        # The eggs swell a little, a vein showing through one of them.
        rows = [row.replace("eE", "ee").replace("hE", "hv", 1) for row in rows]
    return cell(rows, EGGS)


WRAPPED = {"s": SILK, "S": SILK_D, "t": SILK_S, "b": BONE, "B": BONE_D, "k": DARK, "i": IRON, "I": IRON_D}

VICTIMS = [
    [
        "...sSs..........",
        "..sbkbSs........",
        "..SbbbtSsSs.....",
        "...sSttSSSbSs...",
        "..sStSSttSSbSs..",
        ".SsSSttSStSSbSs.",
        "..tSsSSttSStsSb.",
        "...tsSSSStSs.b..",
        ".....tsSSs......",
    ],
    [
        "......iiii......",
        ".....iIIIIi.....",
        "....siikkiis....",
        "...sSbStSbStSs..",
        "..sbSSttSSbSSs..",
        ".sSbBtSSbBStSbs.",
        "..tSsbSStSbsSt..",
        "...tsSSbSSsSt...",
        ".....ttSSt......",
    ],
]


def main():
    eggs = Image.new("RGBA", (2 * S, 2 * S), CLEAR)
    for row in range(2):
        for frame in range(2):
            eggs.paste(clutch(row, frame == 1), (frame * S, row * S))
    eggs.save(OUT / "spider_eggs.tga")
    wrapped = Image.new("RGBA", (S, 2 * S), CLEAR)
    for row in range(2):
        wrapped.paste(cell(VICTIMS[row], WRAPPED), (0, row * S))
    wrapped.save(OUT / "spider_wrapped.tga")


if __name__ == "__main__":
    main()
