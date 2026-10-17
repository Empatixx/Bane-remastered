"""Things that lie across a room from one wall to another at the height of their tops: an iron
grate, wooden beams, a chain, pipes, a stone arch, a thick web, a metal walkway, a liana, a few strands of silk and a
garland of red and gold pennants.

overheads.tga has one row per kind in that order and four 16x24 cells per row: the left end
(meeting the side of the wall), a middle piece that repeats seamlessly, the right end, and a centre piece
(the arch's keystone; for the others it is the plain middle again).

overheads_upright.tga holds the same things running down the screen, from the top of one block
to the top of the block below, seen from above: one row per kind and four 16x16 cells per row,
the upper end, a middle piece that repeats seamlessly downwards, the lower end and a centre piece,
each the horizontal piece turned over its diagonal.

Nothing lies over the walls: each end meets a wall's side, the chain hooked into the eye of a
bolt in a plate fixed to it and the planks sitting in an iron bracket.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
W, H = 16, 24
CLEAR = (0, 0, 0, 0)

IRON = {"d": (34, 30, 40), "m": (78, 74, 90), "l": (110, 106, 122), "h": (140, 134, 154), "r": (178, 172, 188), "u": (112, 66, 48)}
WOOD = {"d": (58, 36, 24), "m": (106, 70, 42), "l": (140, 96, 58), "h": (176, 128, 78), "n": (150, 146, 156), "i": (60, 56, 66)}
PIPE = {"d": (30, 40, 36), "m": (70, 84, 76), "l": (104, 120, 108), "h": (146, 162, 146), "u": (124, 76, 46), "b": (170, 180, 166)}
STONE = {"d": (70, 58, 44), "m": (130, 106, 74), "l": (164, 136, 92), "h": (204, 176, 122), "g": (232, 196, 110), "x": (40, 30, 22)}
SILK = {"s": (220, 220, 196, 225), "f": (160, 160, 140, 170)}


def cell(rows, palette):
    image = Image.new("RGBA", (W, H), CLEAR)
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key in palette:
                colour = palette[key]
                image.putpixel((x, y), colour if len(colour) == 4 else colour + (255,))
    return image


def pad(rows):
    """Centres a shorter picture vertically in the 24-pixel cell."""
    top = (H - len(rows)) // 2
    blank = "." * W
    return [blank] * top + rows + [blank] * (H - len(rows) - top)


def mirror(rows):
    return [row[::-1] for row in rows]


GRATE_MID = pad([
    "hhhhrhhhhhhhrhhh",
    "mmmmmmmmmmmmmmmm",
    "dddddddddddddddd",
    "md.m..m.md.m..m.",
    "md..m.m.md..m.m.",
    "md...mm.md...mm.",
    "md..m.m.md..m.m.",
    "md.m..m.md.m..m.",
    "mdm...m.mdm...m.",
    "hhhhhhhhhhhhhhhh",
    "mmmmmmmmmmmmmmmm",
    "ddddudddddddddud",
])
GRATE_END = pad([
    "..........dddddd",
    "..........dhhhhh",
    "..........dmrmmm",
    "..........dmmddd",
    "..........dm.md.",
    "..........dm.md.",
    "..........dmmmd.",
    "..........dm.md.",
    "..........dm.md.",
    "..........dmmhhh",
    "..........dmrmmm",
    "..........dmmddd",
    "..........dddd..",
])

BEAMS_MID = pad([
    "hhhhhhhhhhhhhhhh",
    "llllldlllllllll.",
    "mmmmmmmmmmdmmmmm",
    "mmmdmmmmmmmmmmmm",
    "dddddddddddddddd",
    "................",
    "................",
    "hhhhhhhhhhhhhhhh",
    "lllllllll.llllll",
    "mmmmmdmmmmmmmmmm",
    "mmmmmmmmmmmmdmmm",
    "dddddddddddddddd",
])
BEAMS_END = pad([
    "..........iiiiii",
    "..........inhhhh",
    "..........inllll",
    "..........inmmmm",
    "..........iiiiii",
    "..........ii....",
    "..........ii....",
    "..........iiiiii",
    "..........inhhhh",
    "..........inllll",
    "..........inmmmm",
    "..........iiiiii",
])

CHAIN_MID = pad([
    ".hhhh....hhhh...",
    "h....h..h....h..",
    "m....mmmm....mmm",
    "d....d..d....d..",
    ".dddd....dddd...",
])
CHAIN_END = pad([
    "..........dddd..",
    ".........dhhhhd.",
    "........dh....hd",
    "........dm..hhhh",
    "........dm..mmmm",
    "........dm..dddd",
    ".........dmmmmd.",
    "..........dddd..",
])

PIPES_MID = pad([
    "dddddddddddddddd",
    "hhhhhhhhhhhhhhhh",
    "llllllllllllllll",
    "mmmmmmmmmumbmmmm",
    "mmmmmmmmmmmbmmmm",
    "mmmmmmmmmmmbmmmm",
    "dddddddddddddddd",
    "................",
    "dddddddddddddddd",
    "llllllllllllllll",
    "mmumbmmmmmmmmmmm",
    "dddddddddddddddd",
])
PIPES_END = pad([
    ".........ddddddd",
    ".........dhbhhhh",
    ".........dlblllll"[:16],
    ".........dmbmmmm",
    ".........dmbmmmm",
    ".........dmbmmmm",
    ".........ddddddd",
    "...........dd...",
    "...........ddddd",
    "...........dbbll",
    "...........dbmmm",
    "...........ddddd",
])

ARCH_MID = pad([
    "hhhhhhhhhhhhhhhh",
    "llllllxlllllllxl",
    "mmmmmmxmmlmmmmxm",
    "mmlmmmxmmmmmmmxm",
    "mmmmmmxmmmmmlmxm",
    "mmmmmmxmmmmmmmxm",
    "mmmmlmxmmmmmmmxm",
    "dddddddddddddddd",
    "xxxxxxxxxxxxxxxx",
])
ARCH_KEY = pad([
    "...hhhhhhhhhh...",
    "..xllllllllllx..",
    "hhxlmmmmmmmmlxhh",
    "llxlmmmggmmmlxll",
    "mmxlmmgmmgmmlxmm",
    "mmxlmgmxxmgmlxmm",
    "mmxlmmgmmgmmlxmm",
    "mmxlmmmggmmmlxmm",
    "mmxlmmmmmmmmlxmm",
    "ddxdddddddddddxd"[:16],
    "xxxxxxxxxxxxxxxx",
    "...xxxxxxxxxx...",
])
ARCH_END = pad([
    "........hhhhhhhh",
    "........xlllllll",
    "........xlmmmmmm",
    "........xlmmlmmm",
    "........xlmmmmmm",
    "........xlmmmmmm",
    "........xlmmmmlm",
    "........xlmmmmmm",
    "........xlmmmmmm",
    "........xddddddd",
    "........xxxxxxxx",
    "........xlmmmmmm",
    "........xlmmmmmm",
    "........xddddddd",
])

WEB_MID = pad([
    "ssssfsssssfsssss",
    "...f..s...s..f..",
    "..f....s.s....f.",
    ".f......s......f",
    "f......s.s......",
    "sssfsss...sssfss",
    "......s...s.....",
    ".....f.s.s.f....",
    "....f...s...f...",
    "sssssfssssssfsss",
])
WEB_END = pad([
    "...........sss..",
    "........ssss.ss.",
    "......ss....s.s.",
    "....ss.......sss",
    "..ss........s..s",
    "ss.........s...s",
    "..ss.......s...s",
    "....ss......s..s",
    "......ss.....sss",
    "ssssssssss....s.",
    "..........ssss..",
])

WALK_MID = pad([
    "h.......h.......",
    "rhhhhhhhrhhhhhhh",
    "m.......m.......",
    "m.......m.......",
    "hhhhhhhhhhhhhhhh",
    "ldldldldldldldld",
    "dldldldldldldldl",
    "ldldldldldldldld",
    "dldldldldldldldl",
    "ldldldldldldldld",
    "mmmmmmmmmmmmmmmm",
    "dddddddddddddddd",
    ".uu.......uu....",
])
WALK_END = pad([
    "..........mmmmmm",
    "..........mrhhhh",
    "..........mm....",
    "..........mm....",
    "..........mmhhhh",
    "..........mmldld",
    "..........mmdldl",
    "..........mmldld",
    "..........mmdldl",
    "..........mmldld",
    "..........mmmmmm",
    "..........mddddd",
    "..........dd....",
])

# The outer three pixels of an end piece, row by row from the top of the object: what holds it
# where it lies on the wall. Shorter lists repeat their last entry.
PLATE = ["ddd", "dmh", "dmm", "dmm", "dmd"]
TERMINALS = {
    "grate": ["dhd", "dmd", "dmd", "dmd", "dmd", "dmd", "dmd", "dmd", "dmd", "dhd", "dmd", "ddd"],
    "beams": ["...", "...", "...", "...", "...", "...", "...", "...", "...", "...", "...", "..."],
    "chain": [".dd", "dhh", "dm.", "dm.", ".dd"],
    "pipes": ["ddd", "dbh", "dbl", "dbm", "dbm", "dbm", "ddd", "...", "ddd", "dbl", "dbm", "ddd"],
    "arch": ["xhh", "xll", "xlm", "xlm", "xlm", "xlm", "xlm", "xdd", "xxx"],
    "web": ["...", "..s", ".s.", "s..", ".s.", "..s", "...", "...", "..s", ".ss"],
    "walk": ["d..", "d..", "d..", "d..", "dhh", "dmd", "dmd", "dmd", "dmd", "dmd", "dmm", "ddd", "..."],
}


def resting_end(middle, name, beam_ends=False):
    """The left end: the middle finished at its outer edge, where it meets the wall's side."""
    rows = [row for row in middle]
    top = next(index for index, row in enumerate(rows) if row.strip("."))
    bottom = max(index for index, row in enumerate(rows) if row.strip("."))
    terminal = TERMINALS[name]
    end = []
    for index, row in enumerate(rows):
        if top <= index <= bottom:
            piece = terminal[min(index - top, len(terminal) - 1)]
            if beam_ends and row[3] != ".":
                piece = "d" + row[1:3]
            row = piece + row[3:]
        end.append(row)
    return end


