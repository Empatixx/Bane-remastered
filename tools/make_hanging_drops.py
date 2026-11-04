"""What is left when a shot brings a hanging thing down, from the pictures drawn by hand in
LibreSprite (tools/art, exported to PNG with
`lsprite render tools/art/<name>.ase --scale 1 --out tools/art/<name>.png`):

- assets/textures/ingame/icicle_crash.tga: an icicle striking the floor (tools/art/icicle_crash.ase):
  a white flash, shards flying out and falling back, a frost puff, the shards melting; 6 frames
  of 32x32.
- assets/textures/ingame/cocoon_burst.tga: a cocoon landing (tools/art/cocoon_burst.ase): split open
  with its goo splashing, and what stays on the floor, two torn halves, silk threads and a
  puddle of goo; 2 frames of 32x24.
- assets/textures/ingame/cocoon_fall.tga and icicle_fall.tga: the thing shot down on its way to
  the floor (tools/art/cocoon_fall.ase, icicle_fall.ase): a flash where the shot struck, the
  thread snapping or the ice cracking, then falling (the cocoon tumbling over), with speed lines;
  4 frames of 32x32, the thing where it hung in the hanging cell's middle.

    uv run -q --with pillow python tools/make_hanging_drops.py
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent

for name in ("icicle_crash", "cocoon_burst", "cocoon_fall", "icicle_fall"):
    Image.open(ROOT / "tools/art" / f"{name}.png").convert("RGBA").save(ROOT / "assets/textures/ingame" / f"{name}.tga")
