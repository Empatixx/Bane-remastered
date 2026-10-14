import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/rounds"

CLEAR = (0, 0, 0, 0)
WHITE = (255, 255, 255, 255)
YELLOW = (255, 252, 0, 255)
ORANGE = (255, 174, 0, 255)
RED = (255, 78, 0, 255)
EMBER = (176, 30, 10, 255)
SMOKE = (92, 88, 96, 255)
SMOKE_L = (140, 136, 146, 255)
CYAN = (150, 236, 255, 255)
CYAN_D = (60, 160, 230, 255)
STEEL_D = (48, 50, 54, 255)
STEEL = (86, 88, 93, 255)
STEEL_L = (149, 150, 155, 255)
WOOD_D = (76, 24, 20, 255)
WOOD = (133, 74, 67, 255)
WOOD_L = (181, 123, 115, 255)
OLIVE_D = (52, 62, 34, 255)
OLIVE = (88, 104, 56, 255)
OLIVE_L = (138, 156, 92, 255)
FLETCH = (200, 50, 46, 255)
LASER_D = (150, 16, 24, 255)
LASER = (240, 40, 48, 255)
LASER_L = (255, 190, 190, 255)
PLASMA_D = (24, 120, 40, 255)
PLASMA = (70, 230, 90, 255)
PLASMA_L = (210, 255, 200, 255)
VOLT_D = (30, 70, 190, 255)
VOLT = (80, 160, 255, 255)
VOLT_L = (210, 236, 255, 255)
COLUMNS, ROWS = 4, 2


class Sheet:
    """Four flight frames on the top row, four impact frames below, every round facing right."""

    def __init__(self, w, h):
        self.w, self.h = w, h
        self.img = Image.new("RGBA", (w * COLUMNS, h * ROWS), CLEAR)
        self.d = ImageDraw.Draw(self.img)

    def cell(self, column, row):
        ox, oy = column * self.w, row * self.h
        sheet = self

        class Cell:
            def rect(self, x0, y0, x1, y1, c):
                sheet.d.rectangle((ox + x0, oy + y0, ox + x1, oy + y1), fill=c)

            def px(self, x, y, c):
                if 0 <= x < sheet.w and 0 <= y < sheet.h:
                    sheet.img.putpixel((ox + x, oy + y), c)

            def poly(self, pts, c):
                sheet.d.polygon([(ox + x, oy + y) for x, y in pts], fill=c)

            def disc(self, cx, cy, r, c):
                sheet.d.ellipse((ox + cx - r, oy + cy - r, ox + cx + r, oy + cy + r), fill=c)

        return Cell()


def sparks(sheet, colors, rng, reach):
    cx, cy = sheet.w // 2, sheet.h // 2
    angles = [rng.uniform(0, math.tau) for _ in range(7)]
    for frame in range(4):
        c = sheet.cell(frame, 1)
        if frame == 0:
            c.disc(cx, cy, 3, colors[1])
            c.disc(cx, cy, 1, colors[0])
        for a in angles:
            dist = 2 + (frame + 1) * reach / 4
            x, y = round(cx + math.cos(a) * dist), round(cy + math.sin(a) * dist)
            color = colors[min(frame, len(colors) - 1)]
            c.px(x, y, color)
            if frame < 2:
                c.px(x - round(math.cos(a)), y - round(math.sin(a)), colors[min(frame + 1, len(colors) - 1)])


def tracer():
    s = Sheet(16, 16)
    for frame in range(4):
        c = s.cell(frame, 0)
        tail = 3 + frame % 2
        c.rect(12 - tail - 4, 7, 9, 8, RED)
        c.rect(8, 7, 11, 8, ORANGE)
        c.rect(10, 7, 12, 8, YELLOW)
        c.px(12, 7, WHITE)
    sparks(s, [WHITE, YELLOW, ORANGE, RED], random.Random(3), 5)
    return s.img


def slug():
    s = Sheet(32, 16)
    for frame in range(4):
        c = s.cell(frame, 0)
        c.rect(2 + frame % 2 * 2, 8, 14, 8, CYAN_D)
        c.rect(12, 7, 26, 8, CYAN_D)
        c.rect(16, 7, 27, 7, CYAN)
        c.rect(24, 7, 28, 8, WHITE)
        c.px(29, 7 + frame % 2, CYAN)
    sparks(s, [WHITE, CYAN, CYAN, CYAN_D], random.Random(7), 9)
    return s.img


