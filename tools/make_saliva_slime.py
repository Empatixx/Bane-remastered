import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
OUTLINE = (112, 92, 132, 255)
BODY_D = (172, 152, 188, 255)
BODY = (210, 196, 222, 255)
BODY_L = (238, 230, 246, 255)
BODY_H = (255, 255, 255, 255)
STRAND = (226, 214, 236, 255)
EYE = (60, 40, 70, 255)
FANG = (255, 250, 236, 255)
CW, CH = 40, 32
COLUMNS = 6


def half_width(rx, t, y, wobble):
    return rx * (1 - (1 - min(1.0, t)) ** 2.3) ** 0.5 + math.sin(y * 0.7 + wobble) * 0.5


def blob(d, cx, base, rx, ry, wobble=0.0):
    top = base - ry * 2
    for y in range(int(top), int(base) + 1):
        t = (y - top) / max(1.0, base - top)
        half = half_width(rx, t, y, wobble)
        d.line((cx - half - 1, y, cx + half + 1, y), fill=OUTLINE)
    for y in range(int(top) + 1, int(base)):
        t = (y - top) / max(1.0, base - top)
        half = half_width(rx, t, y, wobble)
        colour = BODY if t < 0.62 else BODY_D
        d.line((cx - half, y, cx + half, y), fill=colour)
    d.ellipse((cx - rx * 0.65, top + ry * 0.3, cx - rx * 0.05, top + ry * 0.75), fill=BODY_L)
    d.point((cx - rx * 0.5, top + ry * 0.4), fill=BODY_H)
    d.point((cx - rx * 0.4, top + ry * 0.4), fill=BODY_H)
    return top


def face(d, cx, top, base, jaw=0):
    y = top + (base - top) * 0.45
    for ex in (cx - 5, cx + 3):
        d.rectangle((ex, y, ex + 2, y + 2), fill=EYE)
        d.point((ex, y), fill=BODY_H)
    d.line((cx - 4, y + 6 + jaw, cx + 4, y + 6 + jaw), fill=EYE)
    d.point((cx - 3, y + 7 + jaw), fill=FANG)
    d.point((cx + 3, y + 7 + jaw), fill=FANG)


def bubbles(d, cx, top, base, phase):
    for index, (dx, dy, r) in enumerate(((-7, 0.35, 2), (6, 0.55, 3), (1, 0.2, 1))):
        x = cx + dx + math.sin(phase + index) * 0.8
        y = top + (base - top) * dy - math.cos(phase + index) * 0.8
        d.ellipse((x - r, y - r, x + r, y + r), outline=BODY_L)
        d.point((x - r + 1, y - r + 1), fill=BODY_H)


def drool(d, cx, base, length, sway):
    x = cx - 8 + sway
    d.line((x, base - 1, x + sway * 0.3, base + length), fill=STRAND)
    d.ellipse((x - 1 + sway * 0.3, base + length - 1, x + 1 + sway * 0.3, base + length + 2), fill=BODY_L)
    x2 = cx + 7 - sway
    d.line((x2, base - 1, x2, base + length - 2), fill=STRAND)
    d.point((x2, base + length - 1), fill=BODY_H)


def idle(frame):
    image = Image.new("RGBA", (CW, CH), CLEAR)
    d = ImageDraw.Draw(image)
    breathe = (0, 1, 0, -1)[frame]
    base = 27
    top = blob(d, 20, base, 14 + breathe, 10 - breathe, frame * 1.3)
    face(d, 20, top, base)
    bubbles(d, 20, top, base, frame * 1.4)
    drool(d, 20, base, 3 + (frame % 2) * 2, (0, 1, 0, -1)[frame])
    return image


def hop(frame):
    poses = [
        (14, 7, 27, 0),
        (12, 12, 29, 0),
        (9, 15, 26, 1),
        (8, 16, 22, 1),
        (13, 8, 27, 0),
        (14, 8, 27, 0),
    ]
    rx, ry, base, jaw = poses[frame]
    image = Image.new("RGBA", (CW, CH), CLEAR)
    d = ImageDraw.Draw(image)
    top = blob(d, 20, base, rx, ry, frame)
    face(d, 20, top, base, jaw)
    bubbles(d, 20, top, base, frame)
    if frame in (2, 3):
        d.line((20, base + 1, 20, base + 6), fill=STRAND)
        d.point((20, base + 7), fill=BODY_H)
    elif frame < 2 or frame > 3:
        drool(d, 20, base, 2, 0)
    return image


def death(frame):
    image = Image.new("RGBA", (CW, CH), CLEAR)
    d = ImageDraw.Draw(image)
    rng = random.Random(frame + 4)
    if frame == 0:
        top = blob(d, 20, 27, 15, 8, 0)
        face(d, 20, top, 27, 2)
        d.point((20, 29), fill=STRAND)
    elif frame == 1:
        top = blob(d, 20, 28, 17, 5, 0)
        for k in range(6):
            x, y = 20 + rng.uniform(-16, 16), 20 - rng.uniform(0, 8)
            d.ellipse((x - 2, y - 2, x + 2, y + 2), fill=BODY_L, outline=OUTLINE)
    else:
        spread = frame * 3 + 6
        d.ellipse((20 - spread, 27 - 3, 20 + spread, 29), fill=OUTLINE)
        d.ellipse((20 - spread + 1, 27 - 2, 20 + spread - 1, 28), fill=BODY_D if frame < 5 else BODY)
        if frame < 5:
            d.ellipse((20 - spread + 3, 26, 20 + spread - 3, 27), fill=BODY_L)
        for k in range(max(0, 9 - frame * 2)):
            x = 20 + rng.uniform(-1, 1) * (8 + frame * 3)
            y = 28 - rng.uniform(2, 16) * (1 - frame / 6)
            r = rng.choice((1, 1, 2))
            d.ellipse((x - r, y - r, x + r, y + r), fill=BODY_L, outline=OUTLINE if r > 1 else None)
    return image


def main():
    rows = [[idle(i) for i in range(4)], [death(i) for i in range(6)], [hop(i) for i in range(6)]]
    sheet = Image.new("RGBA", (CW * COLUMNS, CH * len(rows)), CLEAR)
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * CW, row * CH))
    sheet.save(OUT / "saliva_slime.tga")


main()
