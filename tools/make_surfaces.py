import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
T = 16
VARIANTS = 3
UP, RIGHT, DOWN, LEFT = 1, 2, 4, 8
DIAGONALS = [(16, UP, RIGHT, (T - 1, 0)), (32, RIGHT, DOWN, (T - 1, T - 1)), (64, DOWN, LEFT, (0, T - 1)), (128, LEFT, UP, (0, 0))]


def reduce(code):
    out = code & 15
    for bit, a, b, _ in DIAGONALS:
        if code & bit and code & a and code & b:
            out |= bit
    return out


VALID = [code for code in range(256) if reduce(code) == code]


def rim_distance(x, y, code):
    distances = []
    if not code & UP:
        distances.append(y)
    if not code & DOWN:
        distances.append(T - 1 - y)
    if not code & LEFT:
        distances.append(x)
    if not code & RIGHT:
        distances.append(T - 1 - x)
    for bit, a, b, (cx, cy) in DIAGONALS:
        if code & a and code & b and not code & bit:
            distances.append(max(abs(x - cx), abs(y - cy)))
    return min(distances) if distances else 99


def corner_cut(x, y, code):
    cuts = ((not code & UP) and (not code & LEFT) and x + y < 2,
            (not code & UP) and (not code & RIGHT) and (T - 1 - x) + y < 2,
            (not code & DOWN) and (not code & LEFT) and x + (T - 1 - y) < 2,
            (not code & DOWN) and (not code & RIGHT) and (T - 1 - x) + (T - 1 - y) < 2)
    return any(cuts)


def make_sheet(frames, painter):
    sheet = Image.new("RGBA", (T * frames, T * len(VALID) * VARIANTS), (0, 0, 0, 0))
    for rank, code in enumerate(VALID):
        for variant in range(VARIANTS):
            for frame in range(frames):
                tile = Image.new("RGBA", (T, T), (0, 0, 0, 0))
                for y in range(T):
                    for x in range(T):
                        if corner_cut(x, y, code):
                            continue
                        colour = painter(x, y, code, frame, rim_distance(x, y, code), variant)
                        if colour:
                            tile.putpixel((x, y), colour)
                sheet.paste(tile, (frame * T, (rank * VARIANTS + variant) * T))
    return sheet


SHORE_CODES = 20
SHORE_VARIANTS = 2


def shore_distance(x, y, code):
    if code >= 16:
        _, _, _, (cx, cy) = DIAGONALS[code - 16]
        return max(abs(x - cx), abs(y - cy))
    distances = []
    if code & UP:
        distances.append(y)
    if code & DOWN:
        distances.append(T - 1 - y)
    if code & LEFT:
        distances.append(x)
    if code & RIGHT:
        distances.append(T - 1 - x)
    return min(distances) if distances else 99


def make_shore(painter):
    sheet = Image.new("RGBA", (T, T * SHORE_CODES * SHORE_VARIANTS), (0, 0, 0, 0))
    for code in range(SHORE_CODES):
        for variant in range(SHORE_VARIANTS):
            tile = Image.new("RGBA", (T, T), (0, 0, 0, 0))
            for y in range(T):
                for x in range(T):
                    colour = painter(x, y, shore_distance(x, y, code), variant)
                    if colour:
                        tile.putpixel((x, y), colour)
            sheet.paste(tile, (0, (code * SHORE_VARIANTS + variant) * T))
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
        return (150, 170, 84, 255) if noise(x, y, v + frame) > 0.4 else (112, 138, 62, 255)
    if d == 1:
        return (78, 104, 46, 255)
    n = smooth_noise(x + v * 6 + frame * 2, y + v * 4, 31 + v)
    if n > 0.8:
        return (132, 150, 72, 255)
    if n > 0.74:
        return (104, 124, 56, 255)
    if n > 0.42:
        return (62, 88, 38, 255)
    if noise(x, y, 120 + v) > 0.985:
        return (96, 74, 40, 255)
    return (46, 68, 32, 255)


def alpha_by(distance, base, reach):
    return max(0, int(base * (1 - distance / reach)))


def water_shore(x, y, d, v):
    if d >= 4:
        return None
    if d <= 1 and noise(x, y, 90 + v) > 0.6:
        return (150, 164, 80, 160)
    return (36, 52, 28, alpha_by(d, 160, 4))


def ice_shore(x, y, d, v):
    if d >= 3:
        return None
    if d <= 1 and noise(x, y, 95 + v) > 0.5:
        return (236, 250, 255, 190)
    return (190, 226, 248, alpha_by(d, 130, 3))


def lava_shore(x, y, d, v):
    if d >= 4:
        return None
    if d <= 1 and noise(x, y, 99 + v) > 0.82:
        return (255, 120, 30, 220)
    return (24, 8, 8, alpha_by(d, 190, 4))


def shade_tile():
    tile = Image.new("RGBA", (T, T), (0, 0, 0, 0))
    for y in range(7):
        for x in range(T):
            tile.putpixel((x, y), (0, 0, 0, int(150 * (1 - y / 7))))
    return tile


RIPPLE_FRAMES = 6
RIPPLE_SIZE = 32


def ripple_sheet():
    sheet = Image.new("RGBA", (RIPPLE_SIZE * RIPPLE_FRAMES, RIPPLE_SIZE), (0, 0, 0, 0))
    centre = (RIPPLE_SIZE - 1) / 2
    for frame in range(RIPPLE_FRAMES):
        radius = 3 + frame * 2.6
        fade = frame / (RIPPLE_FRAMES - 1)
        for y in range(RIPPLE_SIZE):
            for x in range(RIPPLE_SIZE):
                dx, dy = (x - centre), (y - centre) / 0.78
                distance = math.hypot(dx, dy)
                for ring, offset in enumerate((0.0, -3.2)):
                    r = radius + offset
                    if r < 1.5:
                        continue
                    if abs(distance - r) < 0.75:
                        keep = noise(x, y, 200 + frame) > fade * (0.9 if ring == 0 else 1.15)
                        if keep:
                            colour = (186, 204, 112, 255) if ring == 0 else (130, 154, 76, 255)
                            sheet.putpixel((frame * RIPPLE_SIZE + x, y), colour)
    return sheet


def main():
    ripple_sheet().save(OUT / "ripple.tga")
    make_shore(water_shore).save(OUT / "shore_water.tga")
    make_shore(ice_shore).save(OUT / "shore_ice.tga")
    make_shore(lava_shore).save(OUT / "shore_lava.tga")
    shade_tile().save(OUT / "surface_shade.tga")
    make_sheet(1, ice).save(OUT / "surface_ice.tga")
    make_sheet(4, lava).save(OUT / "surface_lava.tga")
    make_sheet(4, water).save(OUT / "surface_water.tga")


main()
