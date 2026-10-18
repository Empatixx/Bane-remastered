"""The hero's walk to the side (player.tga, row 1), with the body moving: the head and torso dip
on the step down and lift on the pass while the feet stay planted, so he no longer slides along
on stiff legs with one frame that jumps.

The drawn frames stay as they are (legs and the swinging arm). The passing frame that was drawn a
pixel higher is brought back in line, and then each frame's upper body is moved by the bob:

    frame   0 contact  1 down  2 pass  3 contact  4 down  5 pass
    bob        0          +1      -1       0         +1      -1

On the lift the hips stretch by a row copied from the top of the legs. The feet never move.
It works on the frames as drawn, so it is run once; run again it would bob them twice.

    uv run -q --with pillow python tools/make_side_walk.py [preview.gif]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
SHEET = ROOT / "assets/textures/ingame/player.tga"
W, H = 13, 31
ROW = 1
FRAMES = 6
# The rows from the hair down to the belt, and the legs below it.
UPPER = range(0, 25)
LEGS = range(25, H)
BOB = [0, 1, -1, 0, 1, -1]
# The frame drawn one pixel higher than the rest, with its legs a row longer.
RAISED = 2


def frame(sheet, index):
    return sheet.crop((index * W, ROW * H, (index + 1) * W, (ROW + 1) * H))


def level(cell, raised):
    """`cell` with its upper body on the same rows as the other frames."""
    if not raised:
        return cell
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    out.paste(cell.crop((0, 0, W, 24)), (0, 1))
    out.paste(cell.crop((0, 25, W, H)), (0, 25))
    # Its belt row (24 once lowered) comes from the row it was drawn on.
    out.paste(cell.crop((0, 23, W, 24)), (0, 24))
    return out


def bob(cell, offset):
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    out.paste(cell.crop((0, LEGS.start, W, H)), (0, LEGS.start))
    if offset < 0:
        # Lifted: the hips stretch by the top row of the legs.
        out.paste(cell.crop((0, LEGS.start, W, LEGS.start + 1)), (0, UPPER.stop - 1))
    upper = cell.crop((0, 0, W, UPPER.stop))
    if offset >= 0:
        out.alpha_composite(upper, (0, offset))
    else:
        out.alpha_composite(upper.crop((0, -offset, W, UPPER.stop)), (0, 0))
    return out


def walk(sheet):
    return [bob(level(frame(sheet, index), index == RAISED), BOB[index]) for index in range(FRAMES)]


def main():
    sheet = Image.open(SHEET).convert("RGBA")
    frames = walk(sheet)
    if len(sys.argv) > 1:
        scale = 10
        pictures = []
        for index in range(FRAMES * 3):
            before = frame(sheet, index % FRAMES)
            after = frames[index % FRAMES]
            picture = Image.new("RGBA", (W * 2 + 4, H), (30, 28, 36, 255))
            picture.alpha_composite(before, (0, 0))
            picture.alpha_composite(after, (W + 4, 0))
            pictures.append(picture.resize((picture.width * scale, picture.height * scale), Image.NEAREST))
        pictures[0].save(sys.argv[1], save_all=True, append_images=pictures[1:], duration=100, loop=0, disposal=2)
        return
    for index, cell in enumerate(frames):
        box = (index * W, ROW * H, (index + 1) * W, (ROW + 1) * H)
        sheet.paste(Image.new("RGBA", (W, H), (0, 0, 0, 0)), box)
        sheet.alpha_composite(cell, (index * W, ROW * H))
    sheet.save(SHEET)


if __name__ == "__main__":
    main()