LEAVES = {"v": (70, 112, 44), "V": (112, 158, 64), "L": (150, 196, 84), "e": (40, 66, 30), "r": (98, 74, 46), "R": (66, 48, 30)}

VINES_MID = pad([
    "..L.........L...",
    ".LVL.......LVL..",
    "..ve........ve..",
    "vvVVvvvvvVVVvvvv",
    "eVVeeVVVVeeVVVVe",
    "..eV....e...Ve..",
    "...LL.......LL..",
    "....L........L..",
])


def vines_end():
    """The liana wound round a root sticking out of the wall's side, a few leaves on it."""
    rows = [list(row) for row in VINES_MID]
    top = next(index for index, row in enumerate(rows) if "".join(row).strip("."))
    for y in range(H):
        for x in range(0, 3):
            rows[y][x] = "."
    for y in range(top + 1, top + 7):
        rows[y][0] = "R"
        rows[y][1] = "r"
    rows[top + 2][2] = "v"
    rows[top + 3][2] = "V"
    rows[top + 4][2] = "e"
    rows[top][1] = "L"
    rows[top + 7][2] = "L"
    return ["".join(row) for row in rows]


SILK_STRANDS = {"s": (222, 220, 196, 220), "f": (176, 172, 150, 160), "w": (255, 255, 255, 255)}
STRANDS_MID = pad([
    "ssffssssffssssff",
    "................",
    "....w.......w...",
    "................",
    "ffssssffssssffss",
    "..........w.....",
])


