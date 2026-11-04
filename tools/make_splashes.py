"""Splashes and drops for the floor's liquids, from the pictures drawn by hand in LibreSprite in
the sewer water's colours (tools/art/splash_big.ase, splash_small.ase, exported to PNG
with `lsprite render tools/art/<name>.ase --scale 1 --out tools/art/<name>.png`), each colour
mapped to a hand-picked one for every liquid. One row per liquid: water, lava, blood, bog.

- assets/textures/ingame/splash_big.tga: something big falling in (7 frames of 32x32): it goes
  in, a crown rises, a column shoots up and falls back, the ring spreads.
- assets/textures/ingame/splash_small.tga: a drop or a small thing falling in (5 frames of 16x16).

    uv run -q --with pillow python tools/make_splashes.py
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
ART = ROOT / "tools/art"
OUT = ROOT / "assets/textures/ingame"

# The drawn colours, lightest to darkest: foam, light, mid, dark, outline.
DRAWN = [(0xF2, 0xF8, 0xE8), (0xC6, 0xDA, 0xB0), (0x8E, 0xAC, 0x6E), (0x4E, 0x6E, 0x36), (0x22, 0x30, 0x1A)]
LIQUIDS = [
    DRAWN,  # water, as drawn
    [(0xFF, 0xF4, 0xA8), (0xFF, 0xC0, 0x40), (0xF0, 0x70, 0x18), (0xA8, 0x30, 0x0E), (0x4A, 0x12, 0x08)],  # lava
    [(0xF0, 0x9A, 0x9A), (0xC8, 0x3A, 0x40), (0x8E, 0x14, 0x1E), (0x56, 0x0A, 0x12), (0x24, 0x04, 0x08)],  # blood
    [(0xD6, 0xC8, 0x96), (0xA8, 0x92, 0x5C), (0x7A, 0x66, 0x3C), (0x4E, 0x40, 0x26), (0x22, 0x1A, 0x10)],  # bog
]


def sheet(name):
    art = Image.open(ART / f"{name}.png").convert("RGBA")
    out = Image.new("RGBA", (art.width, art.height * len(LIQUIDS)), (0, 0, 0, 0))
    for row, colours in enumerate(LIQUIDS):
        mapped = art.copy()
        px = mapped.load()
        for y in range(art.height):
            for x in range(art.width):
                r, g, b, a = px[x, y]
                if a and (r, g, b) in DRAWN:
                    px[x, y] = (*colours[DRAWN.index((r, g, b))], a)
        out.paste(mapped, (0, row * art.height))
    return out


def main():
    sheet("splash_big").save(OUT / "splash_big.tga")
    sheet("splash_small").save(OUT / "splash_small.tga")


if __name__ == "__main__":
    main()
