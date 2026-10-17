"""The opening shot of the cinematic, worked in full: the temple on its hill at night, before the
wound. Drawn as pixel art at 320x180 in layers (sky, moon and clouds, far mountains in mist,
the hill with its winding stair, cypresses, the temple with its braziers, the grass in front)
and animated: stars twinkle, clouds drift, the braziers flicker, fireflies wander.

    uv run -q --with pillow python tools/make_intro_temple.py <out dir>

Writes temple_00.png .. temple_NN.png (the frames, 320x180), temple.gif (scaled four times)
and temple_still.png (one frame scaled four times).
"""

import math
import random
import sys
from pathlib import Path

from PIL import Image

W, H = 320, 180
FRAMES = 24

BAYER = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]

# The night: from the top of the sky down to the glow over the far hills.
SKY = [(6, 6, 18), (10, 9, 28), (16, 13, 40), (24, 18, 54), (36, 24, 68), (52, 32, 82), (72, 42, 94)]
MOON = [(120, 116, 128), (176, 170, 168), (214, 208, 196), (240, 236, 222), (252, 250, 240)]
HALO = [(30, 26, 58), (44, 38, 76), (60, 52, 96)]
CLOUD = [(30, 24, 52), (44, 36, 70), (70, 62, 98), (120, 112, 140)]
FAR = [(26, 22, 50), (34, 28, 62), (54, 48, 86)]
NEAR = [(18, 16, 36), (24, 20, 46), (40, 36, 68)]
MIST = [(52, 44, 84), (70, 62, 104)]
HILL = [(12, 12, 22), (18, 18, 32), (26, 26, 44), (38, 38, 60), (58, 58, 86)]
GRASS = [(16, 22, 30), (24, 34, 42), (40, 54, 62), (70, 88, 92)]
STONE = [(30, 28, 44), (52, 50, 70), (80, 78, 102), (118, 116, 140), (164, 162, 184), (210, 208, 222)]
FIRE = [(110, 30, 14), (200, 70, 20), (250, 140, 40), (255, 210, 110), (255, 246, 210)]
WARM = [(48, 28, 30), (90, 50, 38), (150, 90, 52)]
CYPRESS = [(10, 14, 20), (16, 22, 30), (30, 40, 50)]


def lerp_colour(a, b, t):
    return tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(3))


def ramp(px, x, y, colours, t):
    """The colour at `t` along `colours`, dithered between its steps at (x, y)."""
    steps = len(colours) - 1
    level = max(0.0, min(1.0, t)) * steps
    low = min(int(level), steps)
    if low < steps and (level - low) * 16 > BAYER[y % 4][x % 4]:
        low += 1
    return colours[low]


def blend_over(px, x, y, colour, amount):
    """Mixes `colour` over the pixel by `amount`, stepped by the dither so it stays crisp."""
    if not (0 <= x < W and 0 <= y < H):
        return
    if amount * 16 > BAYER[y % 4][x % 4]:
        px[x, y] = colour


def ridge(seed, base, height, roughness, step):
    """A jagged line across the panel: the top of a mountain range."""
    rng = random.Random(seed)
    points = []
    y = base
    for x in range(0, W + step, step):
        y += rng.uniform(-roughness, roughness)
        y = max(base - height, min(base + height * 0.3, y))
        points.append((x, y))
    line = []
    for x in range(W):
        i = x // step
        (x0, y0), (x1, y1) = points[i], points[i + 1]
        t = (x - x0) / step
        line.append(int(y0 + (y1 - y0) * (t * t * (3 - 2 * t))))
    return line


def draw_sky(px):
    for y in range(H):
        for x in range(W):
            px[x, y] = ramp(px, x, y, SKY, (y / 118) ** 1.25)