def strands_end():
    """The threads stuck to the wall's side in a little tuft of silk."""
    rows = [list(row) for row in STRANDS_MID]
    top = next(index for index, row in enumerate(rows) if "".join(row).strip("."))
    for y in range(top - 1, top + 6):
        for x in range(0, 3):
            rows[y][x] = "s" if (x + y) % 2 else "f"
    return ["".join(row) for row in rows]


PENNANTS = {"c": (150, 108, 56), "C": (200, 150, 70), "r": (172, 38, 40), "R": (120, 22, 28),
            "g": (230, 180, 70), "G": (170, 120, 40), "p": (60, 44, 30), "P": (110, 84, 52)}
PENNANTS_MID = pad([
    "cCccccCccccCcccc",
    ".rrrrr...ggggg..",
    ".rrrrR...gggGG..",
    "..rrR.....ggG...",
    "..rR......gG....",
    "...R.......G....",
])


def pennants_end():
    """The cord tied round a bronze peg driven into the wall's side."""
    rows = [list(row) for row in PENNANTS_MID]
    top = next(index for index, row in enumerate(rows) if "".join(row).strip("."))
    for y in range(H):
        for x in range(0, 3):
            rows[y][x] = "."
    fill(rows, 0, top - 1, 1, top + 1, "P")
    fill(rows, 0, top + 1, 1, top + 1, "p")
    rows[top][2] = "C"
    return ["".join(row) for row in rows]


