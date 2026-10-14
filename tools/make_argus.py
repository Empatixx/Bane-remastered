import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
OUTLINE = (37, 19, 26, 255)
CLOAK_D = (49, 30, 36, 255)
CLOAK = (61, 35, 44, 255)
CLOAK_L = (107, 55, 75, 255)
TRIM = (162, 43, 97, 255)
EYE_D = (191, 99, 38, 255)
EYE = (225, 112, 40, 255)
EYE_M = (225, 130, 62, 255)
EYE_L = (250, 157, 81, 255)
EYE_HL = (244, 194, 126, 255)
SIZE = 64


BURST = (255, 232, 150, 255)
BURST_D = (250, 157, 81, 255)
SMALL = ((17, 30, 3), (49, 29, 3), (21, 44, 2), (44, 45, 2), (30, 48, 2), (36, 40, 2))
FRAMES_PER_ROW = 8


def body_mask(rng_seed, sag=0.0, wave=0.0, bob=0):
    rng = random.Random(rng_seed)
    mask = [[False] * SIZE for _ in range(SIZE)]
    cy = 24 + 16 * sag + bob
    ry = 20.0 * (1.0 - 0.5 * sag)
    neck = cy + ry * 0.8
    for y in range(SIZE):
        for x in range(SIZE):
            dx = (x - 32) / (21.0 + 7.0 * sag)
            if y < neck:
                dy = (y - cy) / ry
                inside = dx * dx + dy * dy < 1.0
            else:
                width = 1.0 - (y - neck) / (40.0 + 20.0 * sag)
                inside = abs(dx) < 0.95 * width and y < SIZE - 1 - 2 * sag * 10
            mask[y][x] = inside
    base = int(neck)
    for x0 in range(14, 51, 4):
        length = int(rng.randint(6, 20) * (1.0 - 0.8 * sag))
        shift = int(round(math.sin(x0 * 0.5 + wave) * 1.5))
        for y in range(base, min(SIZE, base + length)):
            xx = min(SIZE - 2, max(0, x0 + (shift if y > base + 6 else 0)))
            mask[y][xx] = True
            mask[y][xx + 1] = mask[y][xx + 1] or rng.random() < 0.5
    return mask, cy


def mix(colour, towards, amount):
    return tuple(round(c + (t - c) * amount) for c, t in zip(colour, towards[:3] + (255,)))