def rocket():
    s = Sheet(24, 24)
    for frame in range(4):
        c = s.cell(frame, 0)
        flame = [3, 5, 4, 6][frame]
        c.poly([(8, 10), (8 - flame, 12), (8, 13)], RED)
        c.poly([(8, 11), (8 - flame + 2, 12), (8, 12)], YELLOW)
        c.rect(8, 9, 10, 14, STEEL_D)
        c.rect(7, 8, 9, 8, STEEL)
        c.rect(7, 15, 9, 15, STEEL)
        c.rect(10, 10, 15, 13, STEEL)
        c.rect(10, 10, 15, 10, STEEL_L)
        c.poly([(15, 8), (19, 9), (21, 11), (21, 12), (19, 14), (15, 15)], OLIVE)
        c.poly([(15, 8), (19, 9), (21, 11), (15, 11)], OLIVE_L)
        c.rect(15, 14, 19, 14, OLIVE_D)
        c.rect(22, 11, 22, 12, ORANGE)
    rng = random.Random(11)
    for frame in range(4):
        c = s.cell(frame, 1)
        r = 4 + frame * 2
        c.disc(12, 12, r, [YELLOW, ORANGE, RED, SMOKE][frame])
        if frame < 3:
            c.disc(12, 12, r - 2, [WHITE, YELLOW, ORANGE][frame])
        for _ in range(6 + frame * 2):
            a, d = rng.uniform(0, math.tau), r + rng.uniform(0, 2)
            c.px(round(12 + math.cos(a) * d), round(12 + math.sin(a) * d), [ORANGE, RED, SMOKE, SMOKE_L][frame])
    return s.img


def bolt():
    s = Sheet(32, 16)
    for frame in range(4):
        c = s.cell(frame, 0)
        c.rect(5, 7, 24, 8, WOOD)
        c.rect(5, 7, 24, 7, WOOD_L)
        c.rect(5, 8, 24, 8, WOOD_D)
        c.poly([(3, 5), (9, 7), (9, 8), (3, 10)], FLETCH)
        c.rect(3 + frame % 2, 6, 4 + frame % 2, 9, EMBER)
        c.poly([(24, 5), (29, 7), (29, 8), (24, 10)], STEEL)
        c.poly([(24, 5), (29, 7), (24, 7)], STEEL_L)
        c.px(29, 7, WHITE)
    rng = random.Random(5)
    chips = [(rng.uniform(0, math.tau), rng.choice([WOOD, WOOD_L, STEEL_L])) for _ in range(6)]
    for frame in range(4):
        c = s.cell(frame, 1)
        if frame < 2:
            c.rect(18, 7, 24, 8, WOOD)
            c.rect(24, 6, 26, 9, STEEL)
        for a, color in chips:
            d = 2 + frame * 2
            c.px(round(23 + math.cos(a) * d), round(7 + math.sin(a) * d), color)
    return s.img


def flame():
    s = Sheet(24, 24)
    rng = random.Random(9)
    for frame in range(4):
        c = s.cell(frame, 0)
        r = 5 + frame % 2
        c.disc(12, 12, r + 1, RED)
        c.disc(13, 12, r, ORANGE)
        c.disc(14, 11 + frame % 2, r - 2, YELLOW)
        c.disc(15, 12, 1, WHITE)
        for _ in range(5):
            a, d = rng.uniform(math.pi * 0.6, math.pi * 1.4), r + rng.uniform(1, 4)
            c.px(round(12 + math.cos(a) * d), round(12 + math.sin(a) * d), rng.choice([RED, ORANGE]))
    for frame in range(4):
        c = s.cell(frame, 1)
        r = 5 + frame
        c.disc(12, 12 - frame, r, [ORANGE, RED, SMOKE, SMOKE][frame])
        c.disc(12, 12 - frame, max(r - 3, 1), [YELLOW, ORANGE, SMOKE_L, SMOKE][frame])
        if frame == 3:
            for _ in range(10):
                c.px(rng.randrange(4, 20), rng.randrange(2, 20), CLEAR)
    return s.img


def laser():
    s = Sheet(32, 16)
    for frame in range(4):
        c = s.cell(frame, 0)
        c.rect(4, 6, 27, 9, LASER_D)
        c.rect(6, 7, 28, 8, LASER)
        c.rect(12 + frame, 7, 26, 7, LASER_L)
        c.rect(26, 7, 29, 8, WHITE)
    sparks(s, [WHITE, LASER_L, LASER, LASER_D], random.Random(13), 7)
    return s.img


def plasma():
    s = Sheet(24, 24)
    rng = random.Random(17)
    for frame in range(4):
        c = s.cell(frame, 0)
        r = 6 + frame % 2
        c.disc(12, 12, r, PLASMA_D)
        c.disc(12, 12, r - 1, PLASMA)
        c.disc(13, 11, r - 3, PLASMA_L)
        c.px(14, 10, WHITE)
        for _ in range(4):
            a, d = rng.uniform(0, math.tau), r + rng.uniform(1, 3)
            c.px(round(12 + math.cos(a) * d), round(12 + math.sin(a) * d), PLASMA)
    for frame in range(4):
        c = s.cell(frame, 1)
        r = 5 + frame * 2
        c.disc(12, 12, r, [PLASMA_L, PLASMA, PLASMA_D, PLASMA_D][frame])
        if frame < 3:
            c.disc(12, 12, max(r - 3, 1), [WHITE, PLASMA_L, PLASMA][frame])
        if frame == 3:
            c.disc(12, 12, r - 2, CLEAR)
    return s.img


