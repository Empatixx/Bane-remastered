"""Door bars (pathgate.tga) and arrow cursor (cursor.tga) at a chunky pixel scale.

The bars are drawn at 24 px per tile, three to a tile; the cursor is 32x32 so one
texel is one screen pixel.

    uv run --with pillow python tools/make_pathgate.py [out_dir]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/ingame"

CELL = 24
COLUMNS = 6
ROWS = 4

OUTLINE = (19, 19, 19, 255)
SHADOW = (26, 23, 23, 255)
IRON_D = (66, 64, 64, 255)
IRON_M = (89, 87, 87, 255)
IRON = (102, 99, 98, 255)
IRON_L = (122, 118, 118, 255)
TIP_D = (70, 71, 98, 255)
TIP_M = (105, 102, 130, 255)
TIP = (154, 151, 185, 255)
TIP_L = (197, 199, 221, 255)
TIP_W = (230, 231, 240, 255)
GLINT = (255, 255, 255, 255)
STREAK = (230, 231, 240, 170)

# A standing bar, 8 px wide, seen from the front: spike, collar, banded shaft.
# Rows from the top of the bar; "." is transparent.
BAR = [
    "...oo...",
    "..oWLo..",
    "..oWMo..",
    ".oWLMdo.",
    ".oLMmdo.",
    ".oooooo.",
    ".oCCCCo.",
    "..oIio..",
    "..oIio..",
    "..oDDo..",
    "..oIio..",
    "..oIio..",
    "..oDDo..",
    "..oIio..",
    "..oIio..",
    "..oDDo..",
    "..oIio..",
    "..oIio..",
    "..oDDo..",
    "..oIio..",
    "..oIio..",
    "..oDDo..",
]
BAR_KEY = {
    "o": OUTLINE,
    "W": TIP_W,
    "L": TIP_L,
    "M": TIP,
    "m": TIP_M,
    "d": TIP_D,
    "C": IRON_M,
    "I": IRON_L,
    "i": IRON,
    "D": IRON_D,
}

# A bar poking out of a side wall, 6 px tall, seen from above, pointing right.
STUB = [
    "ooooooooo.",
    "sCDCCDoWo.",
    "sIDIIDoLWo",
    "siDiiDomMo",
    "sDDDDDodo.",
    "ooooooooo.",
]
STUB_KEY = dict(BAR_KEY, s=SHADOW)


def put(image, x, y, colour):
    if 0 <= x < image.width and 0 <= y < image.height:
        image.putpixel((x, y), colour)


def stamp(image, pattern, key, ox, oy, clip):
    for row, line in enumerate(pattern):
        for column, char in enumerate(line):
            x, y = ox + column, oy + row
            if char != "." and clip(x, y):
                put(image, x, y, key[char])


def standing(image, cell_x, cell_y, rise, glints=(), streaks=False):
    """Three bars in one cell; `rise` is how many rows stick out of the floor."""
    floor = cell_y + CELL - 1
    for bar, cx in enumerate(range(0, CELL, 8)):
        top = floor - rise
        stamp(
            image,
            BAR,
            BAR_KEY,
            cell_x + cx,
            top,
            lambda x, y: cell_y <= y < floor,
        )
        put(image, cell_x + cx + 1, floor, SHADOW)
        for x in range(2, 6):
            put(image, cell_x + cx + x, floor, OUTLINE)
        put(image, cell_x + cx + 6, floor, SHADOW)
        if streaks and rise > 3:
            for y in range(max(top + 5, cell_y), floor, 2):
                put(image, cell_x + cx + 1, y, STREAK)
                put(image, cell_x + cx + 6, y + 1, STREAK)
        if bar in glints and rise >= len(BAR):
            put(image, cell_x + cx + 3, top + 1, GLINT)
            put(image, cell_x + cx + 2, top + 2, GLINT)


def lying(image, cell_x, cell_y, reach, glints=(), flash=False):
    """Three stubs in one cell; `reach` is how many columns stick out of the wall."""
    key = dict(STUB_KEY)
    if flash:
        key.update(W=GLINT, L=GLINT, M=TIP_L)
    for bar, cy in enumerate(range(1, CELL, 8)):
        left = cell_x - (len(STUB[0]) - reach)
        stamp(
            image,
            STUB,
            key,
            left,
            cell_y + cy,
            lambda x, y: x >= cell_x,
        )
        if bar in glints and reach == len(STUB[0]):
            put(image, left + 7, cell_y + cy + 1, GLINT)
            put(image, left + 8, cell_y + cy + 2, GLINT)


def pathgate():
    image = Image.new("RGBA", (CELL * COLUMNS, CELL * ROWS), (0, 0, 0, 0))
    full = len(BAR)
    for frame, rise in enumerate((4, 11, 17, full)):
        standing(image, frame * CELL, 0, rise, streaks=frame in (1, 2))
    for frame, glints in enumerate(((0,), (), (2,), (1,), (), ())):
        standing(image, frame * CELL, CELL, full, glints)
    reach = len(STUB[0])
    for frame, shown in enumerate((3, 6, 8, reach)):
        lying(image, frame * CELL, 2 * CELL, shown, flash=frame in (1, 2))
    for frame, glints in enumerate(((0,), (), (2,), (1,), (), ())):
        lying(image, frame * CELL, 3 * CELL, reach, glints)
    return image


# The arrow's outline in texels: tip, right wing, notch, tail, inner notch, left wing.
ARROW = [
    (2.5, 2.5),
    (29.5, 18.5),
    (17.5, 18.0),
    (24.5, 26.0),
    (19.0, 29.0),
    (12.5, 20.5),
    (4.0, 26.0),
]
EDGE = (38, 20, 14, 255)
FILL = (223, 113, 38, 255)
RIM = (238, 195, 154, 255)


def inside(polygon, x, y):
    hit = False
    for (x1, y1), (x2, y2) in zip(polygon, polygon[1:] + polygon[:1]):
        if (y1 > y) != (y2 > y) and x < x1 + (y - y1) * (x2 - x1) / (y2 - y1):
            hit = not hit
    return hit


def cursor():
    size = 32
    filled = {
        (x, y)
        for y in range(size)
        for x in range(size)
        if inside(ARROW, x + 0.5, y + 0.5)
    }
    image = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    for x, y in filled:
        edge = any((x + dx, y + dy) not in filled for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)))
        put(image, x, y, RIM if edge else FILL)
    for y in range(size):
        for x in range(size):
            if (x, y) not in filled and any(
                (x + dx, y + dy) in filled for dx in (-1, 0, 1) for dy in (-1, 0, 1)
            ):
                put(image, x, y, EDGE)
    return image


if __name__ == "__main__":
    OUT.mkdir(parents=True, exist_ok=True)
    pathgate().save(OUT / "pathgate.tga")
    cursor().save(OUT / "cursor.tga")
