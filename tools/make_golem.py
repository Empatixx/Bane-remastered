import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

W = 96
OUTLINE = (30, 26, 36, 255)
STONE_D = (74, 70, 84, 255)
STONE = (116, 112, 124, 255)
STONE_L = (158, 154, 166, 255)
MOSS = (86, 112, 62, 255)
MOSS_L = (122, 150, 82, 255)
GLOW = (255, 214, 92, 255)
GLOW_HOT = (255, 248, 190, 255)
GLOW_D = (226, 128, 44, 255)
DARK = (22, 18, 28, 255)


def block(d, x0, y0, x1, y1, seed=0, moss=False):
    d.rectangle((x0, y0, x1, y1), fill=STONE, outline=OUTLINE)
    d.line([(x0 + 1, y0 + 1), (x1 - 1, y0 + 1)], fill=STONE_L)
    d.line([(x0 + 1, y0 + 1), (x0 + 1, y1 - 1)], fill=STONE_L)
    d.line([(x0 + 1, y1 - 1), (x1 - 1, y1 - 1)], fill=STONE_D)
    d.line([(x1 - 1, y0 + 2), (x1 - 1, y1 - 1)], fill=STONE_D)
    r = random.Random(seed)
    for _ in range(max(1, (x1 - x0) * (y1 - y0) // 90)):
        x, y = r.randint(x0 + 3, max(x0 + 3, x1 - 3)), r.randint(y0 + 3, max(y0 + 3, y1 - 3))
        d.point((x, y), fill=STONE_D if r.random() < 0.6 else STONE_L)
    if moss:
        for k in range(3):
            d.point((x0 + 2 + k * 2, y0 + 2), fill=MOSS)
        d.point((x0 + 3, y0 + 3), fill=MOSS_L)
        d.point((x0 + 5, y0 + 3), fill=MOSS)


def limb(d, a, b, thick, seed=0):
    ax, ay = a
    bx, by = b
    length = math.hypot(bx - ax, by - ay) or 1
    nx, ny = -(by - ay) / length * thick / 2, (bx - ax) / length * thick / 2
    pts = [(ax + nx, ay + ny), (bx + nx, by + ny), (bx - nx, by - ny), (ax - nx, ay - ny)]
    d.polygon(pts, fill=STONE, outline=OUTLINE)
    mx, my = (ax + bx) / 2, (ay + by) / 2
    d.line([(ax + nx * 0.6, ay + ny * 0.6), (bx + nx * 0.6, by + ny * 0.6)], fill=STONE_L)
    d.line([(ax - nx * 0.6, ay - ny * 0.6), (bx - nx * 0.6, by - ny * 0.6)], fill=STONE_D)
    d.point((mx, my), fill=STONE_D)


def fist(d, c, size=10):
    x, y = c
    block(d, x - size, y - size, x + size, y + size, int(x * 7 + y), False)


def golem(*, bob=0, left_leg=0, right_leg=0, left_hand=(14, 70), right_hand=(82, 70), head_dy=0, eyes=1.0, core=1.0, lean=0, rage=False, beam=False):
    im = Image.new("RGBA", (W, W), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    cx = 48 + lean
    ground = 90
    # legs
    block(d, cx - 22, 62 + bob - left_leg, cx - 6, ground - left_leg, 1)
    block(d, cx + 6, 62 + bob - right_leg, cx + 22, ground - right_leg, 2)
    block(d, cx - 24, ground - 6 - left_leg, cx - 4, ground - left_leg, 3)
    block(d, cx + 4, ground - 6 - right_leg, cx + 24, ground - right_leg, 4)
    # arms (behind torso)
    left_shoulder, right_shoulder = (cx - 28, 40 + bob), (cx + 28, 40 + bob)
    limb(d, left_shoulder, left_hand, 14)
    limb(d, right_shoulder, right_hand, 14)
    # torso
    block(d, cx - 26, 32 + bob, cx + 26, 66 + bob, 5, True)
    block(d, cx - 20, 58 + bob, cx + 20, 68 + bob, 6)
    # shoulders
    block(d, cx - 38, 30 + bob, cx - 18, 46 + bob, 7, True)
    block(d, cx + 18, 30 + bob, cx + 38, 46 + bob, 8)
    # chest core
    c = int(5 * core)
    d.polygon([(cx, 44 + bob - c), (cx + c + 2, 50 + bob), (cx, 56 + bob + c), (cx - c - 2, 50 + bob)], fill=GLOW if core > 0.3 else GLOW_D, outline=OUTLINE)
    d.point((cx, 50 + bob), fill=GLOW_HOT)
    # head
    hy = 10 + bob + head_dy
    block(d, cx - 13, hy, cx + 13, hy + 22, 9, True)
    eye = GLOW_HOT if beam else GLOW
    if rage:
        eye = (255, 96, 60, 255)
    if eyes <= 0.2:
        eye = DARK
    d.rectangle((cx - 9, hy + 8, cx - 4, hy + 11), fill=eye)
    d.rectangle((cx + 4, hy + 8, cx + 9, hy + 11), fill=eye)
    if eyes > 1.2:
        d.rectangle((cx - 10, hy + 7, cx - 3, hy + 12), outline=GLOW_D)
        d.rectangle((cx + 3, hy + 7, cx + 10, hy + 12), outline=GLOW_D)
    d.line([(cx - 6, hy + 17), (cx + 6, hy + 17)], fill=DARK)
    # hands in front
    fist(d, left_hand)
    fist(d, right_hand)
    return im


def sheet():
    cols, rows = 7, 6
    out = Image.new("RGBA", (W * cols, W * rows), (0, 0, 0, 0))

    def put(row, col, frame):
        out.paste(frame, (col * W, row * W))

    for i in range(6):
        phase = i / 6 * math.tau
        put(0, i, golem(bob=round(math.sin(phase * 2) * 1.5), left_leg=max(0, round(math.sin(phase) * 5)), right_leg=max(0, round(-math.sin(phase) * 5)), left_hand=(14 + round(math.sin(phase) * 3), 70 - round(math.sin(phase) * 4)), right_hand=(82 - round(math.sin(phase) * 3), 70 + round(math.sin(phase) * 4))))
    slam = [
        golem(left_hand=(16, 56), right_hand=(80, 56)),
        golem(left_hand=(20, 30), right_hand=(76, 30), head_dy=1, bob=1),
        golem(left_hand=(26, 6), right_hand=(70, 6), head_dy=2, bob=2, eyes=1.3),
        golem(left_hand=(30, 66), right_hand=(66, 66), bob=-3, head_dy=-2, lean=0, eyes=1.3),
        golem(left_hand=(24, 86), right_hand=(72, 86), bob=2, head_dy=2),
        golem(left_hand=(20, 82), right_hand=(76, 82), bob=1),
        golem(left_hand=(16, 74), right_hand=(80, 74)),
    ]
    for i, f in enumerate(slam):
        put(1, i, f)
    throw = [
        golem(left_hand=(14, 70), right_hand=(88, 46), lean=-1),
        golem(left_hand=(14, 70), right_hand=(92, 22), lean=-2, bob=1),
        golem(left_hand=(14, 70), right_hand=(76, 6), lean=-2, bob=1, eyes=1.3),
        golem(left_hand=(18, 66), right_hand=(60, 56), lean=1, bob=-1),
        golem(left_hand=(14, 70), right_hand=(70, 70), lean=1),
    ]
    for i, f in enumerate(throw):
        put(2, i, f)
    rain = [
        golem(left_hand=(24, 10), right_hand=(72, 10), bob=-1, eyes=1.3),
        golem(left_hand=(22, 8), right_hand=(74, 8), bob=1, eyes=1.3),
        golem(left_hand=(26, 12), right_hand=(70, 12), bob=-1, eyes=1.3),
        golem(left_hand=(22, 8), right_hand=(74, 8), bob=1, eyes=1.3),
    ]
    for i, f in enumerate(rain):
        put(3, i, f)
    laser = [
        golem(core=0.4),
        golem(core=0.8, head_dy=-1, eyes=1.2),
        golem(core=1.2, head_dy=-2, eyes=1.3, beam=True),
        golem(core=1.4, head_dy=-2, eyes=1.3, beam=True, bob=1),
        golem(core=1.2, head_dy=-2, eyes=1.3, beam=True),
        golem(core=0.8, head_dy=-1, eyes=1.2),
    ]
    for i, f in enumerate(laser):
        put(4, i, f)
    for i, f in enumerate(death_frames()):
        put(5, i, f)
    return out


def rubble(blocks, core=None):
    im = Image.new("RGBA", (W, W), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    for x0, y0, x1, y1, seed in blocks:
        block(d, x0, y0, x1, y1, seed, seed % 2 == 0)
    if core:
        cx, cy = core
        d.polygon([(cx, cy - 6), (cx + 4, cy), (cx, cy + 6), (cx - 4, cy)], fill=GLOW_D, outline=OUTLINE)
    return im


def death_frames():
    frames = [
        golem(lean=-2, bob=-1, head_dy=-2, eyes=1.3, left_hand=(12, 62), right_hand=(84, 62)),
        golem(lean=-3, bob=0, head_dy=3, eyes=0.0, core=0.5, left_hand=(10, 74), right_hand=(86, 74)),
        golem(lean=-3, bob=4, head_dy=6, eyes=0.0, core=0.3, left_hand=(8, 86), right_hand=(88, 86)),
    ]
    frames.append(
        rubble(
            [
                (30, 70, 66, 90, 1),
                (8, 78, 28, 90, 2),
                (68, 76, 90, 90, 3),
                (34, 56, 62, 72, 4),
                (40, 40, 58, 56, 5),
            ],
            (48, 66),
        )
    )
    frames.append(
        rubble(
            [
                (14, 76, 46, 90, 1),
                (50, 74, 84, 90, 2),
                (6, 82, 20, 90, 3),
                (74, 82, 90, 90, 4),
                (30, 66, 60, 78, 5),
            ],
            (46, 74),
        )
    )
    frames.append(
        rubble(
            [
                (8, 80, 40, 90, 1),
                (44, 78, 86, 90, 2),
                (28, 72, 58, 82, 3),
                (62, 72, 80, 82, 4),
            ]
        )
    )
    return frames


def rock_chunk(size, seed, turn):
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    r = random.Random(seed)
    c = size / 2
    pts = []
    for k in range(8):
        a = k * math.tau / 8 + r.uniform(-0.2, 0.2)
        rad = c * r.uniform(0.62, 0.95)
        pts.append((c + math.cos(a) * rad, c + math.sin(a) * rad))
    d.polygon(pts, fill=STONE, outline=OUTLINE)
    d.polygon([(c - c * 0.6, c - c * 0.1), (c - c * 0.1, c - c * 0.65), (c + c * 0.3, c - c * 0.35)], fill=STONE_L)
    d.point((int(c + c * 0.3), int(c + c * 0.3)), fill=STONE_D)
    d.point((int(c - c * 0.3), int(c + c * 0.4)), fill=STONE_D)
    if size >= 24:
        d.point((int(c + c * 0.2), int(c - c * 0.5)), fill=MOSS)
    return im.rotate(turn, resample=Image.NEAREST)


def burst_frame(size, step, seed):
    im = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    r = random.Random(seed + step)
    c = size / 2
    reach = size * (0.18 + 0.22 * step)
    dust = [(150, 144, 156, 255), (200, 196, 206, 255)]
    for k in range(7):
        a = k * math.tau / 7 + 0.3
        x, y = c + math.cos(a) * reach, c + math.sin(a) * reach
        w = max(1, int(size / 9 - step * 0.6))
        d.rectangle((x - w, y - w, x + w, y + w), fill=STONE if k % 2 else STONE_L, outline=OUTLINE if w > 1 else None)
    if step < 3:
        rad = size * (0.2 + 0.14 * step)
        d.ellipse((c - rad, c - rad, c + rad, c + rad), outline=dust[step % 2])
    return im


def projectile_sheet(size, seed, big):
    sheet = Image.new("RGBA", (size * 4, size * 2), (0, 0, 0, 0))
    for i in range(4):
        sheet.paste(rock_chunk(size, seed, i * 90), (i * size, 0))
        sheet.paste(burst_frame(size, i, seed), (i * size, size))
    return sheet


def rock_shadow():
    im = Image.new("RGBA", (32, 16), (0, 0, 0, 0))
    d = ImageDraw.Draw(im)
    d.ellipse((1, 1, 30, 14), fill=(24, 18, 24, 150), outline=(255, 170, 60, 200))
    return im


def spark_sheet():
    sheet = Image.new("RGBA", (16, 4), (0, 0, 0, 0))
    colours = [GLOW_HOT, GLOW, GLOW_D, (150, 84, 36, 255)]
    for i in range(4):
        d = ImageDraw.Draw(sheet)
        size = 4 - i
        d.rectangle((i * 4, 0, i * 4 + size - 1, size - 1), fill=colours[i])
    return sheet


def main():
    sheet().save(OUT / "golem.tga")
    projectile_sheet(12, 21, False).save(OUT / "golem_rock.tga")
    projectile_sheet(32, 5, True).save(OUT / "golem_boulder.tga")
    rock_shadow().save(OUT / "rock_shadow.tga")
    spark_sheet().save(OUT / "spark.tga")


main()
