import sys
from pathlib import Path

from PIL import Image, ImageDraw

TILE = 16
SCALE = 3
GAP = 3
ROOT = Path(__file__).resolve().parent.parent / "assets/textures/ingame"

GROUPS = [
    ("Podlaha stred", [5, 6] + list(range(12, 20)) + list(range(21, 29))),
    ("Podlaha pod zdi", [9, 10] + list(range(29, 37))),
    ("Podlaha nad zdi", [1, 2] + list(range(37, 45))),
    ("Podlaha vlevo od zdi", [7] + list(range(45, 53))),
    ("Podlaha vpravo od zdi", [4] + list(range(53, 61))),
    ("Rohy podlahy", [0, 3, 8, 11]),
]

THEMES = [
    ("sewer", "tileset_sewer.png"),
    ("moss", "tileset_moss.png"),
    ("spider", "tileset_spider.png"),
    ("ice", "tileset_ice.png"),
    ("lava", "tileset_lava.png"),
    ("temple", "tileset_temple.png"),
]


def block(image, tiles, label):
    width = (len(tiles) * (TILE * SCALE + GAP)) + 6
    height = TILE * SCALE + 18
    out = Image.new("RGBA", (width, height), (30, 26, 36, 255))
    ImageDraw.Draw(out).text((4, 2), label, fill=(255, 255, 255, 255))
    for index, tile in enumerate(tiles):
        crop = image.crop((tile * TILE, 0, (tile + 1) * TILE, TILE)).resize(
            (TILE * SCALE, TILE * SCALE), Image.NEAREST
        )
        backing = Image.new("RGBA", crop.size, (255, 0, 255, 255))
        backing.alpha_composite(crop)
        out.paste(backing, (4 + index * (TILE * SCALE + GAP), 16))
    return out


def main():
    target = Path(sys.argv[1])
    only = sys.argv[2:] or [name for name, _ in THEMES]
    sheets = []
    for name, file in THEMES:
        if name not in only:
            continue
        image = Image.open(ROOT / file).convert("RGBA")
        blocks = [block(image, tiles, f"{name}: {label}") for label, tiles in GROUPS]
        width = max(b.width for b in blocks)
        sheet = Image.new("RGBA", (width, sum(b.height + 4 for b in blocks)), (18, 16, 22, 255))
        y = 0
        for b in blocks:
            sheet.paste(b, (0, y))
            y += b.height + 4
        sheets.append(sheet)
    width = max(s.width for s in sheets)
    out = Image.new("RGBA", (width, sum(s.height + 14 for s in sheets)), (10, 8, 12, 255))
    y = 0
    for s in sheets:
        out.paste(s, (0, y))
        y += s.height + 14
    out.save(target)


main()
