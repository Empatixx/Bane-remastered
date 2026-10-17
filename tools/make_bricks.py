"""A pile of loose bricks in the treasure rooms, one look per floor.

bricks.tga has a row per floor, in the order of `Theme::style`, of one 16x16 cell: the stone pile
from the first cell of the original sheet, recoloured in that floor's own stone (its colours taken
from the floor's tileset) and redrawn by hand at its pixel scale: the original (stone), dark wet
stone with a slime drip and a puddle (sewer), grey-green blocks under moss (moss), dusty dark stone
strung with webs (spider), pale blue ice with frost caps and icicles (ice), charred stone with
ember-bright cracks (lava) and sandstone with a gilded glyph and drifted sand (temple).

The stone row is the original pile untouched, so the sheet rebuilds from its own first cell.
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else ROOT / "assets/textures/ingame"
SOURCE = ROOT / "assets/textures/ingame/bricks.tga"
S = 16
CLEAR = (0, 0, 0, 0)

# The original pile's colours: its outline, the bricks' sides and their tops.
ORIGINAL = {(37, 19, 26): "O", (84, 55, 64): "M", (110, 74, 72): "L", (30, 22, 22): "O"}


def base():
    """The original pile as a grid of letters, read from the first cell of bricks.tga."""
    image = Image.open(SOURCE).convert("RGBA")
    grid = []
    for y in range(S):
        line = ""
        for x in range(S):
            pixel = image.getpixel((x, y))
            line += "." if pixel[3] == 0 else ORIGINAL[pixel[:3]]
        grid.append(line)
    return grid


def paint(grid, palette, details):
    cell = Image.new("RGBA", (S, S), CLEAR)
    for y, line in enumerate(grid):
        for x, letter in enumerate(line):
            if letter != ".":
                cell.putpixel((x, y), palette[letter] + (255,))
    for x, y, colour in details:
        cell.putpixel((x, y), colour if len(colour) == 4 else colour + (255,))
    return cell


def dots(colour, *points):
    return [(x, y, colour) for x, y in points]


# Each floor: the outline (its backdrop), the bricks' sides and tops, and what is drawn over them.
def stone():
    return {"O": (37, 19, 26), "M": (84, 55, 64), "L": (110, 74, 72)}, []


def sewer():
    wet = (104, 116, 92)
    glint = (140, 150, 120)
    slime = (86, 118, 52)
    slime_dark = (58, 84, 38)
    water = (30, 46, 42)
    return {"O": (16, 26, 22), "M": (42, 58, 53), "L": (60, 82, 72)}, [
        # Wet glints along the tops' front edges.
        *dots(glint, (2, 4), (7, 9)),
        *dots(wet, (3, 4), (6, 5), (8, 9), (12, 10)),
        # Slime running down the upper brick's side.
        *dots(slime, (2, 6), (2, 7), (3, 7)),
        *dots(slime_dark, (2, 8), (2, 9)),
        # A puddle seeping out from under the pile.
        *dots(water, (1, 12), (2, 12), (3, 12), (4, 12), (2, 13), (3, 13)),
        *dots(glint, (2, 12)),
    ]


def moss():
    moss = (78, 110, 44)
    moss_dark = (50, 76, 32)
    moss_light = (116, 146, 62)
    return {"O": (22, 24, 16), "M": (58, 60, 52), "L": (88, 90, 74)}, [
        # A cushion of moss on the upper brick, tufts poking over its edge.
        *dots(moss, (1, 4), (2, 4), (3, 4), (4, 4), (2, 5), (3, 5), (1, 5)),
        *dots(moss_light, (2, 3), (4, 3), (2, 4)),
        *dots(moss_dark, (3, 3), (5, 4), (4, 5)),
        # Some trailing down its side.
        *dots(moss_dark, (1, 6), (1, 7), (1, 8)),
        *dots(moss, (1, 9)),
        # And on the lower brick's far end.
        *dots(moss, (11, 9), (12, 9), (13, 9), (12, 10), (13, 10)),
        *dots(moss_light, (12, 8), (13, 9)),
        *dots(moss_dark, (11, 10), (14, 9)),
    ]


def spider():
    dust = (112, 108, 104)
    web = (150, 146, 140, 210)
    faint = (150, 146, 140, 130)
    return {"O": (14, 10, 16), "M": (42, 37, 48), "L": (62, 56, 72)}, [
        # Dust settled on the tops.
        *dots(dust, (3, 4), (7, 5), (9, 10), (12, 9)),
        # A web strung across the corner between the two bricks: one strand from the upper brick's
        # corner down to the lower one, another crossing it.
        *dots(web, (11, 4), (12, 5), (13, 6), (14, 7)),
        *dots(faint, (11, 7), (12, 6), (13, 5)),
        # A strand across the upper brick's side and down to the floor.
        *dots(web, (3, 8), (4, 9), (5, 10)),
        *dots(faint, (0, 7), (0, 8), (1, 11), (0, 9)),
    ]


def ice():
    frost = (220, 240, 250)
    pale = (150, 190, 214)
    return {"O": (14, 24, 40), "M": (84, 120, 162), "L": (130, 172, 204)}, [
        # Frost along the tops' back edges.
        *dots(frost, (2, 4), (3, 4), (4, 4), (6, 4), (7, 4), (8, 9), (9, 9), (11, 9), (12, 9)),
        *dots(pale, (5, 4), (8, 4), (10, 9), (13, 9), (3, 5)),
        # A glint on each side.
        *dots(pale, (2, 7), (6, 12)),
        # Icicles hanging from the upper brick's underside.
        *dots(pale, (2, 12), (4, 12)),
        *dots(frost, (2, 13)),
    ]


def lava():
    ember = (255, 140, 40)
    ember_dark = (196, 66, 24)
    hot = (255, 214, 96)
    return {"O": (20, 8, 8), "M": (56, 33, 33), "L": (80, 48, 44)}, [
        # Ember-bright cracks in the mortar and across the faces.
        *dots(ember, (5, 6), (6, 11)),
        *dots(hot, (7, 11)),
        *dots(ember_dark, (8, 11), (4, 6), (6, 6)),
        *dots(ember_dark, (3, 7), (4, 8), (5, 9)),
        *dots(ember, (4, 7)),
        *dots(ember_dark, (10, 12), (11, 13)),
        *dots(ember, (11, 12)),
        # Ash on the tops.
        *dots((110, 72, 64), (2, 4), (3, 4), (9, 9), (10, 9)),
    ]


def temple():
    gold = (232, 190, 72)
    gold_dark = (168, 124, 44)
    carved = (100, 80, 54)
    sand = (168, 138, 92)
    return {"O": (36, 24, 18), "M": (130, 106, 74), "L": (168, 138, 92)}, [
        # The tops catch the light.
        *dots((206, 176, 120), (2, 4), (3, 4), (4, 4), (8, 9), (9, 9), (10, 9)),
        # A gilded glyph cut into the upper brick's side.
        *dots(gold, (4, 7), (4, 9)),
        *dots(gold_dark, (3, 8), (5, 8), (4, 8)),
        # A seam carved along the lower brick.
        *dots(carved, (8, 12), (9, 12), (11, 12), (12, 12)),
        # Sand drifted against the foot of the pile.
        *dots(sand, (1, 12), (2, 12), (3, 12), (4, 12), (2, 13), (3, 13)),
        *dots(gold_dark, (2, 12)),
    ]


# In the order of `Theme::style`.
LOOKS = [stone, sewer, moss, spider, ice, lava, temple]


def sheet():
    grid = base()
    image = Image.new("RGBA", (S, S * len(LOOKS)), CLEAR)
    for row, look in enumerate(LOOKS):
        palette, details = look()
        image.paste(paint(grid, palette, details), (0, row * S))
    return image


def preview(image, path, scale=12):
    """The sheet laid out as a strip on a dark floor, enlarged without smoothing."""
    count = image.height // S
    strip = Image.new("RGBA", (count * (S + 4) + 4, S + 8), (18, 14, 20, 255))
    for row in range(count):
        cell = image.crop((0, row * S, S, (row + 1) * S))
        strip.alpha_composite(cell, (4 + row * (S + 4), 4))
    strip.resize((strip.width * scale, strip.height * scale), Image.NEAREST).save(path)


def main():
    image = sheet()
    if "--preview" in sys.argv:
        preview(image, Path(sys.argv[sys.argv.index("--preview") + 1]))
        return
    image.save(OUT / "bricks.tga")


if __name__ == "__main__":
    main()
