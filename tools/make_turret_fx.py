import math
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

CELL = 40
COLUMNS = 5
ROWS = 7
SHARD = 16
SHARD_FRAMES = 4

CLEAR = (0, 0, 0, 0)
INK = (24, 18, 26, 255)
BONE_D = (150, 134, 104, 255)
BONE = (214, 202, 172, 255)
BONE_L = (246, 240, 222, 255)
SOCKET = (120, 20, 24, 255)
EMBER = (255, 150, 60, 255)
IRON_D = (34, 34, 40, 255)
IRON = (66, 66, 76, 255)
IRON_L = (110, 110, 122, 255)
DIRT_D = (62, 42, 34, 255)
DIRT = (104, 74, 54, 255)
FLASH = (255, 226, 130, 255)
FLASH_W = (255, 250, 214, 255)

GROUND_Y = 35
HEAD = (20, 17)
AIMS = (0, 45, 90, 135, 180)


class Canvas:
    def __init__(self, size=CELL):
        self.size = size
        self.img = Image.new("RGBA", (size, size), CLEAR)
        self.d = ImageDraw.Draw(self.img)

    def px(self, x, y, c):
        x, y = round(x), round(y)
        if 0 <= x < self.size and 0 <= y < self.size:
            self.img.putpixel((x, y), c)

    def rect(self, x0, y0, x1, y1, c):
        self.d.rectangle((round(x0), round(y0), round(x1), round(y1)), fill=c)

    def ellipse(self, x0, y0, x1, y1, c):
        self.d.ellipse((round(x0), round(y0), round(x1), round(y1)), fill=c)

    def line(self, a, b, c, width=1):
        self.d.line((round(a[0]), round(a[1]), round(b[0]), round(b[1])), fill=c, width=width)

    def outline(self):
        src = self.img.copy()
        for y in range(self.size):
            for x in range(self.size):
                if src.getpixel((x, y))[3]:
                    continue
                for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    nx, ny = x + dx, y + dy
                    if 0 <= nx < self.size and 0 <= ny < self.size and src.getpixel((nx, ny))[3]:
                        self.img.putpixel((x, y), INK)
                        break


def bone_line(cv, a, b):
    cv.line(a, b, BONE_D, 3)
    cv.line((a[0] - 1, a[1]), (b[0] - 1, b[1]), BONE, 1)
    cv.px(a[0] - 1, a[1], BONE_L)
    cv.ellipse(b[0] - 2, b[1] - 1, b[0] + 1, b[1] + 1, BONE)


def tripod(cv, dx=0, dy=0, spread=0):
    hx, hy = HEAD[0] + dx, HEAD[1] + dy
    bone_line(cv, (hx - 2, hy + 6), (10 - spread, GROUND_Y + dy))
    bone_line(cv, (hx + 2, hy + 6), (30 + spread, GROUND_Y + dy))
    cv.rect(hx - 1, hy + 6, hx, GROUND_Y - 3 + dy, BONE_D)
    cv.px(hx - 1, hy + 8, BONE)
    cv.px(hx - 1, GROUND_Y - 4 + dy, BONE)
    cv.ellipse(hx - 3, hy + 5, hx + 3, hy + 8, BONE_D)
    cv.line((hx - 2, hy + 5), (hx + 1, hy + 5), BONE, 1)


def skull(cv, hx, hy, aim, recoil=0):
    side = math.sin(math.radians(aim))
    back = aim >= 135
    shift = round(side * 2) - recoil * (1 if side >= 0 else -1) * (1 if abs(side) > 0.5 else 0)
    x = hx + shift
    y = hy + (recoil if aim < 100 else 0)
    cv.ellipse(x - 7, y - 7, x + 7, y + 5, BONE_D)
    cv.ellipse(x - 7, y - 7, x + 6, y + 4, BONE)
    cv.ellipse(x - 5, y - 6, x + 1, y - 2, BONE_L)
    if back:
        cv.line((x - 1, y - 7), (x + 1, y - 3), BONE_D, 1)
        cv.px(x + 2, y - 2, BONE_D)
        return
    face = aim <= 45
    if face:
        for ex in (x - 4, x + 1):
            cv.rect(ex, y - 2, ex + 2, y, SOCKET)
            cv.px(ex + 1, y - 1, EMBER)
        cv.rect(x - 1, y + 1, x, y + 1, BONE_D)
    else:
        ex = x + (3 if side > 0 else -5)
        cv.rect(ex, y - 2, ex + 2, y, SOCKET)
        cv.px(ex + 1, y - 1, EMBER)
        jaw_x = x + (2 if side > 0 else -6)
        cv.rect(jaw_x, y + 4, jaw_x + 4, y + 6, BONE_D)
        cv.rect(jaw_x, y + 4, jaw_x + 3, y + 4, BONE)


