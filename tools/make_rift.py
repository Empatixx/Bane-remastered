"""The wound's rift: a tear in the floor full of purple chaos, for the opening cinematic.

One 16x16 tile, four frames of the chaos churning, in sixteen rows: one for each set of open
sides (bit 1 north, 2 east, 4 south, 8 west), so a tile only draws a torn edge where the rift
does not go on. The churning is periodic over the tile, so tiles side by side join without a
seam.

    uv run -q --with pillow python tools/make_rift.py [preview.png]
"""

import math
import random
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
T = 16
FRAMES = 4

VOID = [(8, 3, 14), (16, 5, 28), (30, 8, 50)]
CHAOS = [(70, 16, 110), (120, 30, 170), (176, 64, 220), (230, 130, 255), (255, 214, 255)]
RIM = [(22, 14, 26), (38, 24, 44), (58, 40, 62)]
EDGE_GLOW = (200, 80, 240)

BAYER = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]


def churn(x, y, frame):
    """How bright the chaos is at (x, y): swirling bands periodic over the tile."""
    t = frame / FRAMES * math.tau
    u, v = x / T * math.tau, y / T * math.tau
    a = math.sin(u * 2 + math.sin(v + t) * 1.5 + t)
    b = math.sin(v * 2 - math.cos(u - t) * 1.5 - t * 1.0)
    c = math.sin(u + v + t * 2)
    return (a + b + 0.6 * c) / 2.6


def jag(side, i, depth_seed):
    """How deep the torn edge bites in at position i along a side, 1 or 2 pixels."""
    rng = random.Random(side * 97 + i * 31 + depth_seed)
    return 1 + (rng.random() < 0.35)


def tile(mask, frame):
    image = Image.new("RGBA", (T, T), (0, 0, 0, 0))
    px = image.load()
    # The rim's depth at every pixel along each closed side.
    rim = {}
    for side in range(4):
        if mask & (1 << side):
            continue
        for i in range(T):
            rim[(side, i)] = jag(side, i, mask)
    for y in range(T):
        for x in range(T):
            # Distance into the tile from each closed side, against that side's jagged depth.
            edge = None
            for side, along, into in ((0, x, y), (1, y, T - 1 - x), (2, x, T - 1 - y), (3, y, x)):
                if (side, along) in rim:
                    depth = rim[(side, along)]
                    if into < depth:
                        edge = ("rim", into, depth)
                        break
                    if into == depth:
                        edge = ("glow", into, depth)
            if edge and edge[0] == "rim":
                into, depth = edge[1], edge[2]
                px[x, y] = (*RIM[min(2, depth - 1 - into)], 255)
                continue
            if edge and edge[0] == "glow":
                px[x, y] = (*EDGE_GLOW, 255)
                continue
            level = churn(x, y, frame)
            if level < -0.35:
                colour = VOID[0]
            elif level < -0.05:
                colour = VOID[1] if BAYER[y % 4][x % 4] > 5 else VOID[0]
            elif level < 0.2:
                colour = VOID[2]
            elif level < 0.42:
                colour = CHAOS[0]
            elif level < 0.6:
                colour = CHAOS[1]
            elif level < 0.75:
                colour = CHAOS[2]
            elif level < 0.88:
                colour = CHAOS[3]
            else:
                colour = CHAOS[4]
            px[x, y] = (*colour, 255)
    # A few sparks, different every frame.
    rng = random.Random(mask * 13 + frame * 7)
    for _ in range(2):
        x, y = rng.randrange(3, T - 3), rng.randrange(3, T - 3)
        if px[x, y][:3] in VOID:
            px[x, y] = (*CHAOS[4], 255)
    return image


def main():
    sheet = Image.new("RGBA", (T * FRAMES, T * 16), (0, 0, 0, 0))
    for mask in range(16):
        for frame in range(FRAMES):
            sheet.paste(tile(mask, frame), (frame * T, mask * T))
    if len(sys.argv) > 1:
        # A preview: a little winding rift laid out of the tiles, frame 0, on the temple floor.
        layout = [(1, 1), (2, 1), (3, 1), (3, 2), (4, 2), (5, 2), (6, 2), (6, 3), (7, 3), (8, 3), (2, 2)]
        cells = set(layout)
        board = Image.new("RGBA", (T * 10, T * 5), (150, 110, 70, 255))
        for frame in range(FRAMES):
            pass
        for (cx, cy) in layout:
            mask = sum(bit for bit, (dx, dy) in ((1, (0, -1)), (2, (1, 0)), (4, (0, 1)), (8, (-1, 0))) if (cx + dx, cy + dy) in cells)
            board.alpha_composite(sheet.crop((0, mask * T, T, mask * T + T)), (cx * T, cy * T))
        strip = Image.new("RGBA", (T * FRAMES * 2 + 8, T * 2), (30, 28, 36, 255))
        for frame in range(FRAMES):
            strip.alpha_composite(sheet.crop((frame * T, 0, frame * T + T, T)), (frame * T * 2, 0))
            strip.alpha_composite(sheet.crop((frame * T, 15 * T, frame * T + T, 16 * T)), (frame * T * 2, T))
        out = Image.new("RGBA", (board.width, board.height + strip.height + 4), (30, 28, 36, 255))
        out.alpha_composite(board, (0, 0))
        out.alpha_composite(strip, (0, board.height + 4))
        out.resize((out.width * 6, out.height * 6), Image.NEAREST).save(sys.argv[1])
        return
    sheet.save(ROOT / "assets/textures/ingame/rift.tga")


if __name__ == "__main__":
    main()
