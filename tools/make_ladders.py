import random
import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/ingame"

HOLE = (37, 19, 26, 255)
FRAME_A = (54, 32, 48, 255)
FRAME_B = (61, 37, 59, 255)
RAIL = (137, 90, 69, 255)
RAIL_L = (191, 112, 77, 255)
SHADOW = (82, 59, 64, 255)
BOLT = (144, 145, 158, 255)
BOLT_L = (173, 193, 207, 255)


def rgba(r, g, b):
    return (r, g, b, 255)


THEMES = {
    "sewer": {
        "map": {HOLE: rgba(16, 26, 22), FRAME_A: rgba(43, 60, 54), FRAME_B: rgba(49, 68, 62), RAIL: rgba(112, 78, 58), RAIL_L: rgba(160, 104, 70), SHADOW: rgba(36, 50, 46), BOLT: rgba(150, 128, 70), BOLT_L: rgba(200, 176, 100)},
        "decor": "slime",
    },
    "moss": {
        "map": {HOLE: rgba(22, 24, 16), FRAME_A: rgba(50, 52, 47), FRAME_B: rgba(57, 59, 53), RAIL: rgba(108, 84, 54), RAIL_L: rgba(148, 114, 70), SHADOW: rgba(44, 46, 38), BOLT: rgba(130, 130, 120), BOLT_L: rgba(170, 176, 160)},
        "decor": "moss",
    },
    "spider": {
        "map": {HOLE: rgba(14, 10, 16), FRAME_A: rgba(37, 33, 43), FRAME_B: rgba(42, 38, 49), RAIL: rgba(158, 150, 130), RAIL_L: rgba(206, 198, 174), SHADOW: rgba(30, 26, 36), BOLT: rgba(120, 112, 132), BOLT_L: rgba(170, 164, 182)},
        "decor": "web",
    },
    "ice": {
        "map": {HOLE: rgba(14, 24, 40), FRAME_A: rgba(47, 67, 101), FRAME_B: rgba(53, 77, 115), RAIL: rgba(136, 186, 220), RAIL_L: rgba(200, 232, 248), SHADOW: rgba(36, 54, 84), BOLT: rgba(220, 238, 250), BOLT_L: rgba(255, 255, 255)},
        "decor": "frost",
    },
    "lava": {
        "map": {HOLE: rgba(40, 10, 8), FRAME_A: rgba(56, 33, 33), FRAME_B: rgba(64, 38, 38), RAIL: rgba(74, 62, 66), RAIL_L: rgba(120, 104, 108), SHADOW: rgba(30, 16, 16), BOLT: rgba(226, 100, 40), BOLT_L: rgba(255, 170, 70)},
        "decor": "lava",
    },
    "temple": {
        "map": {HOLE: rgba(36, 24, 18), FRAME_A: rgba(80, 67, 54), FRAME_B: rgba(92, 77, 62), RAIL: rgba(190, 150, 66), RAIL_L: rgba(242, 208, 112), SHADOW: rgba(60, 48, 38), BOLT: rgba(70, 150, 140), BOLT_L: rgba(130, 214, 196)},
        "decor": "carving",
    },
}

SLIME = rgba(86, 124, 60)
SLIME_L = rgba(130, 170, 84)
MOSS = rgba(78, 112, 54)
MOSS_L = rgba(118, 150, 76)
WEB = rgba(214, 214, 190)
WEB_D = rgba(150, 150, 132)
FROST = rgba(230, 246, 255)
FROST_D = rgba(160, 206, 236)
CRACK = rgba(240, 110, 36)
CRACK_L = rgba(255, 190, 80)
GOLD_LINE = rgba(214, 178, 96)
GOLD_D = rgba(150, 118, 60)


def frame_pixels(base):
    return [(x, y) for y in range(32) for x in range(32) if base.getpixel((x, y)) in (FRAME_A, FRAME_B)]


