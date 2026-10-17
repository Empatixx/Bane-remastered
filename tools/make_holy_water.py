"""Holy water for the temple's alcoves in the opening cinematic: the game's water surface and its
banks (surface_water.tga, shore_water.tga) redyed from murky green to a clear, pale, shining
blue, each colour moved by its brightness onto a hand-picked ramp so the ripples keep their
shape.

    uv run -q --with pillow python tools/make_holy_water.py [preview.png]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
INGAME = ROOT / "assets/textures/ingame"

# Dark to bright: deep pool, body, light, sheen, glint.
RAMP = [(84, 156, 196), (116, 192, 224), (152, 218, 238), (188, 236, 248), (224, 248, 253), (252, 255, 255)]


def redye(image):
    out = image.copy()
    px = out.load()
    colours = sorted({px[x, y][:3] for y in range(out.height) for x in range(out.width) if px[x, y][3]},
                     key=lambda c: 0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2])
    # The source's colours, darkest to brightest, spread over the ramp by rank.
    mapping = {colour: RAMP[min(len(RAMP) - 1, i * len(RAMP) // max(1, len(colours)))] for i, colour in enumerate(colours)}
    for y in range(out.height):
        for x in range(out.width):
            r, g, b, a = px[x, y]
            if a:
                px[x, y] = (*mapping[(r, g, b)], a)
    return out


def main():
    surface = redye(Image.open(INGAME / "surface_water.tga").convert("RGBA"))
    shore = redye(Image.open(INGAME / "shore_water.tga").convert("RGBA"))
    if len(sys.argv) > 1:
        crop = surface.crop((0, 0, 64, 64))
        preview = Image.new("RGBA", (64 * 2 + 8, 64), (30, 28, 36, 255))
        preview.alpha_composite(Image.open(INGAME / "surface_water.tga").convert("RGBA").crop((0, 0, 64, 64)), (0, 0))
        preview.alpha_composite(crop, (72, 0))
        preview.resize((preview.width * 6, preview.height * 6), Image.NEAREST).save(sys.argv[1])
        return
    surface.save(INGAME / "surface_holy_water.tga")
    shore.save(INGAME / "shore_holy_water.tga")


if __name__ == "__main__":
    main()