def draw_stars(px, frame):
    rng = random.Random(41)
    for i in range(170):
        x, y = rng.randrange(W), rng.randrange(0, 96)
        kind = rng.random()
        phase = rng.random() * math.tau
        twinkle = 0.5 + 0.5 * math.sin(frame / FRAMES * math.tau * (1 + i % 3) + phase)
        if kind > 0.96:
            core = (230, 228, 250) if twinkle > 0.3 else (160, 156, 190)
            px[x, y] = core
            if twinkle > 0.55:
                for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    if 0 <= x + dx < W and 0 <= y + dy < H:
                        px[x + dx, y + dy] = (96, 90, 140)
        elif kind > 0.8:
            px[x, y] = (170, 166, 200) if twinkle > 0.4 else (100, 96, 134)
        else:
            px[x, y] = lerp_colour(px[x, y], (120, 116, 160), 0.5 + 0.3 * twinkle)


def draw_moon(px, cx, cy, r):
    # The halo first, in dithered rings.
    for y in range(cy - r * 3, cy + r * 3):
        for x in range(cx - r * 3, cx + r * 3):
            if not (0 <= x < W and 0 <= y < H):
                continue
            d = math.hypot(x - cx, y - cy)
            if r < d < r * 3:
                t = 1 - (d - r) / (r * 2)
                if t > 0.05:
                    px[x, y] = ramp(px, x, y, [px[x, y]] + HALO, t ** 1.4 * 0.8)
    craters = [(-5, -4, 3), (4, 3, 4), (-2, 7, 2), (6, -6, 2), (-8, 3, 2), (1, -9, 2)]
    for y in range(cy - r, cy + r + 1):
        for x in range(cx - r, cx + r + 1):
            d = math.hypot(x - cx, y - cy)
            if d > r + 0.3:
                continue
            # Lit from the upper right: darker towards the lower left limb.
            light = 0.55 + 0.45 * ((x - cx) * 0.5 - (y - cy) * 0.6) / r
            colour = ramp(px, x, y, MOON, light)
            for kx, ky, kr in craters:
                kd = math.hypot(x - cx - kx, y - cy - ky)
                if kd <= kr:
                    colour = MOON[2] if kd > kr - 1 and (x - cx - kx) > 0 else MOON[3] if colour == MOON[4] else MOON[2]
            if d > r - 1:
                colour = MOON[2] if (x - cx) - (y - cy) > 0 else MOON[1]
            px[x, y] = colour


def draw_clouds(px, frame):
    drift = frame * 0.5
    bands = [(70, 34, 150, 6, 0.0), (220, 52, 120, 5, 3.0), (20, 62, 90, 4, 7.0), (250, 26, 70, 3, 11.0)]
    for x0, y0, length, thick, seed in bands:
        rng = random.Random(seed)
        start = x0 + drift * (1 + seed % 2 * 0.5)
        for i in range(int(length)):
            x = int(start + i) % (W + 60) - 30
            swell = math.sin(i / length * math.pi) * thick + math.sin(i / 7 + seed) * 1.2
            top = int(y0 - swell)
            bottom = int(y0 + swell * 0.4)
            for y in range(top, bottom + 1):
                if not (0 <= x < W and 0 <= y < H):
                    continue
                t = (y - top) / max(1, bottom - top)
                colour = CLOUD[1] if t > 0.35 else CLOUD[2]
                if y == top:
                    colour = CLOUD[3] if (i + int(seed)) % 3 else CLOUD[2]
                if BAYER[y % 4][x % 4] > 13 and t > 0.7:
                    continue
                px[x, y] = colour


