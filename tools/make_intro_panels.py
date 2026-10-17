"""The opening cinematic's panels, drawn as pixel art at 320x180 and scaled up without smoothing:
the temple on the hill, the wound tearing open beneath it, what crawls out, the laboratory built
from the ruins, the Scientist at his portal, the tubes of subjects, and subject 41 waking.

    uv run -q --with pillow python tools/make_intro_panels.py <out dir>

Writes panel_1.png .. panel_7.png (the bare art, 320x180), and storyboard.png with every
panel scaled up and captioned, for looking them over.
"""

import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
W, H = 320, 180

# One palette for the whole cinematic: night purples, moonlit stone, the wound's crimson and
# ember, the portal's magenta, the subjects' green.
NIGHT = [(8, 6, 16), (14, 10, 26), (22, 16, 38), (34, 24, 54), (48, 32, 70)]
STONE = [(26, 22, 36), (44, 38, 58), (70, 62, 88), (104, 96, 124), (146, 138, 166), (196, 190, 210)]
EARTH = [(14, 10, 18), (22, 16, 26), (32, 24, 34)]
WOUND = [(60, 8, 16), (120, 14, 24), (190, 32, 30), (240, 90, 40), (255, 170, 80), (255, 230, 170)]
PORTAL = [(40, 6, 48), (90, 16, 110), (160, 40, 180), (220, 100, 230), (255, 190, 255)]
GLASS = [(20, 40, 36), (30, 90, 70), (60, 170, 120), (150, 240, 190)]
MOON = (232, 226, 204)
SKIN = [(120, 70, 50), (196, 132, 96), (238, 195, 154)]
HAIR = [(70, 22, 14), (130, 46, 24), (176, 74, 36)]

BAYER = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]]


def dither_fill(px, box, colours, vertical=True, start=0.0, end=1.0):
    """Fills `box` with a stepped gradient through `colours`, dithered between the steps."""
    x0, y0, x1, y1 = box
    steps = len(colours) - 1
    for y in range(y0, y1):
        for x in range(x0, x1):
            span = (y - y0) / max(1, y1 - y0 - 1) if vertical else (x - x0) / max(1, x1 - x0 - 1)
            t = start + (end - start) * span
            level = t * steps
            low = min(int(level), steps)
            frac = level - low
            pick = low + (1 if frac * 16 > BAYER[y % 4][x % 4] and low < steps else 0)
            px[x, y] = colours[pick]


def glow(px, centre, radius, colours, strength=1.0, mask=None):
    """A dithered round glow, brightest at the centre, laid over what is there."""
    cx, cy = centre
    for y in range(max(0, int(cy - radius)), min(H, int(cy + radius) + 1)):
        for x in range(max(0, int(cx - radius)), min(W, int(cx + radius) + 1)):
            if mask and not mask(x, y):
                continue
            d = math.hypot(x - cx, (y - cy) * 1.15) / radius
            if d >= 1:
                continue
            t = (1 - d) ** 1.6 * strength * (len(colours) - 1)
            low = int(t)
            pick = low + (1 if (t - low) * 16 > BAYER[y % 4][x % 4] else 0)
            if pick > 0:
                px[x, y] = colours[min(pick, len(colours) - 1)]


def stars(px, rng, count, area):
    x0, y0, x1, y1 = area
    for _ in range(count):
        x, y = rng.randrange(x0, x1), rng.randrange(y0, y1)
        bright = rng.random()
        px[x, y] = (200, 196, 220) if bright > 0.85 else (110, 100, 140)
        if bright > 0.97 and 0 < x < W - 1 and 0 < y < H - 1:
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                px[x + dx, y + dy] = (90, 80, 120)


def moon(px, centre, r):
    cx, cy = centre
    for y in range(cy - r, cy + r + 1):
        for x in range(cx - r, cx + r + 1):
            d = math.hypot(x - cx, y - cy)
            if d <= r:
                shade = (x - cx + y - cy) / (2 * r)
                px[x, y] = MOON if shade < 0.25 else (196, 190, 172)
    for x, y in ((cx - 4, cy - 2), (cx + 3, cy + 4), (cx - 1, cy + 6)):
        px[x, y] = (180, 174, 156)
        px[x + 1, y] = (180, 174, 156)


