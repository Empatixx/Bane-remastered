"""What is left when a shot brings a hanging thing down, from the pictures drawn by hand in
LibreSprite (tools/art, exported to PNG with
`lsprite render tools/art/<name>.ase --scale 1 --out tools/art/<name>.png`):

- assets/textures/ingame/icicle_crash.tga: an icicle striking the floor (tools/art/icicle_crash.ase):
  a white flash, shards flying out and falling back, a frost puff, the shards melting; 6 frames
  of 32x32.
- assets/textures/ingame/cocoon_burst.tga: a cocoon landing (tools/art/cocoon_burst.ase): squashed,
  split open with its goo splashing, and what stays on the floor, two torn halves, silk threads
  and a puddle of goo; 3 frames of 24x16.

    uv run -q --with pillow python tools/make_hanging_drops.py
"""

from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent

for name in ("icicle_crash", "cocoon_burst"):
    Image.open(ROOT / "tools/art" / f"{name}.png").convert("RGBA").save(ROOT / "assets/textures/ingame" / f"{name}.tga")