def draw_mountains(px):
    far = ridge(5, 100, 22, 5, 10)
    near = ridge(9, 112, 16, 6, 14)
    for x in range(W):
        for y in range(far[x], H):
            depth = (y - far[x]) / 30
            px[x, y] = ramp(px, x, y, [FAR[2], FAR[1], FAR[0]], depth)
        # Moonlight on the slopes facing right.
        if x > 0 and far[x] < far[x - 1]:
            px[x, far[x]] = FAR[2]
            px[x, far[x] + 1] = FAR[2]
    # Mist between the ranges.
    for y in range(104, 126):
        for x in range(W):
            if y >= far[x]:
                t = 1 - abs(y - 114) / 11
                blend_over(px, x, y, MIST[0] if t < 0.6 else MIST[1], t * 0.75)
    for x in range(W):
        for y in range(near[x], H):
            depth = (y - near[x]) / 20
            px[x, y] = ramp(px, x, y, [NEAR[2], NEAR[1], NEAR[0]], depth)
        if x > 0 and near[x] <= near[x - 1]:
            px[x, near[x]] = NEAR[2]


HILL_TOP = 118


def hill_line(x):
    """The hill's crest: a broad rounded top under the temple."""
    d = abs(x - 168) / 150
    return int(HILL_TOP + 50 * d ** 2.1 + 2 * math.sin(x / 6))


def draw_hill(px):
    for x in range(W):
        top = hill_line(x)
        for y in range(top, H):
            depth = (y - top) / 70
            # Lit from the moon on the right.
            side = (x - 168) / 160
            light = 0.62 - depth * 0.7 + side * 0.25
            px[x, y] = ramp(px, x, y, HILL, light)
        # Grass along the crest, the lit tips catching the moon.
        if x % 2 == 0 or x % 5 == 0:
            height = 1 + (x * 7 % 5) // 2
            for h in range(height):
                y = top - h
                if 0 <= y < H:
                    px[x, y] = GRASS[3] if h == height - 1 and x > 150 else GRASS[2]
    # Rocks on the slope.
    for x, y, w in ((40, 150, 6), (88, 140, 4), (252, 146, 5), (286, 160, 7), (130, 160, 3)):
        for dx in range(w):
            for dy in range(3):
                px[x + dx, y - dy] = STONE[1] if dy < 2 else STONE[2]
        px[x + w - 1, y - 2] = STONE[3]


