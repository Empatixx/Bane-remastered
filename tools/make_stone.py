"""What the stone floor, a prison and torture chamber, gets besides its traps, drawn at the
tiles' pixel scale (shown four times over):

- shackles.tga: iron shackles on chains fixed to a wall's face (one per row, 16x16): empty, and
  with a skeleton hanging in them.
- pendulum.tga: the blade of a swinging pendulum, a crescent of steel on an iron collar (24x16,
  top left), one link of the chain it hangs on (8x8, below it), and the iron strap clamped round
  the planks it hangs from with an eye for the chain (16x24, on the right, lined up with the
  planks' own 16x24 piece).
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
CLEAR = (0, 0, 0, 0)

IRON = {"d": (34, 30, 40), "m": (78, 74, 90), "l": (112, 108, 124), "h": (146, 140, 160), "u": (112, 66, 48)}
BONE = {"b": (214, 206, 186), "B": (150, 142, 124), "k": (40, 30, 30)}
STEEL = {"s": (150, 156, 172), "S": (84, 88, 104), "e": (226, 232, 246), "d": (34, 30, 40), "m": (78, 74, 90),
         "l": (112, 108, 124), "u": (112, 66, 48)}


def image(rows, palette):
    out = Image.new("RGBA", (len(rows[0]), len(rows)), CLEAR)
    for y, row in enumerate(rows):
        assert len(row) == len(rows[0]), (row, len(row))
        for x, key in enumerate(row):
            if key in palette:
                colour = palette[key]
                out.putpixel((x, y), colour if len(colour) == 4 else colour + (255,))
    return out


SHACKLES = [
    # Two chains from rings in the wall, the shackles hanging open at their ends.
    [
        "................",
        "..dd........dd..",
        ".dmhd......dmhd.",
        "..dd........dd..",
        "..m..........m..",
        "..l..........l..",
        "..m..........m..",
        "..l..........l..",
        "..m..........m..",
        ".dld........dld.",
        ".d.d........d.d.",
        ".dhd........dhd.",
        "..d..........d..",
        "................",
        "................",
        "................",
    ],
    # A skeleton hung in them by its wrists, its head sunk on its chest.
    [
        "................",
        "..dd........dd..",
        ".dmhd......dmhd.",
        "..dd........dd..",
        "..m..........m..",
        "..l..........l..",
        "..dhb......bhd..",
        "...bB.bbb..Bb...",
        "....bBbkbBbB....",
        "......bbb.......",
        ".....bBbBb......",
        "......bbb.......",
        ".....b.B.b......",
        "......bbb.......",
        ".....b...b......",
        ".....B...B......",
    ],
]

BLADE = [
    "..........dd............",
    ".........dlmd...........",
    ".........dmmd...........",
    "...SS....dlmd....SS.....",
    "..Ssss..ddmmdd..ssssS...",
    ".Sssssssdlmmldsssssss...",
    "Ssssssssdmmmmdssssssss..",
    "Sseeeeeeedmmdeeeeeeess..",
    ".Ssssssssddddsssssss....",
    "..SSsssssssssssssSS.....",
    "....SSSssssssSSS........",
    ".......SSSSSS...........",
    "........................",
    "........................",
    "........................",
    "........................",
]

MOUNT = [
    "................",
    "................",
    "................",
    "................",
    "................",
    ".....dddddd.....",
    ".....dmeemd.....",
    ".....dmllmd.....",
    ".....dmeemd.....",
    ".....dmllmd.....",
    ".....dmllmd.....",
    ".....dddddd.....",
    ".....dmllmd.....",
    ".....dddddd.....",
    ".....dmeemd.....",
    ".....dmllmd.....",
    ".....dmeemd.....",
    ".....dmllmd.....",
    ".....dddddd.....",
    "......dmmd......",
    ".....dm..md.....",
    ".....dl..ld.....",
    "......dmmd......",
    "................",
]

LINK = [
    "..dddd..",
    ".dmllmd.",
    "dm....md",
    "dl....ld",
    "dl....ld",
    "dm....md",
    ".dmllmd.",
    "..dddd..",
]


def main():
    shackles = Image.new("RGBA", (16, 32), CLEAR)
    for row, rows in enumerate(SHACKLES):
        shackles.paste(image(rows, {**IRON, **BONE}), (0, row * 16))
    shackles.save(OUT / "shackles.tga")
    pendulum = Image.new("RGBA", (40, 24), CLEAR)
    pendulum.paste(image(BLADE, STEEL), (0, 0))
    pendulum.paste(image(LINK, STEEL), (0, 16))
    pendulum.paste(image(MOUNT, STEEL), (24, 0))
    pendulum.save(OUT / "pendulum.tga")


if __name__ == "__main__":
    main()
