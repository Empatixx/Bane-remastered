import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
OUTLINE = (24, 16, 18, 255)
ROCK = (64, 48, 50, 255)
ROCK_L = (98, 76, 72, 255)
ROCK_D = (42, 32, 36, 255)
LAVA = (232, 84, 24, 255)
LAVA_H = (255, 150, 44, 255)
LAVA_C = (255, 214, 96, 255)
LAVA_D = (168, 44, 20, 255)
BONE = (214, 200, 170, 255)
BONE_D = (150, 134, 112, 255)
EYE = (255, 236, 150, 255)
W, H = 128, 84


def ellipse(d, cx, cy, rx, ry, fill):
    for y in range(H):
        for x in range(W):
            if ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 <= 1.0:
                d.point((x, y), fill=fill)


def outline(image):
    result = image.copy()
    for y in range(H):
        for x in range(W):
            if image.getpixel((x, y))[3]:
                continue
            for ox, oy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + ox, y + oy
                if 0 <= nx < W and 0 <= ny < H and image.getpixel((nx, ny))[3]:
                    result.putpixel((x, y), OUTLINE)
                    break
    return result


def rot(points, angle, cx, cy, scale=1.0):
    c, s = math.cos(angle), math.sin(angle)
    return [(cx + (x * c - y * s) * scale, cy + (x * s + y * c) * scale) for x, y in points]


def head(d, cx, cy, angle, scale=1.0, jaw=0.0, fire=False):
    skull = [(-8, -6), (2, -7), (10, -3), (17, -1), (17, 3), (8, 5), (-2, 6), (-8, 3)]
    lower = [(-2, 5), (15, 5 + jaw), (13, 9 + jaw), (0, 9 + jaw * 0.5)]
    d.polygon(rot(lower, angle, cx, cy, scale), fill=ROCK_D)
    d.polygon(rot(skull, angle, cx, cy, scale), fill=ROCK)
    d.polygon(rot([(-8, -6), (2, -7), (6, -4), (-4, -1), (-8, 3)], angle, cx, cy, scale), fill=ROCK_L)
    d.polygon(rot([(-6, -6), (-14, -13), (-2, -8)], angle, cx, cy, scale), fill=BONE)
    d.polygon(rot([(-1, -7), (-5, -15), (4, -8)], angle, cx, cy, scale), fill=BONE_D)
    for tx in (9, 13):
        d.polygon(rot([(tx, 5), (tx + 2, 5), (tx + 1, 8 + jaw)], angle, cx, cy, scale), fill=BONE)
    d.polygon(rot([(4, -2), (9, -3), (7, 0), (3, 0)], angle, cx, cy, scale), fill=LAVA_D)
    ex, ey = rot([(3, -3)], angle, cx, cy, scale)[0]
    d.rectangle((round(ex), round(ey), round(ex) + 2, round(ey) + 1), fill=EYE)
    d.point((round(ex) + 1, round(ey) + 2), fill=LAVA_H)
    nx, ny = rot([(16, 0)], angle, cx, cy, scale)[0]
    d.rectangle((round(nx) - 1, round(ny) - 1, round(nx), round(ny)), fill=LAVA)
    if fire:
        for step, colour in ((1, LAVA_C), (2, LAVA_H), (3, LAVA)):
            fx, fy = rot([(17 + step * 3, 3 + jaw * 0.6)], angle, cx, cy, scale)[0]
            d.ellipse((fx - step, fy - step, fx + step, fy + step), fill=colour)


def neck(d, a, b, width, fill):
    d.line((a[0], a[1], b[0], b[1]), fill=fill, width=width)
    ellipse(d, b[0], b[1], width / 2, width / 2, fill)


def cracks(d, pts, fill=LAVA):
    for a, b in zip(pts, pts[1:]):
        d.line((a[0], a[1], b[0], b[1]), fill=fill)


