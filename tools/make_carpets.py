"""The carpets, one look per floor: the red carpet with its gold border as first drawn (kept in
tools/art/carpet_base.tga, a 4x4 sheet of 16x16 tiles: corners, edges and middle), and the same
weave dyed for each of the other floors. The shop and the treasure rooms keep the red one.

The carpets are shapes, not only rectangles, so each look is laid out as the 47 tiles of a blob
set (8 columns, 6 rows, in the order of `dungeon::blob_rank`): for every way a tile's eight
neighbours can be carpet or not, the border runs round the outside of the shape, round its
corners as first drawn, and round the notches where the shape turns inward, mitred there like a
picture frame. The border's weave is read off the base's straight edges at the same depth.

carpet_tilemap.tga stacks the looks one under another, six rows each, in the order of
`Theme::style`: stone (red and gold), sewer (verdigris and brass), moss (a rush mat, green on
straw), spider (black and violet, silver thread), ice (deep blue and frost), lava (soot and
ember), temple (royal blue and gold).
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/ingame/carpet_tilemap.tga"

# The red carpet's five colours: the dark ground, its lighter weave, and the border's three golds.
GROUND, WEAVE = (32, 3, 2), (69, 9, 5)
GOLD, GOLD_MID, GOLD_DARK = (173, 134, 14), (133, 98, 16), (88, 60, 5)

# The other looks as rich as the red: deep, saturated grounds and a bright border.
LOOKS = [
    None,
    ((8, 40, 30), (20, 84, 62), (204, 142, 52), (150, 98, 36), (96, 58, 20)),
    ((60, 58, 14), (98, 96, 24), (124, 194, 52), (84, 140, 36), (50, 90, 20)),
    ((24, 6, 36), (64, 18, 92), (214, 214, 240), (152, 152, 184), (92, 92, 120)),
    ((6, 28, 72), (20, 66, 140), (232, 246, 255), (150, 200, 240), (80, 130, 190)),
    ((36, 8, 4), (92, 28, 10), (255, 146, 34), (212, 84, 20), (132, 42, 10)),
    ((10, 14, 70), (28, 40, 150), (234, 186, 56), (178, 130, 36), (112, 78, 20)),
]


def dyed(base, look):
    if look is None:
        return base.copy()
    swap = dict(zip((GROUND, WEAVE, GOLD, GOLD_MID, GOLD_DARK), look))
    out = base.copy()
    px = out.load()
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = px[x, y]
            if a and (r, g, b) in swap:
                px[x, y] = swap[(r, g, b)] + (a,)
    return out


T = 16
# The base's tiles, by their place in its 4x4 sheet.
TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT = 0, 1, 3, 4, 5, 6, 9, 10, 12
BLOB_COLUMNS, BLOB_ROWS = 8, 6
# The eight neighbours as bits, as `dungeon::surface` numbers them: the sides north, east, south,
# west, then the corners north-east, south-east, south-west, north-west.
N, E, S, W, NE, SE, SW, NW = (1 << bit for bit in range(8))


def reduce(code):
    """The code with a corner kept only where both sides next to it are carpet too."""
    out = code & 0x0F
    for index, corner in enumerate((NE, SE, SW, NW)):
        if code & corner and code & (1 << index) and code & (1 << ((index + 1) % 4)):
            out |= corner
    return out


BLOBS = sorted({reduce(code) for code in range(256)})
assert len(BLOBS) == 47


def tiles_of(base):
    px = base.load()
    return lambda index, x, y: px[(index % 4) * T + x, (index // 4) * T + y]


def blob_tile(tile, code):
    """The tile for a carpet tile whose neighbours are `code`."""
    has = lambda bit: bool(code & bit)
    out = Image.new("RGBA", (T, T), (0, 0, 0, 0))
    px = out.load()
    last = T - 1
    for y in range(T):
        for x in range(T):
            # Every edge of the shape near this pixel: how deep in from it, which side it is
            # on, and how far along it.
            edges = []
            if not has(N):
                edges.append((y, "top", x))
            if not has(S):
                edges.append((last - y, "bottom", x))
            if not has(W):
                edges.append((x, "left", y))
            if not has(E):
                edges.append((last - x, "right", y))
            # The notches: the border turns round the corner where the shape steps inward.
            if has(N) and has(W) and not has(NW):
                depth = max(x, y)
                edges.append((depth, "left", y) if x >= y else (depth, "top", x))
            if has(N) and has(E) and not has(NE):
                depth = max(last - x, y)
                edges.append((depth, "right", y) if last - x >= y else (depth, "top", x))
            if has(S) and has(W) and not has(SW):
                depth = max(x, last - y)
                edges.append((depth, "left", y) if x >= last - y else (depth, "bottom", x))
            if has(S) and has(E) and not has(SE):
                depth = max(last - x, last - y)
                edges.append((depth, "right", y) if last - x >= last - y else (depth, "bottom", x))
            if not edges:
                px[x, y] = tile(CENTER, x, y)
                continue
            depth, side, along = min(edges)
            if depth >= T:
                px[x, y] = tile(CENTER, x, y)
                continue
            # Where two outer edges meet, the corner as first drawn.
            corner = {
                ("top", "left"): TOP_LEFT, ("top", "right"): TOP_RIGHT,
                ("bottom", "left"): BOTTOM_LEFT, ("bottom", "right"): BOTTOM_RIGHT,
            }
            sides = {"top": not has(N), "bottom": not has(S), "left": not has(W), "right": not has(E)}
            vertical = "top" if y < T / 2 else "bottom"
            horizontal = "left" if x < T / 2 else "right"
            if sides[vertical] and sides[horizontal] and side in (vertical, horizontal):
                px[x, y] = tile(corner[(vertical, horizontal)], x, y)
                continue
            px[x, y] = {
                "top": lambda: tile(TOP, along, depth),
                "bottom": lambda: tile(BOTTOM, along, last - depth),
                "left": lambda: tile(LEFT, depth, along),
                "right": lambda: tile(RIGHT, last - depth, along),
            }[side]()
    return out


def blob_sheet(base):
    tile = tiles_of(base)
    sheet = Image.new("RGBA", (BLOB_COLUMNS * T, BLOB_ROWS * T), (0, 0, 0, 0))
    for index, code in enumerate(BLOBS):
        sheet.paste(blob_tile(tile, code), ((index % BLOB_COLUMNS) * T, (index // BLOB_COLUMNS) * T))
    return sheet


def main():
    base = Image.open(ROOT / "tools/art/carpet_base.tga").convert("RGBA")
    height = BLOB_ROWS * T
    sheet = Image.new("RGBA", (BLOB_COLUMNS * T, height * len(LOOKS)), (0, 0, 0, 0))
    for index, look in enumerate(LOOKS):
        sheet.paste(blob_sheet(dyed(base, look)), (0, index * height))
    sheet.save(OUT)


if __name__ == "__main__":
    main()