def eye(image, cx, cy, radius, look=(1, 0), glow=0.0, slit=False, lid=0.0, shut=False):
    if shut:
        for x in range(cx - radius, cx + radius + 1):
            image.putpixel((x, cy), OUTLINE)
        return
    palette = [mix(c, EYE_HL, glow) for c in (EYE_D, EYE, EYE_M, EYE_L)]
    for y in range(cy - radius - 1, cy + radius + 2):
        for x in range(cx - radius - 1, cx + radius + 2):
            d = math.hypot(x - cx, y - cy)
            if d > radius + 0.5:
                continue
            if lid and (y - cy) < -radius * (1.0 - 2.0 * lid):
                image.putpixel((x, y), CLOAK_D if d <= radius - 0.8 else OUTLINE)
                continue
            if d > radius - 0.8:
                colour = palette[0]
            else:
                light = (-(x - cx) - (y - cy)) / (2.0 * radius)
                colour = palette[3] if light > 0.45 else palette[2] if light > 0.0 else palette[1]
            image.putpixel((x, y), colour)
    px, py = cx + look[0] * (radius // 3), cy + look[1] * (radius // 3)
    pupil = max(1, radius // 4)
    height = pupil * 3 if slit else pupil
    width = 0 if slit else pupil
    for y in range(py - height, py + height + 1):
        for x in range(px - width, px + width + 1):
            if 0 <= y < SIZE and not (lid and (y - cy) < -radius * (1.0 - 2.0 * lid)):
                image.putpixel((x, y), OUTLINE)
    if not slit and not lid:
        image.putpixel((cx - radius // 2, cy - radius // 2), EYE_HL)
        if radius >= 4:
            image.putpixel((cx - radius // 2 + 1, cy - radius // 2), EYE_HL)


def cloak(mask):
    image = Image.new("RGBA", (SIZE, SIZE), CLEAR)
    for y in range(SIZE):
        for x in range(SIZE):
            if mask[y][x]:
                light = (-(x - 32) * 0.6 - (y - 24) * 0.4) / 20.0
                image.putpixel((x, y), CLOAK_L if light > 0.55 else CLOAK if light > -0.15 else CLOAK_D)
    for x in range(SIZE):
        for y in range(SIZE):
            if mask[y][x]:
                continue
            for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + ox, y + oy
                if 0 <= nx < SIZE and 0 <= ny < SIZE and mask[ny][nx]:
                    image.putpixel((x, y), OUTLINE)
                    break
    return image


def trim(image, mask, dy):
    for x in range(15, 50):
        y = 38 + dy + int(2 * math.sin((x - 15) * 0.35))
        if 0 <= y < SIZE and mask[y][x]:
            image.putpixel((x, y), TRIM)


def socket(image, mask, cy):
    for y in range(SIZE):
        for x in range(SIZE):
            if mask[y][x] and math.hypot((x - 34) / 14.0, (y - (cy - 2)) / 15.0) < 1.0 and math.hypot((x - 34) / 12.0, (y - (cy - 2)) / 13.0) > 0.92:
                image.putpixel((x, y), CLOAK_D)


def argus(bob=0, wave=0.0, look=(1, 0), glow=0.0, slit=False, lid=0.0, small="open", flare=0):
    mask, cy = body_mask(11, bob=bob, wave=wave)
    image = cloak(mask)
    trim(image, mask, bob)
    socket(image, mask, int(cy))
    eye(image, 35, int(cy) - 1, 11, look, glow, slit, lid)
    for index, (cx, ey, radius) in enumerate(SMALL):
        eye(image, cx, ey + bob, radius + (flare if small == "flare" else 0), look,
            glow=1.0 if small == "flare" else 0.0, shut=small == "shut")
    return image


def crack(image, cx, cy, amount):
    rng = random.Random(5)
    for index in range(int(amount)):
        angle = index * math.tau / 7 + 0.4
        length = rng.randint(5, 11)
        for step in range(length):
            x = round(cx + math.cos(angle) * step + rng.choice((0, 0, 1)))
            y = round(cy + math.sin(angle) * step)
            if 0 <= x < SIZE and 0 <= y < SIZE and image.getpixel((x, y))[3]:
                image.putpixel((x, y), OUTLINE)


def starburst(image, cx, cy, reach):
    for index in range(12):
        angle = index * math.tau / 12
        for step in range(3, reach):
            x = round(cx + math.cos(angle) * step)
            y = round(cy + math.sin(angle) * step)
            if 0 <= x < SIZE and 0 <= y < SIZE:
                image.putpixel((x, y), BURST if step < reach - 3 else BURST_D)


def hollow(image, cx, cy, radius):
    for y in range(cy - radius, cy + radius + 1):
        for x in range(cx - radius, cx + radius + 1):
            d = math.hypot(x - cx, y - cy)
            if d <= radius + 0.3 and 0 <= x < SIZE and 0 <= y < SIZE:
                image.putpixel((x, y), OUTLINE if d <= radius - 0.8 else CLOAK_D)


def crack_eye(image, cx, cy, radius, amount, seed):
    rng = random.Random(seed)
    for index in range(amount):
        angle = index * math.tau / amount + rng.uniform(0.0, 0.8)
        length = max(2, int(radius * rng.uniform(0.7, 1.1)))
        for step in range(length):
            x = round(cx + math.cos(angle) * step + (rng.choice((0, 0, 1)) if radius > 4 else 0))
            y = round(cy + math.sin(angle) * step)
            if 0 <= x < SIZE and 0 <= y < SIZE:
                image.putpixel((x, y), OUTLINE)


def burst_eye(image, cx, cy, reach):
    hollow(image, cx, cy, max(1, reach // 5))
    for index in range(8):
        angle = index * math.tau / 8 + 0.2
        for step in range(max(2, reach // 4), reach):
            x = round(cx + math.cos(angle) * step)
            y = round(cy + math.sin(angle) * step)
            if 0 <= x < SIZE and 0 <= y < SIZE:
                image.putpixel((x, y), BURST if step < reach - 2 else BURST_D)


def death(frame):
    if frame <= 3:
        image = argus(glow=0.35 + 0.2 * frame, small="flare", flare=frame // 2)
        if frame == 0:
            return image
        crack_eye(image, 35, 23, 11, 4 + 3 * frame, 5)
        for index, (cx, ey, radius) in enumerate(SMALL):
            if frame == 1 or index % 2:
                crack_eye(image, cx, ey, radius + frame // 2, 3, index)
            else:
                burst_eye(image, cx, ey, 7)
        if frame == 3:
            for index, (cx, ey, radius) in enumerate(SMALL):
                burst_eye(image, cx, ey, 8)
            burst_eye(image, 35, 23, 26)
        return image
    image = argus(small="shut")
    mask, _ = body_mask(11)
    base = cloak(mask)
    hollow(base, 35, 23, 11)
    for cx, ey, radius in SMALL:
        hollow(base, cx, ey, radius)
    cells, spread, fall, keep = ((7, 5, 0, 1.0), (10, 12, 3, 0.85), (14, 19, 10, 0.6), (18, 24, 24, 0.45))[frame - 4]
    image = shatter(base, cells, spread, fall, keep, 40 + frame)
    if frame == 4:
        for index in range(10):
            angle = index * math.tau / 10 + 0.15
            for step in range(2, 13):
                x, y = round(32 + math.cos(angle) * step), round(26 + math.sin(angle) * step)
                if 0 <= x < SIZE and 0 <= y < SIZE and not image.getpixel((x, y))[3]:
                    image.putpixel((x, y), BURST if step < 9 else BURST_D)
    return image


def shatter(base, cells, spread, fall, keep, seed):
    rng = random.Random(seed)
    seeds = [(rng.uniform(10, 54), rng.uniform(4, 58)) for _ in range(cells)]
    kept = [rng.random() < keep or index < 3 for index in range(cells)]
    shifts = []
    for sx, sy in seeds:
        dx, dy = sx - 32, sy - 28
        length = math.hypot(dx, dy) or 1.0
        power = spread * rng.uniform(0.6, 1.2)
        shifts.append((round(dx / length * power), round(dy / length * power * 0.7 + fall * rng.uniform(0.7, 1.1))))
    out = Image.new("RGBA", (SIZE, SIZE), CLEAR)
    for y in range(SIZE):
        for x in range(SIZE):
            colour = base.getpixel((x, y))
            if colour[3] == 0:
                continue
            index = min(range(cells), key=lambda i: (x - seeds[i][0]) ** 2 + (y - seeds[i][1]) ** 2)
            if not kept[index]:
                continue
            if colour == OUTLINE:
                colour = CLOAK_D
            dx, dy = shifts[index]
            nx, ny = x + dx, min(y + dy, SIZE - 3)
            if 1 <= nx < SIZE - 1 and 1 <= ny < SIZE - 1:
                out.putpixel((nx, ny), colour)
    result = out.copy()
    for y in range(SIZE):
        for x in range(SIZE):
            if out.getpixel((x, y))[3]:
                continue
            for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + ox, y + oy
                if 0 <= nx < SIZE and 0 <= ny < SIZE and out.getpixel((nx, ny))[3]:
                    result.putpixel((x, y), OUTLINE)
                    break
    return result


def main():
    sheet = Image.new("RGBA", (SIZE * FRAMES_PER_ROW, SIZE * 4), CLEAR)
    rows = [
        [argus(bob=b, wave=w * 0.9, look=look) for w, (b, look) in enumerate(((0, (1, 0)), (1, (1, 0)), (1, (1, 1)), (0, (1, 0)), (-1, (1, 0)), (-1, (1, -1))))],
        [argus(glow=g, slit=g > 0.3, small="shut", look=(1, 0), wave=g * 3) for g in (0.0, 0.15, 0.3, 0.5, 0.75, 1.0)],
        [argus(lid=0.15 if f % 2 else 0.0, small="flare", flare=f % 2, wave=f) for f in range(6)],
        [death(f) for f in range(8)],
    ]
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * SIZE, row * SIZE))
    sheet.save(OUT / "argus.tga")


main()