def hound(bob=0, legs=((0, 0), (0, 0), (0, 0), (0, 0)), jaws=(0, 0, 0), breath=0.0, tail=0, turn=(0.0, 0.0, 0.0), lean=0, drop=(0, 0, 0), fiery=False, pitch=0):
    image = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(image)
    ground = 76
    tail_points = [(18, 44 + bob), (10, 46 + bob), (5, 40 + bob), (4, 32 + bob - tail), (8, 24 + bob - tail * 2), (14, 20 + bob - tail * 2)]
    for index, (a, b2) in enumerate(zip(tail_points, tail_points[1:])):
        width = max(2, 7 - index)
        d.line((a[0], a[1], b2[0], b2[1]), fill=ROCK_D, width=width)
        ellipse(d, b2[0], b2[1], width / 2, width / 2, ROCK_D)
        d.line((a[0], a[1] - 1, b2[0], b2[1] - 1), fill=LAVA_D if index % 2 else LAVA)
    tx, ty = tail_points[-1]
    for step, colour in ((5, LAVA), (4, LAVA_H), (2, LAVA_C)):
        d.polygon([(tx - step, ty + 2), (tx + step, ty + 2), (tx + 1, ty - step * 2 - 2)], fill=colour)

    def leg(x, top, hind, lift, step):
        x += step
        knee = (x + (4 if hind else -2), top + 12)
        paw = (x + (-1 if hind else 1) + step // 2, ground - lift)
        d.line((x, top, knee[0], knee[1]), fill=ROCK, width=8 if hind else 7)
        d.line((knee[0], knee[1], paw[0], paw[1] - 3), fill=LAVA_D if fiery else ROCK_D, width=5)
        if fiery:
            d.line((knee[0], knee[1], paw[0], paw[1] - 3), fill=LAVA, width=1)
        ellipse(d, knee[0], knee[1], 4, 4, ROCK)
        d.rectangle((paw[0] - 5, paw[1] - 3, paw[0] + 6, paw[1]), fill=ROCK_D)
        d.line((paw[0] - 5, paw[1] - 3, paw[0] + 6, paw[1] - 3), fill=ROCK_L)
        for cx in (paw[0] - 5, paw[0] - 1, paw[0] + 3, paw[0] + 6):
            d.rectangle((cx, paw[1] + 1, cx, paw[1] + 2), fill=BONE)
        if fiery:
            for colour, size in ((LAVA, 1.0), (LAVA_H, 0.7), (LAVA_C, 0.4)):
                d.polygon([(paw[0] + 5, paw[1] + 1), (paw[0] - 5 - 12 * size, paw[1] - 8 * size), (paw[0] - 2 - 6 * size, paw[1] - 1), (paw[0] - 8 - 14 * size, paw[1] + 1), (paw[0] + 4, paw[1] + 3)], fill=colour)

    for (x, top, hind), (lift, step) in zip(((22, 48, True), (34, 50, True), (56, 50, False), (66, 50, False)), legs):
        leg(x, top + bob, hind, lift, step)
    ellipse(d, 42 + lean, 40 + bob - pitch // 2, 28, 14, ROCK)
    ellipse(d, 38 + lean, 36 + bob - pitch // 2, 20, 8, ROCK_L)
    ellipse(d, 24, 42 + bob + pitch, 12, 13, ROCK)
    ellipse(d, 58 + lean, 38 + bob - pitch, 14, 14, ROCK)
    cracks(d, [(16, 44 + bob), (22, 40 + bob), (26, 46 + bob), (32, 42 + bob), (38, 48 + bob)])
    cracks(d, [(44, 34 + bob), (50, 38 + bob), (54, 33 + bob), (60, 40 + bob), (66, 36 + bob)], LAVA_H)
    cracks(d, [(30, 52 + bob), (36, 50 + bob), (42, 54 + bob), (48, 51 + bob)], LAVA_D)
    for x in range(20, 64, 5):
        y = 26 + bob + abs(x - 40) // 6
        d.polygon([(x, y + 4), (x + 4, y + 4), (x + 2, y - 5 - (x % 3))], fill=ROCK_D)
        d.polygon([(x + 1, y + 1), (x + 3, y + 1), (x + 2, y - 3)], fill=LAVA)
    heads = (
        ((62, 36), (78, 44), 10, ROCK_D, (86, 46), 0.45, 0.95, 2),
        ((62, 28), (72, 16), 10, ROCK_D, (79, 11), -0.6, 0.95, 1),
        ((62, 32), (80, 31), 12, ROCK, (90, 31), 0.0, 1.15, 0),
    )
    for base, joint, width, colour, centre, angle, size, index in heads:
        dy = drop[index]
        neck(d, (base[0] + lean, base[1] + bob - pitch), (joint[0] + lean, joint[1] + bob + dy - pitch), width, colour)
        head(d, centre[0] + lean, centre[1] + bob + dy - pitch, angle + turn[index], size, jaws[index], False)
        if breath > 0:
            fx, fy = rot([(17, 2 + jaws[index] * 0.5)], angle + turn[index], centre[0] + lean, centre[1] + bob + dy - pitch, size)[0]
            flame(d, fx, fy, angle + turn[index], breath)
    d.rectangle((60 + lean, 24 + bob, 66 + lean, 34 + bob), fill=BONE_D)
    for y in (26, 30):
        d.point((63 + lean, y + bob), fill=BONE)
    return outline(image)


def flame(d, x, y, angle, amount):
    length = 8 + 24 * amount
    for colour, scale in ((LAVA, 1.0), (LAVA_H, 0.7), (LAVA_C, 0.4)):
        reach = length * scale
        half = (2 + 5 * amount) * scale
        d.polygon(rot([(0, -half * 0.5), (reach, -half), (reach + 3, 0), (reach, half), (0, half * 0.5)], angle, x, y), fill=colour)


def run():
    frames = []
    for index in range(8):
        phase = index / 8 * math.tau
        legs = []
        for leg, offset in enumerate((0.0, math.pi, math.pi, 0.0)):
            angle = phase + offset + (0.0 if leg < 2 else 0.4)
            reach = 8 if leg < 2 else 7
            legs.append((round(max(0.0, math.sin(angle)) * 8), round(math.cos(angle) * reach)))
        frames.append(
            hound(
                bob=9 + round(abs(math.sin(phase)) * 3) - 1,
                legs=tuple(legs),
                jaws=(2 + (index % 4 == 1), 1, 2),
                tail=round(math.sin(phase + 1) * 3),
                turn=(math.sin(phase * 2) * 0.1, math.sin(phase * 2 + 1) * 0.12, math.sin(phase * 2 + 2) * 0.1),
                lean=round(math.sin(phase) * 2),
                pitch=round(math.sin(phase * 2 + 1) * 3),
                drop=(round(math.sin(phase * 2)), round(math.sin(phase * 2 + 1)), round(math.sin(phase * 2 + 2))),
            )
        )
    return frames


def lunge():
    poses = [
        dict(bob=12, legs=((0, -3), (0, -3), (0, 3), (0, 3)), lean=-3, jaws=(0, 0, 0), drop=(2, 2, 2)),
        dict(bob=15, legs=((0, -5), (0, -5), (0, 4), (0, 4)), lean=-5, jaws=(1, 1, 1), tail=3, drop=(3, 3, 3)),
        dict(bob=5, legs=((8, 8), (8, 8), (9, -8), (9, -8)), lean=4, jaws=(5, 4, 4), tail=-1, turn=(0.1, 0.15, 0.1), fiery=True),
        dict(bob=2, legs=((10, 10), (10, 10), (11, -10), (11, -10)), lean=6, jaws=(6, 5, 5), tail=-2, turn=(0.1, 0.2, 0.1), fiery=True),
        dict(bob=4, legs=((9, 8), (9, 8), (10, -8), (10, -8)), lean=6, jaws=(6, 5, 5), tail=-2, turn=(0.1, 0.2, 0.1), fiery=True),
        dict(bob=11, legs=((0, 4), (0, 4), (0, -2), (0, -2)), lean=2, jaws=(3, 2, 2), tail=2, drop=(2, 2, 2), fiery=True),
        dict(bob=11, legs=((0, 2), (0, 2), (0, -1), (0, -1)), jaws=(1, 1, 1), tail=1),
        dict(bob=9, jaws=(1, 1, 1)),
    ]
    return [hound(**pose) for pose in poses]


def breath():
    poses = [
        dict(bob=9, jaws=(1, 1, 1)),
        dict(bob=10, lean=-2, jaws=(4, 4, 4), turn=(-0.25, -0.25, -0.3), drop=(-2, -3, -2)),
        dict(bob=10, lean=-1, jaws=(6, 5, 5), breath=0.4, turn=(0.0, -0.05, 0.1)),
        dict(bob=10, jaws=(7, 6, 6), breath=0.8, turn=(0.0, -0.1, 0.15), drop=(1, 0, 1)),
        dict(bob=10, jaws=(7, 6, 6), breath=1.0, turn=(0.0, -0.1, 0.15), drop=(-1, 1, -1)),
        dict(bob=10, jaws=(7, 6, 6), breath=1.0, turn=(0.0, -0.1, 0.15), drop=(1, -1, 1)),
        dict(bob=10, jaws=(6, 5, 5), breath=0.6, turn=(0.0, -0.05, 0.1)),
        dict(bob=9, jaws=(2, 2, 2)),
    ]
    return [hound(**pose) for pose in poses]


def howl():
    poses = [
        dict(bob=9, jaws=(1, 1, 1)),
        dict(bob=8, lean=-2, jaws=(3, 3, 3), turn=(-0.35, -0.3, -0.4), drop=(-2, -2, -3)),
        dict(bob=7, lean=-3, jaws=(7, 7, 7), turn=(-0.7, -0.5, -0.8), drop=(-4, -3, -5), tail=3),
        dict(bob=7, lean=-3, jaws=(8, 8, 8), turn=(-0.75, -0.55, -0.85), drop=(-5, -3, -6), tail=4),
        dict(bob=7, lean=-3, jaws=(7, 7, 7), turn=(-0.7, -0.5, -0.8), drop=(-4, -4, -5), tail=3),
        dict(bob=8, lean=-2, jaws=(5, 5, 5), turn=(-0.4, -0.3, -0.5), drop=(-2, -2, -3)),
        dict(bob=9, jaws=(2, 2, 2)),
        dict(bob=9, jaws=(1, 1, 1)),
    ]
    return [hound(**pose) for pose in poses]


EMBER = (LAVA_C, LAVA_H, LAVA)
CHUNK = (LAVA, LAVA, LAVA_H, LAVA_D)


def blast(base, cells, spread, fall, keep, seed, cracks_only=False, sparks=20):
    rng = random.Random(seed)
    box = base.getbbox() or (0, 0, W, H)
    seeds = [(rng.uniform(box[0], box[2]), rng.uniform(box[1], box[3])) for _ in range(cells)]
    kept = [rng.random() < keep or index < 4 for index in range(cells)]
    tones = [rng.choice(CHUNK) for _ in range(cells)]
    centre = ((box[0] + box[2]) / 2, (box[1] + box[3]) / 2 + 6)
    shifts = []
    for sx, sy in seeds:
        dx, dy = sx - centre[0], sy - centre[1]
        length = math.hypot(dx, dy) or 1.0
        power = spread * rng.uniform(0.6, 1.2)
        shifts.append((round(dx / length * power), round(dy / length * power * 0.6 + fall * rng.uniform(0.7, 1.1))))
    cell_of = {}
    for y in range(H):
        for x in range(W):
            if base.getpixel((x, y))[3]:
                cell_of[(x, y)] = min(range(cells), key=lambda i: (x - seeds[i][0]) ** 2 + (y - seeds[i][1]) ** 2)

    def near_edge(x, y):
        return any((x + ox, y + oy) not in cell_of for ox in (-2, -1, 0, 1, 2) for oy in (-2, 0, 2))

    chunks = Image.new("RGBA", (W, H), CLEAR)
    for (x, y), index in cell_of.items():
        if not kept[index]:
            continue
        colour = base.getpixel((x, y))
        if colour == OUTLINE:
            colour = ROCK_D
        if cracks_only:
            if any(cell_of.get((x + ox, y + oy), index) != index for ox in (-1, 0, 1) for oy in (-1, 0, 1)):
                colour = LAVA_H
        elif not near_edge(x, y) and colour not in (BONE, BONE_D):
            colour = tones[index]
        dx, dy = shifts[index]
        nx, ny = x + dx, min(y + dy, H - 4)
        if 1 <= nx < W - 1 and 1 <= ny < H - 1:
            chunks.putpixel((nx, ny), colour)
    chunks = outline(chunks)
    out = chunks
    d = ImageDraw.Draw(out)
    for _ in range(sparks):
        angle = rng.uniform(0, math.tau)
        reach = rng.uniform(4, spread * 1.5 + 6)
        x, y = round(centre[0] + math.cos(angle) * reach), round(centre[1] - 8 + math.sin(angle) * reach * 0.8 + fall * 0.4)
        if 1 <= x < W - 1 and 1 <= y < H - 3:
            size = rng.choice((1, 1, 2))
            d.rectangle((x, y, x + size - 1, y + size - 1), fill=rng.choice(EMBER))
    return out


def death():
    fallen = hound(bob=17, legs=((0, 4), (6, -4), (0, 3), (5, -3)), jaws=(5, 5, 5), turn=(0.2, 0.3, 0.3), drop=(5, 6, 6), tail=-2)
    frames = [
        hound(bob=12, legs=((0, -4), (0, -4), (0, 4), (0, 4)), lean=-3, jaws=(7, 7, 7), turn=(-0.4, -0.3, -0.4)),
        hound(bob=14, legs=((0, 5), (3, -3), (0, 4), (3, -2)), lean=2, jaws=(6, 6, 6), drop=(4, 4, 4)),
        fallen,
    ]
    for cells, spread, fall, keep, sparks in ((10, 5, 0, 1.0, 8), (16, 15, 5, 0.9, 24), (22, 23, 14, 0.7, 36), (28, 30, 26, 0.5, 30), (30, 34, 34, 0.35, 24)):
        frames.append(blast(fallen, cells, spread, fall, keep, 11 + cells, cracks_only=spread == 5, sparks=sparks))
    return frames


CRUST = (52, 22, 22, 255)


def lava_drop_sheet():
    size = 16
    sheet = Image.new("RGBA", (size * 4, size), CLEAR)
    d = ImageDraw.Draw(sheet)
    for frame in range(4):
        cx, cy = frame * size + 8, 9
        radius = 5 + (frame % 2)
        d.ellipse((cx - radius - 1, cy - radius + 1, cx + radius + 1, cy + radius), fill=CRUST)
        d.ellipse((cx - radius, cy - radius + 1, cx + radius, cy + radius - 1), fill=LAVA_D)
        d.ellipse((cx - radius + 2, cy - radius + 2, cx + radius - 2, cy + radius - 2), fill=LAVA)
        d.ellipse((cx - 2, cy - 2, cx + 2, cy + 1), fill=LAVA_H)
        d.point((cx - 1 + frame % 3, cy - 1), fill=LAVA_C)
    return sheet


def spark_sheet():
    sheet = Image.new("RGBA", (6 * 4, 6), CLEAR)
    d = ImageDraw.Draw(sheet)
    for frame, (size, colour) in enumerate(((5, LAVA_C), (4, LAVA_H), (3, LAVA), (2, LAVA_D))):
        d.rectangle((frame * 6, 0, frame * 6 + size - 1, size - 1), fill=colour)
    return sheet


def flame_sheet():
    size = 16
    sheet = Image.new("RGBA", (size * 4, size * 2), CLEAR)
    d = ImageDraw.Draw(sheet)
    for frame in range(4):
        cx, cy = frame * size + 8, 8
        for tongue in range(6):
            angle = tongue * math.tau / 6 + frame * 0.5
            reach = 5.5 + ((tongue + frame) % 3) * 1.4
            tip = (cx + math.cos(angle) * reach, cy + math.sin(angle) * reach)
            side = (math.cos(angle + math.pi / 2) * 2.2, math.sin(angle + math.pi / 2) * 2.2)
            d.polygon([(cx + side[0], cy + side[1]), (cx - side[0], cy - side[1]), tip], fill=LAVA)
        d.ellipse((cx - 4, cy - 4, cx + 4, cy + 4), fill=LAVA_H)
        d.ellipse((cx - 2.4, cy - 2.4, cx + 2.4, cy + 2.4), fill=LAVA_C)
        d.point((cx - 1, cy - 1), fill=(255, 250, 220, 255))
    for frame in range(4):
        cx, cy = frame * size + 8, size + 8
        reach = 2 + frame * 1.7
        for index in range(7):
            angle = index * math.tau / 7 + 0.3
            x, y = cx + math.cos(angle) * reach, cy + math.sin(angle) * reach
            colour = (LAVA_C, LAVA_H, LAVA, LAVA_D)[frame]
            d.rectangle((x, y, x + (1 if frame > 1 else 2), y + (1 if frame > 1 else 2)), fill=colour)
        if frame < 2:
            d.ellipse((cx - 3 + frame, cy - 3 + frame, cx + 3 - frame, cy + 3 - frame), fill=LAVA_H)
    return sheet


def main():
    rows = [run(), lunge(), breath(), howl(), death()]
    sheet = Image.new("RGBA", (W * 8, H * len(rows)), CLEAR)
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * W, row * H))
    sheet.save(OUT / "hellhound.tga")
    lava_drop_sheet().save(OUT / "hound_lava.tga")
    spark_sheet().save(OUT / "hound_spark.tga")
    flame_sheet().save(OUT / "hound_flame.tga")


main()
