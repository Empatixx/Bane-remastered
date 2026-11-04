"""The sewer's little moving things, from the pictures drawn by hand in LibreSprite (tools/art,
exported to PNG with `lsprite render tools/art/<name>.ase --scale 1 --out tools/art/<name>.png`),
laid out as timelines the game loops through:

- assets/textures/ingame/sewer_grate.tga: a drain grate in the floor (tools/art/grate.ase: still,
  eyes in the dark, a rat's snout through the bars), 24 frames of 16x16, a quarter second each:
  mostly still, now and then eyes, then a snout.
- assets/textures/ingame/valve_steam.tga: steam puffing out of a pipe's valve (tools/art/steam.ase,
  four frames of a puff), 24 frames of 24x16, mostly nothing, then a puff.

    uv run -q --with pillow python tools/make_sewer_props.py
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
ART = ROOT / "tools/art"
OUT = ROOT / "assets/textures/ingame"

GRATE_TIMELINE = [0] * 12 + [1, 1, 1, 0, 1, 1, 2, 2, 2, 2, 1, 0]
STEAM_TIMELINE = [None] * 20 + [0, 1, 2, 3]


def frames(name, width):
    art = Image.open(ART / f"{name}.png").convert("RGBA")
    return [art.crop((i * width, 0, (i + 1) * width, art.height)) for i in range(art.width // width)]


def timeline(pictures, order, size):
    sheet = Image.new("RGBA", (size[0] * len(order), size[1]), (0, 0, 0, 0))
    for index, picture in enumerate(order):
        if picture is not None:
            sheet.paste(pictures[picture], (index * size[0], 0))
    return sheet


def main():
    timeline(frames("grate", 16), GRATE_TIMELINE, (16, 16)).save(OUT / "sewer_grate.tga")
    timeline(frames("steam", 24), STEAM_TIMELINE, (24, 16)).save(OUT / "valve_steam.tga")


if __name__ == "__main__":
    main()
