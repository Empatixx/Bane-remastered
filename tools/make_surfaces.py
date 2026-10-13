import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
T = 16
MASKS = 16
VARIANTS = 3
UP, RIGHT, DOWN, LEFT = 1, 2, 4, 8


def inside_distance(x, y, mask):
    distances = []
    if not mask & UP:
        distances.append(y)
    if not mask & DOWN:
        distances.append(T - 1 - y)
    if not mask & LEFT:
        distances.append(x)
    if not mask & RIGHT:
        distances.append(T - 1 - x)
    return min(distances) if distances else 99


def corner_cut(x, y, mask):
    cuts = ((not mask & UP) and (not mask & LEFT) and x + y < 2,
            (not mask & UP) and (not mask & RIGHT) and (T - 1 - x) + y < 2,
            (not mask & DOWN) and (not mask & LEFT) and x + (T - 1 - y) < 2,
            (not mask & DOWN) and (not mask & RIGHT) and (T - 1 - x) + (T - 1 - y) < 2)
    return any(cuts)


def make_sheet(frames, painter):
    sheet = Image.new("RGBA", (T * frames, T * MASKS * VARIANTS), (0, 0, 0, 0))
    for mask in range(MASKS):
        for variant in range(VARIANTS):
            for frame in range(frames):
                tile = Image.new("RGBA", (T, T), (0, 0, 0, 0))
                for y in range(T):
                    for x in range(T):
                        if corner_cut(x, y, mask):
                            continue
                        colour = painter(x, y, mask, frame, inside_distance(x, y, mask), variant)
                        if colour:
                            tile.putpixel((x, y), colour)
                sheet.paste(tile, (frame * T, (mask * VARIANTS + variant) * T))
    return sheet


def noise(x, y, seed):
    value = (x * 374761393 + y * 668265263 + seed * 2147483647) & 0xFFFFFFFF
    value = ((value ^ (value >> 13)) * 1274126177) & 0xFFFFFFFF
    return ((value ^ (value >> 16)) & 0xFFFF) / 65535.0


def smooth_noise(x, y, seed):
    fx, fy = x / 4.0, y / 4.0
    x0, y0 = int(math.floor(fx)), int(math.floor(fy))
    tx, ty = fx - x0, fy - y0
    tx, ty = tx * tx * (3 - 2 * tx), ty * ty * (3 - 2 * ty)
    a = noise(x0, y0, seed) * (1 - tx) + noise(x0 + 1, y0, seed) * tx
    b = noise(x0, y0 + 1, seed) * (1 - tx) + noise(x0 + 1, y0 + 1, seed) * tx
    return a * (1 - ty) + b * ty


def ice(x, y, mask, frame, d, v):
    if d == 0:
        return (226, 246, 255, 255)
    if d == 1:
        return (170, 214, 240, 255)
    n = smooth_noise(x + v * 7, y + v * 3, 11 + v)
    base = (118, 172, 214, 255) if n < 0.45 else (132, 184, 224, 255) if n < 0.75 else (150, 198, 232, 255)
    if noise(x, y, 40 + v) > 0.97:
        base = (226, 246, 255, 255)
    streak = (x * 2 + y + v * 5) % 23
    if streak == 0 and 2 < d:
        base = (190, 226, 246, 255)
    return base


def lava(x, y, mask, frame, d, v):
    if d == 0:
        return (48, 22, 24, 255)
    if d == 1:
        return (96, 36, 24, 255) if noise(x, y, v) > 0.5 else (140, 48, 22, 255)
    n = smooth_noise(x + v * 9 + frame * 2, y + v * 5 - frame, 21 + v)
    if n > 0.78:
        return (255, 214, 96, 255)
    if n > 0.58:
        return (255, 140, 40, 255)
    if n > 0.3:
        return (222, 82, 28, 255)
    return (170, 44, 22, 255)


def water(x, y, mask, frame, d, v):
    if d == 0:
        return (150, 196, 190, 255) if noise(x, y, v + frame) > 0.4 else (110, 160, 160, 255)
    if d == 1:
        return (74, 130, 140, 255)
    n = smooth_noise(x + v * 6 + frame * 2, y + v * 4, 31 + v)
    if n > 0.74:
        return (118, 184, 182, 255)
    if n > 0.42:
        return (60, 116, 128, 255)
    return (44, 92, 106, 255)


def main():
    make_sheet(1, ice).save(OUT / "surface_ice.tga")
    make_sheet(4, lava).save(OUT / "surface_lava.tga")
    make_sheet(4, water).save(OUT / "surface_water.tga")


main()
