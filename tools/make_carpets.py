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

LOOKS = [
    None,
    ((14, 30, 26), (30, 62, 52), (176, 120, 60), (134, 86, 42), (84, 52, 26)),
    ((70, 60, 28), (104, 92, 44), (132, 170, 70), (96, 128, 48), (58, 82, 30)),
    ((18, 10, 24), (46, 22, 60), (196, 196, 214), (140, 140, 160), (84, 84, 100)),
    ((14, 30, 58), (32, 62, 108), (220, 236, 250), (160, 196, 228), (96, 130, 170)),
    ((20, 12, 10), (48, 26, 20), (240, 120, 40), (190, 74, 24), (120, 40, 16)),
    ((16, 20, 56), (36, 44, 110), (214, 170, 60), (160, 120, 40), (100, 72, 24)),
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
