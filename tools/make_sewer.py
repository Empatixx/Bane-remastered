"""The sewer's waterworks, drawn at the tiles' pixel scale (16x16 cells, shown four times over):

- sewer_pipe.tga: two round outflow pipes set into a wall's face in the wall's own stone colours,
  one mossy and one with a rusty band, each with the grime it has left on the wall under it.
- sewer_stream.tga: four frames of the water falling out of a pipe down the wall (top row) and
  four of it splashing into the pool at the wall's foot (bottom row).
- sewer_debris.tga: what floats on the water: a plank, a crate, a bottle, an old boot, a rat on
  a plank (two frames of its tail) and the ring of ruffled water round anything afloat.
- sewer_bridge.tga: a plank footbridge over the water, across the screen (top row: the end on
  the left bank, the middle, the end on the right bank) and down it (bottom row: the top end,
  the middle, the bottom end); rope rails along its sides, posts at its ends on the banks'
  edges.

Hand-picked palettes: rotting wood, rusty iron, murky green foam.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

W0 = (30, 22, 16)
W1 = (58, 42, 28)
W2 = (86, 64, 40)
W3 = (116, 90, 56)
W4 = (146, 118, 74)
I0 = (24, 26, 28)
I1 = (52, 56, 58)
I2 = (86, 92, 92)
I3 = (124, 132, 128)
RUST = (112, 62, 36)
RUST_L = (150, 88, 48)
GRIME = (40, 54, 34)
GRIME_L = (66, 84, 50)
WATER_H = (196, 222, 200)
WATER = (128, 170, 140)
WATER_D = (76, 114, 90)
FOAM = (220, 236, 214)
ROPE = (150, 128, 88)
ROPE_D = (104, 86, 56)
GLASS = (70, 130, 80)
GLASS_H = (150, 210, 150)
LEATHER = (60, 40, 30)
LEATHER_L = (92, 64, 44)
RAT = (110, 104, 104)
RAT_L = (150, 144, 140)
RAT_D = (66, 60, 62)
PINK = (210, 150, 150)


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


# --- Pipes -----------------------------------------------------------------------------------


# The sewer wall's own stone colours, so the pipes look built into it.
WALL0 = (16, 26, 22)
WALL1 = (46, 64, 58)
WALL2 = (78, 92, 76)
WALL3 = (104, 116, 92)
WALL4 = (140, 150, 120)
HOLE = (8, 12, 10)


def round_pipe(mossy):
    """A round pipe mouth set into the wall: a stone collar lit from the top, the dark bore, a
    lip of grime under it; one has moss on its collar, the other a rusty band."""
    c = Cell()
    c.rect(5, 13, 10, 15, GRIME, 170)
    c.rect(6, 14, 9, 15, GRIME_L, 120)
    c.pattern(2, 2, [
        "....dddd....",
        "..ddhhhhdd..",
        ".dhhllllhhd.",
        ".dlmkkkkmld.",
        "dlmkkkkkkmld",
        "dlmkkkkkkmld",
        "dlmkkkkkkmld",
        "dlmkkkkkkmld",
        ".dmmkkkkmmd.",
        ".ddmmmmmmdd.",
        "..ddssssdd..",
        "....dddd....",
    ], {"d": WALL1, "h": WALL4, "l": WALL4, "m": WALL3, "k": HOLE, "s": WALL2})
    if mossy:
        for x, y in ((3, 4), (4, 3), (12, 4), (11, 3), (3, 9), (12, 10)):
            c.put(x, y, GRIME_L)
        c.put(4, 4, GRIME)
    else:
        for x in range(4, 12):
            c.put(x, 11, RUST if x % 3 else RUST_L)
    # A trickle of light on the bore's lower lip.
    c.put(7, 9, WALL3)
    c.put(8, 9, WALL3)
    return c


# --- The stream and its splash ---------------------------------------------------------------


def stream(frame):
    c = Cell()
    for y in range(S):
        # Light bands sliding down the falling water.
        band = (y - frame * 4) % 8
        for x in range(5, 11):
            colour = WATER
            if x in (5, 10):
                colour = WATER_D
            elif band < 2 and x in (6, 7, 8):
                colour = WATER_H
            elif band == 4 and x == 9:
                colour = WATER_H
            c.put(x, y, colour, 230)
    return c


def splash(frame):
    c = Cell()
    shapes = [
        ["....f.ff.f....", "..ffWWWWWWff..", ".fWWwwwwwwWWf.", "..fWWwwwwWWf..", "....ffffff...."],
        ["..f..f..f..f..", ".ffWWWWWWWWff.", "fWWwwwwwwwwWWf", ".fWWwwwwwwWWf.", "...ffffffff..."],
        [".f...f..f...f.", "..ffWWWWWWff..", ".fWWwwwwwwWWf.", "..fWWwwwwWWf..", "....ffffff...."],
        ["...f..ff..f...", ".ffWWWWWWWWff.", "fWWwwwwwwwwWWf", ".fWWwwwwwwWWf.", "...ffffffff..."],
    ]
    c.pattern(1, 1, shapes[frame], {
        "f": FOAM + (230,), "W": WATER_H + (220,), "w": WATER + (200,),
    })
    return c


# --- Debris ----------------------------------------------------------------------------------


def plank():
    c = Cell()
    c.pattern(1, 5, [
        "..dddddddddd..",
        ".dhhhhhhhhhhd.",
        "dlllkllllllld.",
        "dmmmmmmmkmmmd.",
        ".ddddddddddd..",
    ], {"d": W0, "h": W4, "l": W3, "m": W2, "k": W1})
    c.put(4, 7, I2)
    c.put(11, 7, I2)
    return c


def crate():
    c = Cell()
    c.pattern(3, 3, [
        "dddddddddd",
        "dhhhhhhhhd",
        "dlkllllkld",
        "dllkllklld",
        "dlllkklllld"[:10],
        "dllkllklld",
        "dlkllllkld",
        "dmmmmmmmmd",
        "dddddddddd",
    ], {"d": W0, "h": W4, "l": W3, "m": W2, "k": W1})
    return c


def bottle():
    c = Cell()
    c.pattern(4, 6, [
        "....ddddd.",
        ".dddgGggdc",
        "dgGGgggggc",
        ".dddggggdc",
        "....ddddd.",
    ], {"d": (30, 60, 36), "g": GLASS, "G": GLASS_H, "c": (150, 120, 80)})
    return c


def boot():
    c = Cell()
    c.pattern(3, 4, [
        "..ddddd...",
        "..dlLld...",
        "..dllld...",
        "..dllldd..",
        ".ddlllLdd.",
        "dlllllllld",
        "dmmmmmmmmd",
        ".dddddddd.",
    ], {"d": (30, 20, 16), "l": LEATHER, "L": LEATHER_L, "m": (40, 28, 22)})
    return c


def rat_raft(frame):
    c = plank()
    tail = [["t.", ".t", ".t"], [".t", "t.", "t."]][frame]
    c.pattern(1, 2, tail, {"t": PINK})
    c.pattern(3, 2, [
        "..dddd...",
        ".dlllld..",
        "dlLlllldk",
        ".dddddde.",
    ], {"d": RAT_D, "l": RAT, "L": RAT_L, "k": (20, 20, 20), "e": PINK})
    return c


def ring():
    c = Cell()
    c.pattern(0, 9, [
        "....ffffffff....",
        "..ff........ff..",
        ".f............f.",
        "..ff........ff..",
        "....ffffffff....",
    ], {"f": FOAM + (150,)})
    return c


# --- The footbridge --------------------------------------------------------------------------


def bridge_across(part):
    """`part` 0 is the end on the left bank, 1 the middle, 2 the end on the right bank. The ends
    reach only the bank's edge, where two posts hold the ropes."""
    c = Cell()
    if part == 1:
        span = range(0, S)
    elif part == 0:
        span = range(10, S)
    else:
        span = range(0, 6)
    for x in span:
        board = x % 4
        if board != 3:
            colour = (W3, W4, W2)[board]
            c.rect(x, 2, x, 13, colour)
            c.put(x, 13, W1)
        c.put(x, 1, ROPE)
        c.put(x, 14, ROPE)
        c.put(x, 15, ROPE_D)
    for x in span:
        if x % 4 == 1:
            c.put(x, 6, W1)
            c.put(x + 1, 9, W1)
    if part != 1:
        x = 10 if part == 0 else 3
        for y0 in (0, 11):
            c.rect(x, y0, x + 2, y0 + 4, W1)
            c.rect(x, y0, x + 2, y0, W4)
            c.put(x + 1, y0 + 1, W3)
    return c


