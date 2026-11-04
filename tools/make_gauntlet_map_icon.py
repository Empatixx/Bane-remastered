"""Adds the gauntlet's minimap icon, a chest over spikes (drawn in tools/art/gauntlet_map_icon.ase,
exported to PNG with `lsprite render --scale 1`), as the eighth cell of minimap_icons.tga, on the
same tile as the others.

    uv run -q --with pillow python tools/make_gauntlet_map_icon.py ICON.png
"""
import sys
from pathlib import Path

from PIL import Image

CELL = 16
SHEET = Path(__file__).resolve().parent.parent / "assets/textures/ingame/minimap_icons.tga"


def main():
    old = Image.open(SHEET).convert("RGBA")
    sheet = Image.new("RGBA", (CELL * 8, CELL), (0, 0, 0, 0))
    sheet.paste(old.crop((0, 0, CELL * 7, CELL)), (0, 0))
    tile = old.crop((CELL, 0, CELL * 2, CELL))
    tile.alpha_composite(Image.open(sys.argv[1]).convert("RGBA"))
    sheet.paste(tile, (CELL * 7, 0))
    sheet.save(SHEET)


main()