def barrel(cv, hx, hy, aim, recoil=0):
    side = math.sin(math.radians(aim))
    front = math.cos(math.radians(aim))
    length = 13 - recoil * 3
    tip = (hx + side * length, hy + 1 + front * length * 0.6)
    start = (hx + side * 2, hy + 2 + front * 2)
    cv.line(start, tip, IRON_D, 5)
    cv.line((start[0], start[1] - 1), (tip[0], tip[1] - 1), IRON, 3)
    cv.line((start[0], start[1] - 2), (tip[0], tip[1] - 2), IRON_L, 1)
    cv.ellipse(tip[0] - 3, tip[1] - 3, tip[0] + 3, tip[1] + 3, IRON_D)
    cv.ellipse(tip[0] - 2, tip[1] - 2, tip[0] + 2, tip[1] + 2, IRON)
    cv.ellipse(tip[0] - 1, tip[1] - 1, tip[0] + 1, tip[1] + 1, IRON_D)
    return tip


def flash(cv, tip, aim):
    side = math.sin(math.radians(aim))
    front = math.cos(math.radians(aim))
    fx, fy = tip[0] + side * 3, tip[1] + front * 2
    cv.ellipse(fx - 3, fy - 3, fx + 3, fy + 3, FLASH)
    cv.ellipse(fx - 1, fy - 1, fx + 1, fy + 1, FLASH_W)


def turret(cv, aim, recoil=0, shot=False, dx=0, dy=0):
    tripod(cv, dx, dy, spread=recoil)
    hx, hy = HEAD[0] + dx, HEAD[1] + dy
    back = aim >= 135
    if back:
        tip = barrel(cv, hx, hy, aim, recoil)
        skull(cv, hx, hy, aim, recoil)
    else:
        skull(cv, hx, hy, aim, recoil)
        tip = barrel(cv, hx, hy, aim, recoil)
    if shot:
        flash(cv, tip, aim)


def mound(cv, spread, height):
    cx = 20
    for x in range(cx - spread, cx + spread + 1):
        top = GROUND_Y + 2 - round(height * (1 - abs(x - cx) / (spread + 1)))
        cv.rect(x, top, x, GROUND_Y + 3, DIRT)
        cv.px(x, top, DIRT_D) if (x + top) % 2 == 0 else None
    for x in range(cx - spread, cx + spread + 1):
        cv.px(x, GROUND_Y + 3, DIRT_D)


def rise_frame(index):
    cv = Canvas()
    offsets = (24, 17, 10, 4, 0)
    turret(cv, 0, dy=offsets[index])
    mask = Image.new("RGBA", (CELL, CELL), CLEAR)
    ImageDraw.Draw(mask).rectangle((0, 0, CELL, GROUND_Y + 1), fill=(255, 255, 255, 255))
    keep = Image.new("RGBA", (CELL, CELL), CLEAR)
    keep.paste(cv.img, (0, 0), mask)
    cv.img = keep
    cv.d = ImageDraw.Draw(cv.img)
    heights = (5, 6, 5, 4, 2)
    spreads = (7, 10, 12, 12, 11)
    if index < 5:
        mound(cv, spreads[index], heights[index])
    crumbs = (((14, 28), (26, 30)), ((10, 31), (30, 29), (16, 25)), ((8, 33), (32, 32)), ((12, 34),), ())
    for x, y in crumbs[index]:
        cv.px(x, y, DIRT)
    cv.outline()
    return cv.img


