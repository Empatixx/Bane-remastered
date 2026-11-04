"""Writes the chibi people from tools/art/chibi_npcs.py, in the hero's proportions (20x26 frames):

- assets/textures/ingame/priest.tga and pilgrim.tga: 4x4 frames (stand, run, pray, asleep);
- assets/textures/ingame/elian.tga: Elian kneeling at prayer, 4x3 frames;
- assets/textures/ingame/elian_walk.tga: the hero's first four rows (stand, run to the side,
  run down, run up), for the cinematics;
- assets/textures/ingame/scientist.tga: 6x1 frames of idle;
- assets/textures/ingame/shopkeeper.tga: the merchant, 6x2 frames (idle, rubbing hands).

    uv run -q --with pillow python tools/make_chibi_npcs.py [--preview DIR]

With --preview it also writes, for each sheet, a 6x upscaled PNG on a dark background and one
animated GIF per row into DIR.
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "tools/art"))

import chibi_elian  # noqa: E402
import chibi_npcs as npcs  # noqa: E402

OUT = ROOT / "assets/textures/ingame"
W, H = npcs.W, npcs.H
COLUMNS = {"priest": 4, "pilgrim": 4, "elian": 4, "elian_walk": 6, "scientist": 6, "shopkeeper": 6}
# frames per row and seconds per frame, as the game plays them (for the previews)
CLIPS = {
    "priest": [(4, 0.3), (4, 0.14), (2, 0.6), (2, 1.1)],
    "pilgrim": [(4, 0.3), (4, 0.14), (2, 0.6), (2, 1.1)],
    "elian": [(4, 0.3), (4, 0.45), (4, 0.5)],
    "elian_walk": [(6, 0.12), (6, 0.07), (6, 0.07), (6, 0.07)],
    "scientist": [(6, 0.1)],
    "shopkeeper": [(6, 0.2), (6, 0.12)],
}


def image(rows, palette):
    out = Image.new("RGBA", (len(rows[0]), len(rows)), (0, 0, 0, 0))
    px = out.load()
    for y, line in enumerate(rows):
        for x, key in enumerate(line):
            if key != "." and palette.get(key):
                colour = palette[key].lstrip("#")
                px[x, y] = (int(colour[0:2], 16), int(colour[2:4], 16), int(colour[4:6], 16), 255)
    return out


def sheet(rows, palette, columns):
    out = Image.new("RGBA", (W * columns, H * len(rows)), (0, 0, 0, 0))
    for r, frames in enumerate(rows):
        for c, frame in enumerate(frames):
            out.alpha_composite(image(frame, palette), (c * W, r * H))
    return out


def sheets():
    out = {}
    for name, (palette, rows) in npcs.SHEETS.items():
        if name == "pilgrim":
            # no glint for the people: only the priests' prayers shine
            palette = dict(palette, y=None)
        out[name] = sheet(rows(), palette, COLUMNS[name])
    out["elian_walk"] = sheet(chibi_elian.ROWS[:4], chibi_elian.PAL, 6)
    return out


def preview(name, img, folder, scale=6):
    bg = (40, 36, 48, 255)
    big = img.resize((img.width * scale, img.height * scale), Image.NEAREST)
    board = Image.new("RGBA", big.size, bg)
    board.alpha_composite(big)
    board.save(folder / f"{name}.png")
    for r, (frames, seconds) in enumerate(CLIPS[name]):
        shots = []
        for c in range(frames):
            cell = img.crop((c * W, r * H, (c + 1) * W, (r + 1) * H))
            cell = cell.resize((W * scale, H * scale), Image.NEAREST)
            frame = Image.new("RGBA", cell.size, bg)
            frame.alpha_composite(cell)
            shots.append(frame.convert("RGB"))
        shots[0].save(folder / f"{name}_{r}.gif", save_all=True, append_images=shots[1:],
                      duration=int(seconds * 1000), loop=0)


def main():
    built = sheets()
    for name, img in built.items():
        img.save(OUT / f"{name}.tga")
    if "--preview" in sys.argv:
        folder = Path(sys.argv[sys.argv.index("--preview") + 1])
        folder.mkdir(parents=True, exist_ok=True)
        for name, img in built.items():
            preview(name, img, folder)


if __name__ == "__main__":
    main()