def hill(draw, top, colour, px=None, rim=None):
    """A rounded hill across the panel. With `px` it is shaded: darker towards the bottom, its
    crest picked out by the moon and tufts of grass along it."""
    points = [(0, H)]
    crest = {}
    for x in range(0, W + 8, 8):
        y = top + int(26 * (abs(x - W / 2) / (W / 2)) ** 1.7) + (2 if (x // 8) % 3 == 0 else 0)
        points.append((x, y))
    points.append((W, H))
    draw.polygon(points, fill=colour)
    if px is None:
        return
    for x in range(W):
        y = next(y for y in range(H) if px[x, y] == colour) if any(px[x, y] == colour for y in range(H)) else H
        crest[x] = y
    lighter = tuple(min(255, c + 12) for c in colour)
    darker = tuple(max(0, c - 8) for c in colour)
    for x, y in crest.items():
        for dy in range(0, H - y):
            t = dy / 60
            pick = lighter if t < 0.25 else colour if t < 0.7 else darker
            if 0.2 < t < 0.3 or 0.65 < t < 0.75:
                pick = lighter if BAYER[(y + dy) % 4][x % 4] > 8 else colour
            px[x, y + dy] = pick
        if rim and y < H:
            px[x, y] = rim
            if x % 7 == 0 and y > 1:
                px[x, y - 1] = rim
                px[x + 1 if x + 1 < W else x, y - 2] = rim


def temple(draw, px, base_x, base_y, broken=False):
    """A temple of six columns on three steps, moonlit from the right."""
    width = 96
    left = base_x - width // 2
    for i, step in enumerate(range(3)):
        y = base_y - i * 3
        draw.rectangle([left - 6 + i * 3, y - 3, left + width + 6 - i * 3, y - 1], fill=STONE[2])
        draw.line([left - 6 + i * 3, y - 3, left + width + 6 - i * 3, y - 3], fill=STONE[3])
    floor = base_y - 9
    tops = []
    for c in range(6):
        x = left + 6 + c * 16
        height = 40
        if broken and c in (1, 4):
            height = 16 + c * 2
        top = floor - height
        draw.rectangle([x, top, x + 7, floor], fill=STONE[2])
        draw.line([x + 6, top, x + 6, floor], fill=STONE[4])
        draw.line([x + 7, top, x + 7, floor], fill=STONE[3])
        draw.line([x, top, x, floor], fill=STONE[1])
        for fy in range(top + 3, floor, 6):
            px[x + 3, fy] = STONE[1]
        draw.rectangle([x - 1, top - 2, x + 8, top], fill=STONE[3])
        tops.append(top)
    roof = floor - 40 - 2
    if broken:
        # The pediment slid off to the left and cracked.
        draw.polygon([(left - 2, roof - 2), (left + 52, roof - 22), (left + 66, roof - 4), (left + 30, roof + 2)], fill=STONE[2])
        draw.line([(left - 2, roof - 2), (left + 52, roof - 22)], fill=STONE[4])
        draw.line([(left + 34, roof - 12), (left + 40, roof - 2)], fill=STONE[0])
    else:
        draw.rectangle([left - 4, roof - 4, left + width + 4, roof], fill=STONE[3])
        draw.line([left - 4, roof - 4, left + width + 4, roof - 4], fill=STONE[4])
        draw.polygon([(left - 4, roof - 4), (base_x, roof - 22), (left + width + 4, roof - 4)], fill=STONE[2])
        draw.line([(base_x, roof - 22), (left + width + 4, roof - 4)], fill=STONE[4])
        draw.polygon([(left + 8, roof - 6), (base_x, roof - 17), (left + width - 8, roof - 6)], fill=STONE[1])


def panel_temple(broken=False):
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    rng = random.Random(7)
    if broken:
        dither_fill(px, (0, 0, W, 120), [NIGHT[0], NIGHT[1], (40, 12, 26), (80, 18, 30)])
    else:
        dither_fill(px, (0, 0, W, 120), NIGHT[:5])
    stars(px, rng, 90 if not broken else 40, (0, 0, W, 100))
    moon(px, (262, 30), 12)
    if broken:
        glow(px, (W // 2, 128), 110, [(0, 0, 0), WOUND[0], WOUND[1], WOUND[2]], 0.9)
    hill(draw, 112, EARTH[2], px, STONE[1])
    hill(draw, 126, EARTH[1], px, EARTH[2])
    temple(draw, px, W // 2, 116, broken)
    if broken:
        # The tear down the hill, glowing from below.
        crack = [(W // 2 - 3, 116), (W // 2 + 4, 128), (W // 2 - 6, 140), (W // 2 + 8, 156), (W // 2 - 2, 180)]
        for width, colour in ((7, WOUND[1]), (4, WOUND[3]), (1, WOUND[5])):
            draw.line(crack, fill=colour, width=width)
        for _ in range(60):
            x = W // 2 + rng.randrange(-60, 60)
            y = rng.randrange(60, 130)
            px[x, y] = rng.choice([WOUND[3], WOUND[4], STONE[3]])
        # Light rising from the tear.
        for y in range(40, 116):
            spread = (116 - y) // 3
            for x in range(W // 2 - spread, W // 2 + spread):
                if (x + y) % 5 == 0 and BAYER[y % 4][x % 4] > 10:
                    px[x, y] = WOUND[1]
    else:
        # A light in the temple: it is still holy, for now.
        glow(px, (W // 2, 96), 14, [(0, 0, 0), (60, 50, 40), (120, 100, 70)], 0.6)
    return image


def spider(draw, px, x, y, size, colour, eye):
    draw.ellipse([x - size, y - size // 2, x + size, y + size // 2 + 1], fill=colour)
    draw.ellipse([x - size // 2, y - size, x + size // 2, y - size // 3], fill=colour)
    for side in (-1, 1):
        for leg in range(4):
            knee = (x + side * (size + 3 + leg), y - size + leg * 3)
            foot = (x + side * (size + 6 + leg * 2), y + leg * 3 + 2)
            draw.line([(x + side * size // 2, y - 1 + leg), knee, foot], fill=colour)
    px[x - 2, y - size + 2] = eye
    px[x + 2, y - size + 2] = eye


def rat(draw, px, x, y, colour, eye):
    draw.ellipse([x - 9, y - 4, x + 7, y + 4], fill=colour)
    draw.polygon([(x + 6, y - 3), (x + 14, y), (x + 6, y + 3)], fill=colour)
    draw.line([(x - 9, y + 1), (x - 18, y - 3), (x - 24, y)], fill=colour)
    px[x + 10, y - 1] = eye


def panel_wound():
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    rng = random.Random(11)
    dither_fill(px, (0, 0, W, H), [EARTH[0], EARTH[1], EARTH[2], EARTH[1]])
    # The wound: a jagged mouth in the rock, its depths burning.
    cx, cy = W // 2, 104
    mouth = []
    for i in range(28):
        a = 2 * math.pi * i / 28
        r = 70 + rng.randrange(-10, 10)
        mouth.append((cx + math.cos(a) * r * 1.5, cy + math.sin(a) * r * 0.55))
    draw.polygon(mouth, fill=WOUND[0])
    inside = lambda x, y: ((x - cx) / 100) ** 2 + ((y - cy) / 36) ** 2 < 1
    glow(px, (cx, cy + 6), 120, [WOUND[0], WOUND[1], WOUND[2], WOUND[3], WOUND[4], WOUND[5]], 1.0, inside)
    # Swirl of embers rising.
    for i in range(140):
        a = rng.random() * 2 * math.pi
        r = rng.random() ** 0.5 * 95
        x, y = int(cx + math.cos(a) * r * 1.4), int(cy - abs(math.sin(a)) * r * 0.9)
        if 0 <= x < W and 0 <= y < H:
            px[x, y] = rng.choice(WOUND[3:])
    # The rim, lit from below.
    draw.line(mouth + [mouth[0]], fill=STONE[1], width=3)
    for (x0, y0), (x1, y1) in zip(mouth[7:22], mouth[8:23]):
        draw.line([(x0, y0 + 2), (x1, y1 + 2)], fill=WOUND[2])
    # What crawls out: the first spiders and rats, black against the glow, eyes alight.
    shade = (6, 4, 8)
    spider(draw, px, 92, 82, 7, shade, WOUND[5])
    spider(draw, px, 236, 74, 9, shade, WOUND[5])
    spider(draw, px, 170, 58, 5, shade, WOUND[4])
    rat(draw, px, 120, 132, shade, WOUND[5])
    rat(draw, px, 214, 128, shade, WOUND[5])
    # Roots and rock over the top edge, framing it.
    for x in range(0, W, 3):
        top = 6 + int(4 * math.sin(x / 11)) + rng.randrange(3)
        draw.line([(x, 0), (x, top)], fill=EARTH[0])
    return image


def lab(draw, px, x, y):
    """The laboratory: a squat stone block grown from the temple's ruins, pipes and a chimney,
    warm windows and the portal's round window glowing magenta."""
    draw.rectangle([x, y - 46, x + 90, y], fill=STONE[1])
    draw.line([x + 90, y - 46, x + 90, y], fill=STONE[3])
    for row in range(y - 44, y, 6):
        offset = 0 if (row // 6) % 2 else 6
        for col in range(x + offset, x + 90, 12):
            px[col, row] = STONE[0]
    draw.polygon([(x - 4, y - 46), (x + 45, y - 64), (x + 94, y - 46)], fill=STONE[0])
    draw.line([(x + 45, y - 64), (x + 94, y - 46)], fill=STONE[2])
    draw.rectangle([x + 66, y - 78, x + 74, y - 56], fill=STONE[1])
    draw.line([x + 74, y - 78, x + 74, y - 56], fill=STONE[3])
    for wx in (x + 10, x + 70):
        draw.rectangle([wx, y - 34, wx + 9, y - 22], fill=(250, 200, 110))
        draw.line([wx, y - 28, wx + 9, y - 28], fill=STONE[0])
        draw.line([wx + 4, y - 34, wx + 4, y - 22], fill=STONE[0])
    draw.ellipse([x + 32, y - 40, x + 58, y - 14], fill=PORTAL[1])
    draw.ellipse([x + 36, y - 36, x + 54, y - 18], fill=PORTAL[3])
    draw.ellipse([x + 41, y - 31, x + 49, y - 23], fill=PORTAL[4])
    draw.line([x + 32, y - 27, x + 58, y - 27], fill=STONE[0])
    draw.line([(x - 8, y - 20), (x, y - 20)], fill=STONE[2], width=3)
    draw.line([(x - 8, y - 20), (x - 8, y)], fill=STONE[2], width=3)


def panel_lab():
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    rng = random.Random(3)
    dither_fill(px, (0, 0, W, 130), [NIGHT[0], NIGHT[1], NIGHT[2], (40, 18, 50)])
    stars(px, rng, 80, (0, 0, W, 90))
    moon(px, (48, 28), 10)
    hill(draw, 116, EARTH[2], px, STONE[1])
    # Ruined columns either side of the lab.
    for x, height in ((86, 24), (100, 34), (220, 30), (236, 14)):
        draw.rectangle([x, 120 - height, x + 6, 120], fill=STONE[2])
        draw.line([x + 6, 120 - height, x + 6, 120], fill=STONE[4])
        draw.line([(x - 1, 120 - height), (x + 3, 118 - height), (x + 7, 121 - height)], fill=STONE[3])
    lab(draw, px, 116, 120)
    glow(px, (161, 94), 60, [(0, 0, 0), PORTAL[0], PORTAL[1]], 0.6, lambda x, y: y < 120 and not (116 <= x <= 206 and 74 <= y <= 120))
    # Smoke from the chimney.
    for i in range(16):
        x = 186 + int(math.sin(i / 2) * 3) + i // 2
        y = 40 - i * 3
        if y > 0:
            draw.ellipse([x - 3 - i // 4, y - 2, x + 3 + i // 4, y + 2], fill=NIGHT[3] if i % 2 else NIGHT[4])
    hill(draw, 130, EARTH[1], px, EARTH[2])
    # The wound under the hill, a faint red line beneath it all.
    for x in range(100, 220):
        y = 150 + int(3 * math.sin(x / 9))
        if BAYER[y % 4][x % 4] > 4:
            px[x, y] = WOUND[1]
    glow(px, (160, 150), 50, [(0, 0, 0), WOUND[0]], 0.8, lambda x, y: y > 140)
    return image


def scientist(draw, px, x, y):
    """The Scientist from behind and a little to the side, at the game's pixel scale doubled:
    grey hair, the strap of his goggles, a white coat."""
    s = 2
    def block(x0, y0, x1, y1, colour):
        draw.rectangle([x + x0 * s, y + y0 * s, x + x1 * s - 1, y + y1 * s - 1], fill=colour)
    block(-6, 0, 6, 6, (97, 83, 83))
    block(-5, -1, 5, 0, (112, 96, 96))
    block(-6, 3, 6, 4, (41, 22, 30))
    block(4, 3, 6, 5, (41, 94, 78))
    block(-5, 6, 5, 8, SKIN[1])
    block(-7, 8, 7, 24, (245, 237, 237))
    block(5, 8, 7, 24, (199, 193, 193))
    block(-7, 8, -5, 24, (179, 173, 173))
    block(-1, 8, 1, 22, (199, 193, 193))
    block(-5, 24, -1, 32, (112, 96, 96))
    block(1, 24, 5, 32, (97, 83, 83))
    block(7, 12, 9, 20, (245, 237, 237))
    block(8, 19, 10, 21, SKIN[1])


def panel_portal():
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    dither_fill(px, (0, 0, W, H), [NIGHT[0], NIGHT[1], NIGHT[2]])
    # Back wall bricks.
    for row in range(4, 130, 10):
        offset = 0 if (row // 10) % 2 else 12
        for col in range(offset, W, 24):
            draw.rectangle([col, row, col + 22, row + 8], fill=NIGHT[1])
    # Floor.
    dither_fill(px, (0, 130, W, H), [STONE[0], EARTH[1], EARTH[0]])
    # The portal: a stone ring, the inside a swirl of magenta.
    cx, cy = 200, 82
    draw.ellipse([cx - 52, cy - 62, cx + 52, cy + 62], fill=STONE[2])
    draw.ellipse([cx - 46, cy - 56, cx + 46, cy + 56], fill=PORTAL[0])
    inside = lambda x, y: ((x - cx) / 46) ** 2 + ((y - cy) / 56) ** 2 < 1
    glow(px, (cx, cy), 70, PORTAL, 1.0, inside)
    for i in range(240):
        a = i * 0.21
        r = 4 + i * 0.18
        x, y = int(cx + math.cos(a) * r * 0.9), int(cy + math.sin(a) * r * 1.1)
        if inside(x, y):
            px[x, y] = PORTAL[4] if i % 3 == 0 else PORTAL[2]
    for i in range(10):
        a = i * math.pi / 5
        x, y = int(cx + math.cos(a) * 49), int(cy + math.sin(a) * 59)
        draw.rectangle([x - 3, y - 3, x + 3, y + 3], fill=STONE[3])
        px[x, y] = PORTAL[4]
    glow(px, (cx, cy + 60), 80, [(0, 0, 0), PORTAL[0], PORTAL[1]], 0.7, lambda x, y: y >= 130 and not inside(x, y))
    # Shelves of flasks on the left wall.
    for sy in (40, 70):
        draw.rectangle([14, sy, 96, sy + 3], fill=STONE[1])
        for i, colour in enumerate([GLASS[2], WOUND[2], PORTAL[3], GLASS[3], WOUND[4], GLASS[2]]):
            fx = 20 + i * 13
            draw.rectangle([fx, sy - 9, fx + 6, sy - 1], fill=colour)
            draw.rectangle([fx + 2, sy - 13, fx + 4, sy - 9], fill=STONE[3])
            px[fx + 1, sy - 8] = (255, 255, 255)
    # The Scientist, facing his work.
    scientist(draw, px, 112, 92)
    return image


def tube(draw, px, x, y, number, state, font):
    """A glass tube for a subject: 'broken', 'drained' or 'alive'."""
    draw.rectangle([x - 2, y - 4, x + 34, y + 2], fill=STONE[2])
    draw.rectangle([x - 2, y + 94, x + 34, y + 102], fill=STONE[2])
    draw.line([x - 2, y - 4, x + 34, y - 4], fill=STONE[4])
    if state == "alive":
        dither_fill(px, (x + 1, y + 3, x + 32, y + 94), [GLASS[1], GLASS[2], GLASS[1]], start=0.2, end=0.9)
        # A figure curled inside, the player's red hair.
        draw.ellipse([x + 10, y + 26, x + 22, y + 38], fill=SKIN[0])
        draw.ellipse([x + 9, y + 23, x + 23, y + 31], fill=HAIR[1])
        draw.rectangle([x + 11, y + 38, x + 21, y + 66], fill=(40, 70, 56))
        draw.rectangle([x + 12, y + 66, x + 15, y + 84], fill=(34, 60, 48))
        draw.rectangle([x + 17, y + 66, x + 20, y + 84], fill=(34, 60, 48))
        for i in range(8):
            px[x + 5 + (i * 7) % 24, y + 88 - i * 10] = GLASS[3]
        glow(px, (x + 16, y + 48), 36, [(0, 0, 0), GLASS[0], GLASS[1]], 0.5, lambda gx, gy: not (x <= gx <= x + 32 and y <= gy <= y + 94))
    else:
        dither_fill(px, (x + 1, y + 3, x + 32, y + 94), [NIGHT[1], NIGHT[2]], start=0.0, end=0.6)
        if state == "drained":
            draw.rectangle([x + 1, y + 84, x + 32, y + 94], fill=GLASS[0])
        if state == "broken":
            for sx, sy, ex, ey in ((x + 4, y + 20, x + 14, y + 44), (x + 14, y + 44, x + 8, y + 70), (x + 14, y + 44, x + 26, y + 52)):
                draw.line([sx, sy, ex, ey], fill=STONE[4])
            for i in range(10):
                px[x - 6 + i * 5, y + 100 + (i % 2)] = STONE[4]
    draw.line([x, y + 2, x, y + 94], fill=STONE[4])
    draw.line([x + 32, y + 2, x + 32, y + 94], fill=STONE[3])
    draw.line([x + 4, y + 6, x + 4, y + 30], fill=(220, 230, 240))
    draw.rectangle([x + 6, y + 106, x + 26, y + 116], fill=STONE[1])
    draw.text((x + 16, y + 111), number, fill=STONE[5] if state != "alive" else GLASS[3], font=font, anchor="mm")


def panel_tubes(font):
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    dither_fill(px, (0, 0, W, H), [NIGHT[0], NIGHT[1], NIGHT[0]])
    for row in range(0, 140, 12):
        draw.line([0, row, W, row], fill=NIGHT[1])
    dither_fill(px, (0, 140, W, H), [STONE[0], EARTH[0]])
    for i, (number, state) in enumerate((("38", "broken"), ("39", "drained"), ("40", "broken"), ("41", "alive"))):
        tube(draw, px, 36 + i * 70, 22, number, state, font)
    # Cables along the floor to the last tube.
    draw.line([(0, 150), (60, 152), (140, 149), (250, 151), (262, 124)], fill=STONE[1], width=2)
    return image


def panel_wake(closed=False):
    """Subject 41 standing where he woke, the game's own sprite four times over, the portal's
    light on him from the left. `closed` has his eyes still shut."""
    image = Image.new("RGB", (W, H))
    px = image.load()
    draw = ImageDraw.Draw(image)
    dither_fill(px, (0, 0, W, H), [NIGHT[0], (12, 8, 18), NIGHT[0]])
    dither_fill(px, (0, 156, W, H), [EARTH[1], EARTH[0]])
    # The portal's edge at the left, its light across the floor.
    glow(px, (-10, 80), 150, [(0, 0, 0), PORTAL[0], PORTAL[1], PORTAL[2]], 0.8)
    sheet = Image.open(ROOT / "assets/textures/ingame/player.tga").convert("RGBA")
    frame = sheet.crop((0, 0, 16, 38))
    if closed:
        fpx = frame.load()
        for y in range(38):
            for x in range(16):
                r, g, b, a = fpx[x, y]
                # The blue of his eyes and their whites go under the lids.
                if a and (b > r + 20 or (r > 200 and g > 200 and b > 200)):
                    fpx[x, y] = (196, 132, 96, 255)
    scale = 4
    big = frame.resize((16 * scale, 38 * scale), Image.NEAREST)
    x, y = W // 2 - 8 * scale, 168 - 38 * scale
    # His shadow on the floor.
    draw.ellipse([x + 6, 164, x + 16 * scale - 6, 172], fill=EARTH[0])
    image.paste(big, (x, y), big)
    # The portal's light on his left side.
    for yy in range(y, y + 38 * scale):
        for xx in range(x, x + 6 * scale):
            if big.getpixel((xx - x, yy - y))[3] and BAYER[yy % 4][xx % 4] > 9:
                r, g, b = px[xx, yy]
                px[xx, yy] = (min(255, r + 40), g, min(255, b + 60))
    return image


CAPTIONS = [
    "Long ago, a temple stood on the hill.",
    "Then something tore open beneath it.",
    "They called it Bane. The wound.",
    "From the temple's ruins, he built his laboratory.",
    "He found a way in.",
    "He sends his subjects down. None come back the same.",
    "Subject 41. Wake up.",
]


def main():
    out = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/cinematic"
    out.mkdir(parents=True, exist_ok=True)
    small = ImageFont.truetype(str(ROOT / "assets/fonts/PixelFJVerdana12pt.ttf"), 6)
    big = ImageFont.truetype(str(ROOT / "assets/fonts/PixelFJVerdana12pt.ttf"), 24)
    mid = ImageFont.truetype(str(ROOT / "assets/fonts/PixelFJVerdana12pt.ttf"), 17)
    panels = [
        panel_temple(),
        panel_temple(broken=True),
        panel_wound(),
        panel_lab(),
        panel_portal(),
        panel_tubes(small),
        panel_wake(),
    ]
    panel_wake(closed=True).save(out / "panel_7_closed.png")
    scale = 3
    board = Image.new("RGB", (2 * W * scale + 30, 4 * (H * scale + 70) + 10), (10, 8, 14))
    draw = ImageDraw.Draw(board)
    for index, (panel, caption) in enumerate(zip(panels, CAPTIONS)):
        panel.save(out / f"panel_{index + 1}.png")
        big_panel = panel.resize((W * scale, H * scale), Image.NEAREST)
        x = 10 + (index % 2) * (W * scale + 10)
        y = 10 + (index // 2) * (H * scale + 70)
        board.paste(big_panel, (x, y))
        draw.text((x + W * scale // 2, y + H * scale + 32), f"{index + 1}. {caption}", fill=(236, 226, 210), font=big, anchor="mm") if big.getlength(caption) < W * scale - 60 else draw.text((x + W * scale // 2, y + H * scale + 32), f"{index + 1}. {caption}", fill=(236, 226, 210), font=mid, anchor="mm")
    board.save(out / "storyboard.png")


if __name__ == "__main__":
    main()