def crumble_frame(index):
    cv = Canvas()
    if index == 0:
        turret(cv, 0)
        cv.px(18, 14, BONE_D)
        cv.px(19, 15, BONE_D)
        cv.px(21, 12, INK)
    elif index == 1:
        bone_line(cv, (13, 23), (7, GROUND_Y))
        bone_line(cv, (27, 22), (34, GROUND_Y - 1))
        cv.rect(19, 24, 20, GROUND_Y - 3, BONE_D)
        skull(cv, 20, 21, 0)
        barrel(cv, 20, 21, 0)
    elif index == 2:
        bone_line(cv, (11, 33), (5, GROUND_Y))
        bone_line(cv, (26, 31), (35, GROUND_Y))
        cv.rect(18, 29, 19, 33, BONE_D)
        skull(cv, 20, 28, 0)
        cv.line((26, 31), (33, 34), IRON_D, 4)
        cv.line((26, 30), (32, 33), IRON, 2)
    elif index == 3:
        bone_line(cv, (9, 34), (4, GROUND_Y))
        bone_line(cv, (29, 34), (35, GROUND_Y))
        skull(cv, 19, 31, 0)
        cv.line((26, 34), (34, 35), IRON_D, 4)
        cv.line((26, 33), (33, 34), IRON, 2)
        for x, y in ((8, 28), (31, 29), (14, 26), (26, 27)):
            cv.px(x, y, (130, 110, 100, 255))
    else:
        cv.ellipse(8, 32, 32, 37, BONE_D)
        cv.ellipse(9, 31, 31, 36, BONE)
        skull(cv, 22, 31, 0)
        cv.line((10, 34), (16, 35), BONE_D, 2)
        cv.line((26, 35), (33, 36), IRON_D, 3)
        cv.px(14, 33, BONE_L)
        cv.px(20, 29, BONE_L)
    cv.outline()
    return cv.img


def aim_frames(aim):
    out = []
    for recoil, shot in ((0, False), (1, True), (2, False)):
        cv = Canvas()
        turret(cv, aim, recoil, shot)
        cv.outline()
        out.append(cv.img)
    return out


def rotated_shard(angle, spin_tint):
    cv = Canvas(SHARD)
    c = (SHARD - 1) / 2
    ca, sa = math.cos(angle), math.sin(angle)
    for y in range(SHARD):
        for x in range(SHARD):
            dx, dy = x - c, y - c
            u = dx * ca + dy * sa
            v = -dx * sa + dy * ca
            if u < -5.0 or u > 6.0:
                continue
            width = 2.0 if u < 1.0 else 2.0 * (6.0 - u) / 5.0
            if u < -3.0:
                width = max(width, 2.2)
            if abs(v) > width:
                continue
            cv.px(x, y, BONE_L if v < -0.7 else BONE_D if v > 0.9 else BONE)
    cv.px(c - 1.5 * ca - 0.5, c - 1.5 * sa - 0.5, BONE_D)
    if spin_tint:
        cv.px(c + 3 * ca, c + 3 * sa, BONE_L)
    cv.outline()
    return cv.img


def impact_frame(index):
    cv = Canvas(SHARD)
    c = 7.5
    radius = (2, 4, 6, 7)[index]
    chips = 6
    for k in range(chips):
        a = k * math.tau / chips + 0.4
        x, y = c + radius * math.cos(a), c + radius * math.sin(a)
        if index < 3:
            cv.px(x, y, BONE_L if k % 2 else BONE)
            cv.px(x + 1, y, BONE_D)
        elif k % 2 == 0:
            cv.px(x, y, BONE_D)
    if index < 2:
        cv.ellipse(c - 1, c - 1, c + 1, c + 1, FLASH_W if index == 0 else FLASH)
    return cv.img


def main():
    sheet = Image.new("RGBA", (CELL * COLUMNS, CELL * ROWS), CLEAR)
    for i in range(5):
        sheet.paste(rise_frame(i), (i * CELL, 0))
        sheet.paste(crumble_frame(i), (i * CELL, CELL))
    for row, aim in enumerate(AIMS):
        for i, frame in enumerate(aim_frames(aim)):
            sheet.paste(frame, (i * CELL, (2 + row) * CELL))
    OUT.mkdir(parents=True, exist_ok=True)
    sheet.save(OUT / "turret.tga")

    shards = Image.new("RGBA", (SHARD * SHARD_FRAMES, SHARD * 2), CLEAR)
    for i in range(SHARD_FRAMES):
        shards.paste(rotated_shard(i * math.pi / SHARD_FRAMES + 0.3, i % 2), (i * SHARD, 0))
        shards.paste(impact_frame(i), (i * SHARD, SHARD))
    shards.save(OUT / "bone_shard.tga")
    print("wrote", OUT / "turret.tga", OUT / "bone_shard.tga")


if __name__ == "__main__":
    main()
