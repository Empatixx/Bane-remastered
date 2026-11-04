"""The lava floor's moving things, from the pictures drawn by hand in LibreSprite (tools/art,
exported to PNG with `lsprite render tools/art/<name>.ase --scale 1 --out tools/art/<name>.png`):

- assets/textures/ingame/lava_trough.tga: a trough of molten metal lying across a gap over the
  lava (tools/art/trough.ase: four frames of the metal flowing, each a 16x24 wall end beside a
  16x24 middle piece), as three rows of four frames: the left end, the middle, the right end.
- assets/textures/ingame/lava_drip.tga: a drop of metal falling from the trough into the lava
  (tools/art/drip.ase, six frames of 8x30), as a 24-frame timeline: mostly nothing, then the drop.
- assets/textures/ingame/floor_crack.tga: cracks in the floor glowing with heat
  (tools/art/crack.ase: four kinds side by side, four frames of glow), a row per kind.

    uv run -q --with pillow python tools/make_lava_props.py
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
ART = ROOT / "tools/art"
OUT = ROOT / "assets/textures/ingame"
FLIP = Image.Transpose.FLIP_LEFT_RIGHT

DRIP_TIMELINE = [None] * 18 + [0, 1, 2, 3, 4, 5]


def strip(name, width):
    art = Image.open(ART / f"{name}.png").convert("RGBA")
    return [art.crop((i * width, 0, (i + 1) * width, art.height)) for i in range(art.width // width)]


def trough():
    frames = strip("trough", 32)
    sheet = Image.new("RGBA", (16 * len(frames), 24 * 3), (0, 0, 0, 0))
    for column, frame in enumerate(frames):
        end, middle = frame.crop((0, 0, 16, 24)), frame.crop((16, 0, 32, 24))
        for row, piece in enumerate([end, middle, end.transpose(FLIP)]):
            sheet.paste(piece, (column * 16, row * 24))
    return sheet


def drip():
    frames = strip("drip", 8)
    sheet = Image.new("RGBA", (8 * len(DRIP_TIMELINE), 30), (0, 0, 0, 0))
    for column, frame in enumerate(DRIP_TIMELINE):
        if frame is not None:
            sheet.paste(frames[frame], (column * 8, 0))
    return sheet


def cracks():
    frames = strip("crack", 64)
    sheet = Image.new("RGBA", (16 * len(frames), 16 * 4), (0, 0, 0, 0))
    for column, frame in enumerate(frames):
        for kind in range(4):
            sheet.paste(frame.crop((kind * 16, 0, kind * 16 + 16, 16)), (column * 16, kind * 16))
    return sheet


def main():
    trough().save(OUT / "lava_trough.tga")
    drip().save(OUT / "lava_drip.tga")
    cracks().save(OUT / "floor_crack.tga")


if __name__ == "__main__":
    main()