def draw_stair(px):
    """A stone stair winding up to the temple: switchbacks of short flights."""
    path = [(118, 180), (150, 168), (124, 154), (156, 142), (148, 132), (168, 124)]
    for (x0, y0), (x1, y1) in zip(path, path[1:]):
        steps = max(abs(y1 - y0) // 2, 1)
        for s in range(steps + 1):
            t = s / steps
            x = int(x0 + (x1 - x0) * t)
            y = int(y0 + (y1 - y0) * t)
            for dx in range(-5, 6):
                if 0 <= x + dx < W and 0 <= y < H:
                    px[x + dx, y] = STONE[2] if s % 2 == 0 else STONE[1]
                    if 0 <= y + 1 < H:
                        px[x + dx, y + 1] = STONE[0]
            if 0 <= x + 6 < W and 0 <= y < H:
                px[x + 6, y] = STONE[3]


def draw_cypress(px, x, base, height):
    for y in range(base - height, base):
        t = (y - (base - height)) / height
        half = int(1 + 5 * math.sin(t * math.pi * 0.92) ** 0.8)
        for dx in range(-half, half + 1):
            colour = CYPRESS[1]
            if dx >= half - 1:
                colour = CYPRESS[2] if (y + dx) % 3 else CYPRESS[1]
            if dx <= -half + 1:
                colour = CYPRESS[0]
            if (y * 3 + dx) % 7 == 0:
                colour = CYPRESS[0]
            px[x + dx, y] = colour
    for dy in range(3):
        px[x, base + dy] = (20, 14, 16)


def column(px, x, top, bottom):
    """A fluted column with its base and capital, moonlit from the right."""
    for y in range(top + 3, bottom - 2):
        for dx in range(7):
            colour = STONE[2]
            if dx in (1, 3):
                colour = STONE[1]
            if dx == 5:
                colour = STONE[4]
            if dx == 6:
                colour = STONE[3]
            if dx == 0:
                colour = STONE[0]
            px[x + dx, y] = colour
    # Entasis: a slight swell, a lighter band at the middle.
    for dx in range(-1, 8):
        px[x + dx, top] = STONE[4]
        px[x + dx, top + 1] = STONE[3]
        px[x + dx, top + 2] = STONE[2]
        px[x + dx, bottom - 2] = STONE[3]
        px[x + dx, bottom - 1] = STONE[2]
    px[x - 2, top] = STONE[3]
    px[x + 8, top] = STONE[4]


def draw_temple(px, frame):
    cx = 168
    floor = 118
    width = 104
    left = cx - width // 2
    # Three steps of the platform.
    for i in range(3):
        y = floor - i * 3
        x0, x1 = left - 10 + i * 3, left + width + 10 - i * 3
        for x in range(x0, x1 + 1):
            px[x, y - 1] = STONE[1]
            px[x, y - 2] = STONE[2]
            px[x, y - 3] = STONE[4] if x > cx else STONE[3]
        px[x1, y - 2] = STONE[4]
    base = floor - 9
    col_top = base - 44
    # The cella behind the columns, its doorway alight.
    for y in range(col_top + 4, base):
        for x in range(left + 8, left + width - 8):
            px[x, y] = (22, 18, 30) if (x + y) % 9 else (28, 22, 36)
    door_l, door_r = cx - 9, cx + 9
    flicker = 0.85 + 0.15 * math.sin(frame / FRAMES * math.tau * 3)
    for y in range(base - 30, base):
        for x in range(door_l, door_r):
            t = (y - (base - 30)) / 30
            px[x, y] = ramp(px, x, y, [WARM[1], WARM[2], FIRE[2], FIRE[3]], (0.35 + t * 0.6) * flicker)
    for y in range(base - 33, base - 30):
        for x in range(door_l - 2, door_r + 2):
            px[x, y] = STONE[2]
    for y in range(base - 30, base):
        px[door_l - 1, y] = STONE[1]
        px[door_l - 2, y] = STONE[2]
        px[door_r, y] = STONE[3]
        px[door_r + 1, y] = STONE[2]
        # The door's inner edge in shadow.
        px[door_l, y] = WARM[1]
        px[door_l + 1, y] = WARM[2]
    # Light from the door down the steps.
    for y in range(base, floor + 6):
        spread = 9 + (y - base)
        for x in range(cx - spread, cx + spread):
            if 0 <= y < H:
                blend_over(px, x, y, WARM[2] if abs(x - cx) < spread // 2 else WARM[1], 0.55 - (y - base) / 30)
    # Six columns.
    for c in range(6):
        x = left + 5 + c * 18
        column(px, x, col_top, base)
    # Architrave, frieze of triglyphs, cornice.
    for x in range(left - 4, left + width + 5):
        px[x, col_top - 1] = STONE[3]
        px[x, col_top - 2] = STONE[2]
        px[x, col_top - 3] = STONE[2]
        frieze = STONE[1] if (x - left) % 9 in (0, 2) else STONE[2]
        for y in range(col_top - 8, col_top - 3):
            px[x, y] = frieze
        px[x, col_top - 9] = STONE[4]
        px[x, col_top - 10] = STONE[3]
    # The pediment, its tympanum with a carved figure.
    apex = col_top - 30
    for y in range(apex, col_top - 10):
        t = (y - apex) / (col_top - 10 - apex)
        half = int(t * (width // 2 + 6))
        for x in range(cx - half, cx + half + 1):
            edge = x in (cx - half, cx + half) or x in (cx - half + 1, cx + half - 1)
            px[x, y] = STONE[4] if edge and x > cx else STONE[3] if edge else STONE[1]
    for (dx, dy) in ((0, -6), (-1, -5), (0, -5), (1, -5), (0, -4), (-2, -4), (2, -4), (0, -3), (-1, -2), (1, -2),
                     (-12, -3), (-11, -3), (-10, -2), (12, -3), (11, -3), (10, -2)):
        px[cx + dx, col_top - 10 + dy] = STONE[3]
    for x in (cx, cx - width // 2 - 5, cx + width // 2 + 5):
        y = apex - 1 if x == cx else col_top - 11
        px[x, y] = STONE[4]
        px[x, y - 1] = STONE[3]
    # Braziers either side of the steps, flames flickering.
    for bx in (left - 14, left + width + 14):
        for y in range(floor - 10, floor - 1):
            px[bx, y] = STONE[1]
            px[bx + 1, y] = STONE[2]
        for x in range(bx - 3, bx + 5):
            px[x, floor - 11] = STONE[3]
            px[x, floor - 12] = STONE[2]
        rng = random.Random(frame * 13 + bx)
        height = 6 + rng.randrange(3)
        for h in range(height):
            y = floor - 13 - h
            half = max(0, int((1 - h / height) * 3.2))
            sway = int(math.sin(frame * 1.3 + h * 0.7 + bx) * (h / height) * 1.5)
            for dx in range(-half, half + 2):
                colour = FIRE[min(4, 1 + (half - abs(dx)) + (2 if h < 2 else 0))]
                px[bx + dx + sway, y] = colour
        # Their glow on the stone round them.
        for y in range(floor - 28, floor + 4):
            for x in range(bx - 14, bx + 16):
                d = math.hypot(x - bx, (y - (floor - 15)) * 1.2)
                if 4 < d < 15 and 0 <= x < W:
                    blend_over(px, x, y, lerp_colour(px[x, y], FIRE[1], 0.3), (1 - d / 15) * 0.7)


def draw_fireflies(px, frame):
    rng = random.Random(23)
    for i in range(14):
        x0, y0 = rng.randrange(20, 300), rng.randrange(128, 172)
        a = frame / FRAMES * math.tau + i
        x = int(x0 + math.sin(a) * 6)
        y = int(y0 + math.cos(a * 1.3) * 3)
        on = (frame + i * 5) % 12 < 8
        if on and 0 <= x < W and 0 <= y < H:
            px[x, y] = (220, 255, 150)
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                if 0 <= x + dx < W and 0 <= y + dy < H:
                    blend_over(px, x + dx, y + dy, (90, 130, 70), 0.6)


def draw_foreground(px, frame):
    """Tall dark grass across the bottom, swaying."""
    for x in range(W):
        sway = math.sin(frame / FRAMES * math.tau + x / 9) * 1.2
        height = 6 + int(5 * (0.5 + 0.5 * math.sin(x * 1.7))) + (x * 13 % 4)
        for h in range(height):
            y = H - 1 - h
            xx = int(x + sway * h / height)
            if 0 <= xx < W:
                px[xx, y] = (6, 6, 12) if h < height - 1 else (14, 16, 26)


def render(frame):
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw_sky(px)
    draw_stars(px, frame)
    draw_moon(px, 250, 34, 15)
    draw_clouds(px, frame)
    draw_mountains(px)
    draw_hill(px)
    draw_stair(px)
    draw_cypress(px, 74, hill_line(74) + 1, 42)
    draw_cypress(px, 86, hill_line(86) + 1, 30)
    draw_cypress(px, 262, hill_line(262) + 1, 38)
    draw_temple(px, frame)
    draw_fireflies(px, frame)
    draw_foreground(px, frame)
    return image


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("assets/textures/cinematic")
    out.mkdir(parents=True, exist_ok=True)
    frames = [render(frame) for frame in range(FRAMES)]
    for index, frame in enumerate(frames):
        frame.save(out / f"temple_{index:02}.png")
    big = [frame.resize((W * 4, H * 4), Image.NEAREST) for frame in frames]
    big[0].save(out / "temple_still.png")
    big[0].save(out / "temple.gif", save_all=True, append_images=big[1:], duration=110, loop=0)


if __name__ == "__main__":
    main()