def bridge_down(part):
    """`part` 0 is the top end, 1 the middle, 2 the bottom end; the ends reach the bank's edge."""
    c = Cell()
    if part == 1:
        span = range(0, S)
    elif part == 0:
        span = range(10, S)
    else:
        span = range(0, 6)
    for y in span:
        board = y % 4
        if board != 3:
            colour = (W4, W3, W2)[board]
            c.rect(2, y, 13, y, colour)
            c.put(13, y, W1)
        c.put(1, y, ROPE)
        c.put(14, y, ROPE)
        c.put(15, y, ROPE_D)
    for y in span:
        if y % 4 == 1:
            c.put(6, y, W1)
            c.put(9, y + 1, W1)
    if part != 1:
        y = 10 if part == 0 else 2
        for x0 in (0, 11):
            c.rect(x0, y, x0 + 4, y + 3, W1)
            c.rect(x0, y, x0 + 4, y, W4)
            c.put(x0 + 2, y + 1, W3)
    return c


def main():
    sheet([[round_pipe(True), round_pipe(False)]]).save(OUT / "sewer_pipe.tga")
    sheet([[stream(f) for f in range(4)], [splash(f) for f in range(4)]]).save(OUT / "sewer_stream.tga")
    sheet([[plank(), crate(), bottle(), boot(), rat_raft(0), rat_raft(1), ring()]]).save(OUT / "sewer_debris.tga")
    sheet([[bridge_across(p) for p in range(3)], [bridge_down(p) for p in range(3)]]).save(OUT / "sewer_bridge.tga")


if __name__ == "__main__":
    main()