KINDS = [
    ("grate", GRATE_MID, GRATE_MID, IRON),
    ("beams", BEAMS_MID, BEAMS_MID, WOOD),
    ("chain", CHAIN_MID, CHAIN_MID, IRON),
    ("pipes", PIPES_MID, PIPES_MID, PIPE),
    ("arch", ARCH_MID, ARCH_KEY, STONE),
    ("web", WEB_MID, WEB_MID, SILK),
    ("walk", WALK_MID, WALK_MID, IRON),
    ("vines", VINES_MID, VINES_MID, LEAVES),
    ("strands", STRANDS_MID, STRANDS_MID, SILK_STRANDS),
    ("pennants", PENNANTS_MID, PENNANTS_MID, PENNANTS),
]


def grid():
    return [["."] * W for _ in range(H)]


def fill(rows, x0, y0, x1, y1, key):
    for y in range(y0, y1 + 1):
        for x in range(x0, x1 + 1):
            rows[y][x] = key


def chain_end():
    """The chain's last link hooked through the eye of a bolt in a plate fixed to the wall's side."""
    rows = [list(row) for row in CHAIN_MID]
    for y in range(H):
        for x in range(6):
            rows[y][x] = "."
    # The plate against the wall, lit from the top, with two bolt heads.
    fill(rows, 0, 7, 3, 15, "m")
    fill(rows, 0, 7, 3, 7, "h")
    for y in range(7, 16):
        rows[y][0] = rows[y][3] = "d"
    fill(rows, 0, 15, 3, 15, "d")
    rows[9][1] = rows[13][2] = "r"
    # The eye of the bolt standing out off the plate, the chain's first link through it.
    fill(rows, 4, 9, 5, 9, "h")
    rows[10][5] = rows[11][5] = rows[12][5] = "m"
    fill(rows, 4, 13, 5, 13, "d")
    return ["".join(row) for row in rows]


def beams_end():
    """The planks' ends sitting in an iron bracket bolted to the wall's side."""
    rows = [list(row) for row in BEAMS_MID]
    top = next(index for index, row in enumerate(rows) if "".join(row).strip("."))
    bottom = max(index for index, row in enumerate(rows) if "".join(row).strip("."))
    fill(rows, 0, top - 2, 2, bottom + 2, "i")
    fill(rows, 1, top - 2, 1, bottom + 2, "n")
    for y in range(top, bottom + 1):
        if rows[y][3] != ".":
            rows[y][3] = "d"
    rows[top - 1][1] = rows[bottom + 1][1] = "h"
    return ["".join(row) for row in rows]


MOUNTED = {"chain": chain_end, "beams": beams_end, "vines": vines_end, "strands": strands_end,
           "pennants": pennants_end}


def transpose(rows):
    """The piece turned to run down the screen, its light still from the left, cut to a tile."""
    width = len(rows[0])
    turned = ["".join(rows[y][x] for y in range(len(rows))) for x in range(width)]
    start = (len(turned[0]) - W) // 2
    return [row[start:start + W] for row in turned]


def pieces(name, middle, centre):
    end = MOUNTED[name]() if name in MOUNTED else resting_end(middle, name, beam_ends=(name == "beams"))
    return [end, middle, mirror(end), centre]


def upright_sheet():
    """Each piece transposed: the left end becomes the upper one, the right end the lower."""
    sheet = Image.new("RGBA", (4 * W, len(KINDS) * W), CLEAR)
    for row, (name, middle, centre, palette) in enumerate(KINDS):
        for column, rows in enumerate(pieces(name, middle, centre)):
            sheet.paste(cell(transpose(rows), palette).crop((0, 0, W, W)), (column * W, row * W))
    return sheet


def main():
    upright_sheet().save(OUT / "overheads_upright.tga")
    sheet = Image.new("RGBA", (4 * W, len(KINDS) * H), CLEAR)
    for row, (name, middle, centre, palette) in enumerate(KINDS):
        for column, rows in enumerate(pieces(name, middle, centre)):
            assert all(len(line) == W for line in rows), (row, column)
            sheet.paste(cell(rows, palette), (column * W, row * H))
    sheet.save(OUT / "overheads.tga")


if __name__ == "__main__":
    main()
