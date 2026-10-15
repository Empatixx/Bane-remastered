"""Things that lie across a room from the top of one wall to the top of another: an iron grate, wooden beams, a chain, pipes,
a stone arch, a thick web and a metal walkway.

overheads.tga has one row per kind in that order and four 16x24 cells per row: the left end
(lying on the top of the wall, with a contact shadow under it), a middle piece that repeats seamlessly, the right end, and a centre piece
(the arch's keystone; for the others it is the plain middle again).
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

SHADOW = {"z": (0, 0, 0, 120), "y": (0, 0, 0, 55)}

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
    """The left end: the middle carried on over the top of the wall, finished at its outer edge,
    with a contact shadow under it where it lies on the wall."""
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
    for offset, key in ((1, "z"), (2, "y")):
        line = bottom + offset
        if line < H:
            end[line] = "".join(key if ch == "." else ch for ch in end[line])
    return end


KINDS = [
    ("grate", GRATE_MID, GRATE_MID, IRON),
    ("beams", BEAMS_MID, BEAMS_MID, WOOD),
    ("chain", CHAIN_MID, CHAIN_MID, IRON),
    ("pipes", PIPES_MID, PIPES_MID, PIPE),
    ("arch", ARCH_MID, ARCH_KEY, STONE),
    ("web", WEB_MID, WEB_MID, SILK),
    ("walk", WALK_MID, WALK_MID, IRON),
]


def main():
    sheet = Image.new("RGBA", (4 * W, len(KINDS) * H), CLEAR)
    for row, (name, middle, centre, palette) in enumerate(KINDS):
        end = resting_end(middle, name, beam_ends=(name == "beams"))
        palette = {**palette, **SHADOW}
        pieces = [end, middle, mirror(end), centre]
        for column, rows in enumerate(pieces):
            assert all(len(line) == W for line in rows), (row, column)
            sheet.paste(cell(rows, palette), (column * W, row * H))
    sheet.save(OUT / "overheads.tga")


if __name__ == "__main__":
    main()
