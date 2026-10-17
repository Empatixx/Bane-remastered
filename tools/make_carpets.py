"""The carpets, one look per floor: the red carpet with its gold border as first drawn (kept in
tools/art/carpet_base.tga, a 4x4 sheet of 16x16 tiles: corners, edges and middle), and the same
weave dyed for each of the other floors. The shop and the treasure rooms keep the red one.

carpet_tilemap.tga stacks the looks one under another, four rows each, in the order of
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


def main():
    base = Image.open(ROOT / "tools/art/carpet_base.tga").convert("RGBA")
    sheet = Image.new("RGBA", (base.width, base.height * len(LOOKS)), (0, 0, 0, 0))
    for index, look in enumerate(LOOKS):
        sheet.paste(dyed(base, look), (0, index * base.height))
    sheet.save(OUT)


if __name__ == "__main__":
    main()
