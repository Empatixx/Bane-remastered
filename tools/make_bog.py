"""The moss floor's counterpart to the sewer's waterworks (see make_sewer.py), in the same layout
so the game draws all of them alike (16x16 cells, shown four times over):

- bog_seep.tga: two seeps in a wall's face, a crack between gnarled roots, in the wall's own
  stone colours, murky water welling out at the bottom.
- bog_trickle.tga: four frames of murky water trickling down the wall (top row) and four of it
  dripping into the bog at the wall's foot (bottom row).
- bog_debris.tga: what floats on the bog: two lily pads, a fallen leaf, a frog on a lily pad
  (sitting, then leaping), two empty cells, and the ring of stirred mud round anything afloat.
- bog_log.tga: a mossy fallen log over the bog, across the screen (top row: the end on the left
  bank, the middle, the end on the right bank) and down it (bottom row), cut ends on the banks'
  edges.

Palettes from the moss tileset and the bog surface.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

WALL0 = (22, 24, 16)
WALL1 = (54, 56, 50)
WALL2 = (82, 84, 72)
WALL3 = (116, 116, 98)
WALL4 = (150, 148, 124)
ROOT0 = (34, 24, 16)
ROOT1 = (66, 48, 30)
ROOT2 = (98, 74, 46)
ROOT3 = (130, 102, 64)
MUD_D = (52, 42, 28)
MUD = (86, 70, 42)
MUD_L = (122, 108, 66)
MUD_H = (160, 148, 100)
MOSS_D = (52, 82, 34)
MOSS = (84, 112, 52)
MOSS_L = (120, 152, 70)
LILY_D = (40, 80, 40)
LILY = (70, 128, 56)
LILY_L = (112, 170, 80)
PETAL = (232, 196, 214)
PETAL_D = (196, 140, 170)
LEAF = (170, 120, 50)
LEAF_D = (120, 78, 34)
FROG = (96, 150, 60)
FROG_L = (150, 196, 90)
FROG_D = (46, 80, 30)
EYE = (20, 20, 16)
BARK0 = (38, 28, 20)
BARK1 = (70, 52, 34)
BARK2 = (100, 76, 48)
BARK3 = (132, 104, 66)
WOOD = (176, 146, 96)
WOOD_D = (140, 110, 70)


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


SEEP_PALETTE = {"w": WALL0, "s": WALL2, "S": WALL3, "r": ROOT1, "R": ROOT2, "q": ROOT3, "k": ROOT0,
                "m": MUD, "M": MUD_L, "g": MOSS, "G": MOSS_L}


def seep(variant):
    c = Cell()
    shapes = [
        [
            "..r......r......",
            "..Rr....rR......",
            "...Rr..rRq......",
            "...qRrrRq.......",
            "....kwwwk..g....",
            "...rkmMmkr.Gg...",
            "..rRkmMMkRr.....",
            ".rR.kmmMk.Rr....",
            ".R..wkmmkw.R....",
            "..g..kmmk.......",
            ".gG...mm........",
            "......mM........",
        ],
        [
            "......r....r....",
            "......Rr..rR....",
            ".....qRrrRq.....",
            "......kwwwk.....",
            "..g..rkmMmkr....",
            ".Gg.rRkmMMkRr...",
            "...rR.kmmMk.Rr..",
            "...R..wkmmkw.R..",
            "........kmmk..g.",
            "........mm...gG.",
            "........mM......",
            "................",
        ],
    ]
    c.pattern(0, 2, shapes[variant], SEEP_PALETTE)
    return c


def trickle(frame):
    c = Cell()
    for y in range(S):
        band = (y - frame * 4) % 8
        for x in range(6, 10):
            colour = MUD
            if x in (6, 9):
                colour = MUD_D
            elif band < 2:
                colour = MUD_H
            elif band in (2, 3):
                colour = MUD_L
            c.put(x, y, colour, 235)
    return c


def drip(frame):
    c = Cell()
    shapes = [
        ["...h..h...", ".mmMMMMmm.", "mMMHHHHMMm", ".mmMMMMmm."],
        ["..h....h..", "mmMMMMMMmm", "MMHHHHHHMM", "mmMMMMMMmm"],
        [".h..h..h..", ".mmMMMMmm.", "mMMHHHHMMm", ".mmMMMMmm."],
        ["h..h..h..h", "mmMMMMMMmm", "MMHHHHHHMM", "mmMMMMMMmm"],
    ]
    c.pattern(3, 2, shapes[frame], {"h": MUD_H + (220,), "H": MUD_H + (200,), "M": MUD_L + (210,), "m": MUD + (200,)})
    return c


LILY_PALETTE = {"d": LILY_D, "l": LILY, "L": LILY_L, "p": PETAL, "P": PETAL_D, "y": (240, 220, 110)}


def lily(variant):
    c = Cell()
    shapes = [
        [
            "...dddddd...",
            ".ddlLLLlldd.",
            "dlLLLlllllld",
            "dlLlll.lllld",
            "dllll..lllLd",
            ".dlll.lllld.",
            "..dddddddd..",
        ],
        [
            "...dddddd...",
            ".ddlLpplldd.",
            "dlLLpypPlld.",
            "dlLllPPllld.",
            "dlll...lllLd",
            ".ddll.lldd..",
            "...ddddd....",
        ],
    ]
    c.pattern(2, 4, shapes[variant], LILY_PALETTE)
    return c


def leaf():
    c = Cell()
    c.pattern(3, 5, [
        "......ddd.",
        "....dllLd.",
        "..dllLLld.",
        ".dlLLlld..",
        "dlLlldd...",
        ".ddd......",
    ], {"d": LEAF_D, "l": LEAF, "L": (210, 160, 80)})
    return c


def frog(leaping):
    c = lily(0)
    if leaping:
        c.pattern(4, 0, [
            "..fFFf..",
            ".fFeeFf.",
            "fFFFFFFf",
            "f.fFFf.f",
            "..f..f..",
        ], {"f": FROG_D, "F": FROG, "e": EYE})
    else:
        c.pattern(4, 3, [
            ".fe..ef.",
            "fFFFFFFf",
            "fFLLLLFf",
            ".fFFFFf.",
            "ff....ff",
        ], {"f": FROG_D, "F": FROG, "L": FROG_L, "e": EYE})
    return c


def ring():
    c = Cell()
    c.pattern(0, 9, [
        "....mmmmmmmm....",
        "..mm........mm..",
        ".m............m.",
        "..mm........mm..",
        "....mmmmmmmm....",
    ], {"m": MUD_H + (150,)})
    return c


def log(part, across):
    """A mossy fallen log; its ends, cut on the banks' edges, show the rings of the wood."""
    c = Cell()
    span = range(0, S) if part == 1 else (range(10, S) if part == 0 else range(0, 6))
    for at in span:
        for side in range(2, 14):
            if side in (2, 13):
                colour = BARK0
            elif side < 5:
                colour = MOSS_L if (at + side) % 3 else MOSS
            elif side < 7:
                colour = BARK3
            elif side < 11:
                colour = BARK2 if (at * 3 + side) % 7 else BARK1
            else:
                colour = BARK1
            x, y = (at, side) if across else (side, at)
            c.put(x, y, colour)
        x, y = (at, 14) if across else (14, at)
        c.put(x, y, (0, 0, 0), 90)
    if part != 1:
        edge = 10 if part == 0 else 5
        for side in range(3, 13):
            x, y = (edge, side) if across else (side, edge)
            c.put(x, y, WOOD_D if side in (3, 12) else WOOD)
        mid = 8
        x, y = (edge, mid) if across else (mid, edge)
        c.put(x, y, WOOD_D)
    return c


def main():
    sheet([[seep(0), seep(1)]]).save(OUT / "bog_seep.tga")
    sheet([[trickle(f) for f in range(4)], [drip(f) for f in range(4)]]).save(OUT / "bog_trickle.tga")
    sheet([[lily(0), lily(1), leaf(), frog(False), frog(True), Cell(), ring()]]).save(OUT / "bog_debris.tga")
    sheet([[log(p, True) for p in range(3)], [log(p, False) for p in range(3)]]).save(OUT / "bog_log.tga")


if __name__ == "__main__":
    main()
