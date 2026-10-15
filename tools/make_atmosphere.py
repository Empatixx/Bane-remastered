"""Biome atmosphere: drifting fog banks and the small motes (dust, embers,
snow, spores, water drips) that float through the dungeon.

Fog is white and get their colour in the game; the motes carry their own palettes.
Everything is drawn at the tiles' pixel scale (one pixel here is four world units) and the soft
shapes use a few hard alpha bands instead of smooth gradients, so they read as pixel art.
"""

import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
CLEAR = (0, 0, 0, 0)
BANDS = (0, 70, 130, 190, 255)

FOG_SIZE = (48, 24)
MOTE = 8


def band(value):
    """Snaps a 0..1 strength to one of the hard alpha bands."""
    index = min(len(BANDS) - 1, max(0, int(value * (len(BANDS) - 1) + 0.35)))
    return BANDS[index]


def fog(seed):
    r = random.Random(seed)
    width, height = FOG_SIZE
    lumps = [
        (r.uniform(0.25, 0.75) * width, r.uniform(0.4, 0.6) * height, r.uniform(9, 14), r.uniform(5, 7))
        for _ in range(5)
    ]
    image = Image.new("RGBA", FOG_SIZE, CLEAR)
    for y in range(height):
        for x in range(width):
            strength = 0.0
            for cx, cy, rx, ry in lumps:
                d = ((x + 0.5 - cx) / rx) ** 2 + ((y + 0.5 - cy) / ry) ** 2
                strength = max(strength, 1.0 - d)
            # A slow ripple keeps the band edges from looking like perfect ellipses.
            strength += 0.08 * math.sin(x * 0.7 + seed) * math.cos(y * 0.9 - seed)
            alpha = band(min(0.8, max(0.0, strength)))
            if alpha:
                image.putpixel((x, y), (255, 255, 255, alpha))
    return image


DUST = {"o": (206, 192, 166, 255), "O": (238, 228, 204, 255)}
EMBER = {"y": (255, 242, 176, 255), "o": (255, 186, 72, 255), "r": (230, 102, 34, 255), "d": (150, 44, 24, 255)}
SNOW = {"w": (242, 250, 255, 255), "b": (178, 210, 238, 255)}
SPORE = {"y": (232, 246, 132, 255), "g": (146, 196, 72, 255)}
WATER = {"w": (214, 242, 238, 255), "t": (122, 186, 186, 255)}

MOTES = [
    # Dust.
    (["........", "........", "........", "...Oo...", "...oo...", "........", "........", "........"], DUST),
    # Big ember.
    (["........", "........", "....d...", "...ror..", "..dryod.", "...ror..", "....d...", "........"], EMBER),
    # Small ember.
    (["........", "........", "........", "...yo...", "...or...", "........", "........", "........"], EMBER),
    # Big flake.
    (["........", "........", "....b...", "...bwb..", "....b...", "........", "........", "........"], SNOW),
    # Small flake.
    (["........", "........", "........", "...w....", "....b...", "........", "........", "........"], SNOW),
    # Spore.
    (["........", "........", "........", "...y....", "....g...", "........", "........", "........"], SPORE),
    # Drip.
    (["........", "...w....", "...w....", "...t....", "........", "........", "........", "........"], WATER),
    # Splash.
    (["........", "........", "........", "........", ".t...t..", "..twt...", "........", "........"], WATER),
]


def mote(rows, palette):
    image = Image.new("RGBA", (MOTE, MOTE), CLEAR)
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key in palette:
                image.putpixel((x, y), palette[key])
    return image


def strip(images):
    width = sum(image.width for image in images)
    sheet = Image.new("RGBA", (width, images[0].height), CLEAR)
    x = 0
    for image in images:
        sheet.paste(image, (x, 0))
        x += image.width
    return sheet


def main():
    strip([fog(3), fog(11)]).save(OUT / "fog.tga")
    strip([mote(rows, palette) for rows, palette in MOTES]).save(OUT / "motes.tga")


if __name__ == "__main__":
    main()