def zigzag(c, x0, x1, y, rng, color, spread):
    x, py = x0, y
    while x < x1:
        nx = min(x1, x + rng.randint(3, 6))
        ny = y + rng.randint(-spread, spread)
        steps = max(abs(nx - x), abs(ny - py), 1)
        for i in range(steps + 1):
            c.px(round(x + (nx - x) * i / steps), round(py + (ny - py) * i / steps), color)
        x, py = nx, ny


def spark():
    s = Sheet(24, 24)
    rng = random.Random(19)
    for frame in range(4):
        c = s.cell(frame, 0)
        c.disc(12, 12, 6, VOLT_D)
        c.disc(12, 12, 4, VOLT)
        c.disc(12, 12, 2, VOLT_L)
        c.px(12, 12, WHITE)
        for _ in range(3):
            a = rng.uniform(0, math.tau)
            x, y = 12.0, 12.0
            for _ in range(5):
                a += rng.uniform(-0.8, 0.8)
                x, y = x + math.cos(a) * 2, y + math.sin(a) * 2
                c.px(round(x), round(y), rng.choice([VOLT_L, WHITE, VOLT]))
    for frame in range(4):
        c = s.cell(frame, 1)
        r = 4 + frame * 2
        if frame < 2:
            c.disc(12, 12, r, VOLT)
            c.disc(12, 12, r - 2, VOLT_L)
        for _ in range(6 - frame):
            a = rng.uniform(0, math.tau)
            x, y = 12.0, 12.0
            for _ in range(3 + frame * 2):
                a += rng.uniform(-0.9, 0.9)
                x, y = x + math.cos(a) * 1.5, y + math.sin(a) * 1.5
                c.px(round(x), round(y), [WHITE, VOLT_L, VOLT, VOLT_D][frame])
    return s.img


def rail():
    s = Sheet(48, 16)
    for frame in range(4):
        c = s.cell(frame, 0)
        c.rect(0, 6, 47, 9, (20, 110, 140, 255))
        c.rect(0, 7, 47, 8, (60, 220, 240, 255))
        c.rect(8 + frame * 4, 7, 47, 7, (200, 252, 255, 255))
        c.rect(40, 6, 46, 9, WHITE)
        for x in range(frame * 3, 40, 12):
            c.px(x, 5, (60, 220, 240, 255))
            c.px(x + 6, 10, (60, 220, 240, 255))
    sparks(s, [WHITE, (200, 252, 255, 255), (60, 220, 240, 255), (20, 110, 140, 255)], random.Random(23), 12)
    return s.img


def arc():
    """A lightning bolt drawn left to right; the game stretches it between two enemies."""
    s = Sheet(64, 16)
    rng = random.Random(29)
    for row in range(2):
        for frame in range(4):
            c = s.cell(frame, row)
            fade = row == 1 and frame >= 2
            zigzag(c, 0, 63, 8, rng, VOLT_D if fade else VOLT, 4)
            if not (row == 1 and frame == 3):
                zigzag(c, 0, 63, 8, rng, VOLT_L if not fade else VOLT, 3)
            if row == 0:
                zigzag(c, 0, 63, 8, rng, WHITE, 2)
    return s.img


class Motes:
    """Three tiny frames in a row, one mote each, for the dust system to pick from."""

    def __init__(self, size):
        self.w = self.h = size
        self.img = Image.new("RGBA", (size * 3, size), CLEAR)

    def px(self, frame, x, y, c):
        if 0 <= x < self.w and 0 <= y < self.h:
            self.img.putpixel((frame * self.w + x, y), c)


def mote_volt():
    m = Motes(8)
    rng = random.Random(31)
    for frame in range(3):
        x, y = 4.0, 4.0
        a = rng.uniform(0, math.tau)
        for step in range(4 + frame):
            a += rng.uniform(-1.0, 1.0)
            x, y = x + math.cos(a) * 1.4, y + math.sin(a) * 1.4
            m.px(frame, round(x), round(y), [WHITE, VOLT_L, VOLT][min(step, 2)])
        m.px(frame, 4, 4, WHITE)
        m.px(frame, 3 + frame % 2, 4 - frame % 2, VOLT_L)
    return m.img


def mote_laser():
    m = Motes(8)
    for frame in range(3):
        r = 1 + frame % 2
        for dx in range(-r, r + 1):
            for dy in range(-r, r + 1):
                if dx * dx + dy * dy <= r * r:
                    m.px(frame, 4 + dx, 4 + dy, LASER if abs(dx) + abs(dy) == r else LASER_L)
        m.px(frame, 4, 4, WHITE)
        if frame == 2:
            m.px(frame, 1, 4, LASER_D)
            m.px(frame, 7, 4, LASER_D)
    return m.img


ROUNDS = {
    "tracer": tracer,
    "slug": slug,
    "rocket": rocket,
    "bolt": bolt,
    "flame": flame,
    "laser": laser,
    "plasma": plasma,
    "spark": spark,
    "rail": rail,
    "arc": arc,
    "mote_volt": mote_volt,
    "mote_laser": mote_laser,
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, draw in ROUNDS.items():
        img = draw()
        img.save(OUT / f"{name}.tga")
        print(name, img.size)


if __name__ == "__main__":
    main()