def hole_edge(base):
    edge = []
    for y in range(32):
        for x in range(32):
            if base.getpixel((x, y)) not in (FRAME_A, FRAME_B):
                continue
            for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + ox, y + oy
                if 0 <= nx < 32 and 0 <= ny < 32 and base.getpixel((nx, ny)) == HOLE:
                    edge.append((x, y))
                    break
    return edge


def decorate(image, base, kind):
    rng = random.Random(7)
    frame = frame_pixels(base)
    if kind == "slime":
        for x, y in hole_edge(base):
            if rng.random() < 0.5:
                for step in range(rng.randint(1, 3)):
                    if image.getpixel((x, min(31, y + step))) in [v for v in THEMES["sewer"]["map"].values()][1:3]:
                        image.putpixel((x, min(31, y + step)), SLIME if step else SLIME_L)
    elif kind == "moss":
        for _ in range(7):
            cx, cy = rng.choice(frame)
            for dx in range(-2, 3):
                for dy in range(-1, 2):
                    p = (cx + dx, cy + dy)
                    if p in frame and abs(dx) + abs(dy) < 3:
                        image.putpixel(p, MOSS_L if dy < 0 else MOSS)
        for y in range(4, 28, 5):
            for x in (8, 9, 22, 23):
                image.putpixel((x, y), MOSS)
    elif kind == "web":
        for corner in ((0, 0, 1, 1), (31, 0, -1, 1), (0, 31, 1, -1), (31, 31, -1, -1)):
            cx, cy, sx, sy = corner
            for step in range(7):
                for pos in ((cx + sx * step, cy), (cx, cy + sy * step), (cx + sx * step, cy + sy * step)):
                    if 0 <= pos[0] < 32 and 0 <= pos[1] < 32 and pos in frame:
                        image.putpixel(pos, WEB if step % 2 == 0 else WEB_D)
            for arc in (3, 5):
                for t in range(arc + 1):
                    pos = (cx + sx * t, cy + sy * (arc - t))
                    if pos in frame:
                        image.putpixel(pos, WEB_D)
    elif kind == "frost":
        for x, y in frame:
            if (x * 7 + y * 3) % 11 == 0:
                image.putpixel((x, y), FROST)
            elif (x * 5 + y * 9) % 17 == 0:
                image.putpixel((x, y), FROST_D)
        for x, y in hole_edge(base):
            if (x + y) % 3 == 0:
                image.putpixel((x, y), FROST_D)
    elif kind == "lava":
        for cx, cy in rng.sample(frame, 5):
            x, y = cx, cy
            for _ in range(rng.randint(3, 6)):
                if (x, y) in frame:
                    image.putpixel((x, y), CRACK_L if rng.random() < 0.25 else CRACK)
                x += rng.choice((-1, 0, 1))
                y += rng.choice((0, 1))
        for x, y in hole_edge(base):
            if (x * 3 + y) % 4 == 0:
                image.putpixel((x, y), CRACK)
    elif kind == "carving":
        for x, y in frame:
            if (x in (1, 30) or y in (1, 30)) and 1 <= x <= 30 and 1 <= y <= 30:
                image.putpixel((x, y), GOLD_LINE)
        for x, y in ((3, 3), (4, 3), (3, 4), (28, 3), (27, 3), (28, 4), (3, 28), (4, 28), (3, 27), (28, 28), (27, 28), (28, 27)):
            if (x, y) in frame:
                image.putpixel((x, y), GOLD_D)


def main():
    base = Image.open(ROOT / "assets/textures/ingame/ladder.tga").convert("RGBA")
    for name, spec in THEMES.items():
        image = Image.new("RGBA", base.size)
        for y in range(32):
            for x in range(32):
                pixel = base.getpixel((x, y))
                image.putpixel((x, y), spec["map"].get(pixel, pixel))
        decorate(image, base, spec["decor"])
        image.save(OUT / f"ladder_{name}.tga")


main()
