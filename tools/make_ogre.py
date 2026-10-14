import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
OUTLINE = (22, 18, 20, 255)
SKIN = (74, 96, 56, 255)
SKIN_L = (102, 124, 74, 255)
SKIN_D = (50, 66, 44, 255)
BELLY = (112, 128, 84, 255)
LEATHER = (74, 48, 36, 255)
LEATHER_D = (48, 32, 28, 255)
IRON = (96, 100, 108, 255)
IRON_D = (62, 66, 76, 255)
RUST = (110, 74, 52, 255)
RUST_D = (72, 48, 38, 255)
BONE = (214, 204, 176, 255)
BONE_D = (160, 148, 124, 255)
MOUTH = (48, 18, 24, 255)
EYE = (232, 196, 72, 255)
SCAR = (128, 80, 84, 255)
TAIL = (170, 130, 104, 255)
W, H = 80, 80
COLUMNS = 8


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


def limb(d, a, b, width, fill):
    d.line((a[0], a[1], b[0], b[1]), fill=fill, width=width)
    for point in (a, b):
        ellipse(d, point[0], point[1], width / 2, width / 2, fill)


def club(d, grip, tip):
    gx, gy = grip
    tx, ty = tip
    length = math.hypot(tx - gx, ty - gy) or 1.0
    ux, uy = (tx - gx) / length, (ty - gy) / length
    nx, ny = -uy, ux

    def at(t, half):
        return (gx + ux * length * t + nx * half, gy + uy * length * t + ny * half)

    outline_points = [at(0, 2), at(0.6, 2.6), at(0.64, 6.5), at(1.0, 6.0), at(1.0, -6.0), at(0.64, -6.5), at(0.6, -2.6), at(0, -2)]
    d.polygon(outline_points, fill=RUST)
    d.polygon([at(0, 2), at(0.6, 2.6), at(0.64, 6.5), at(1.0, 6.0), at(1.0, 1.0), at(0, 0)], fill=RUST_D)
    for t in (0.68, 0.84, 0.98):
        a, b = at(t, -6.0), at(t, 6.0)
        d.line((a[0], a[1], b[0], b[1]), fill=IRON)
    for t, half in ((0.74, 3.0), (0.9, -3.0), (0.8, 0.0), (0.96, 3.5)):
        x, y = at(t, half)
        d.point((round(x), round(y)), fill=BONE)
        d.point((round(x) + 1, round(y)), fill=BONE_D)


def layer():
    image = Image.new("RGBA", (W, H), CLEAR)
    return image, ImageDraw.Draw(image)


def ogre(bob=0, lleg=0, rleg=0, lstep=0, rstep=0, larm=0, grip=(57, 47), tip=(63, 5), jaw=0, blink=False, kneel=0, rat=False, sway=0, hdx=0, hdy=0, rarm=0):
    real_bob = bob
    bob = 0
    legs, dl = layer()
    body, db = layer()
    head, dh = layer()
    d = db
    club(d, (grip[0], grip[1] + bob + rarm), (tip[0], tip[1] + bob + rarm))
    d = dl
    d = dl
    for x, lift, step in ((22, lleg, lstep), (46, rleg, rstep)):
        top = 52 + bob
        bottom = 72 - lift - kneel
        d.rectangle((x + step // 2, top, x + 11 + step // 2, max(top + 2, bottom)), fill=SKIN_D)
        d.rectangle((x + step // 2, top, x + 3 + step // 2, max(top + 2, bottom)), fill=SKIN)
        d.rectangle((x - 2 + step, bottom - 3, x + 13 + step, bottom + 2), fill=LEATHER_D)
        d.rectangle((x - 2 + step, bottom - 3, x + 13 + step, bottom - 2), fill=LEATHER)
        d.rectangle((x + 8 + step, bottom - 6, x + 11 + step, bottom - 4), fill=SKIN_D)
        d.rectangle((x - 2 + step, bottom + 1, x - 1 + step, bottom + 2), fill=BONE)
    d.polygon([(22, 46 + bob), (58, 46 + bob), (55, 63 - kneel // 2), (40, 57 - kneel // 2), (25, 63 - kneel // 2)], fill=LEATHER)
    d.polygon([(40, 48 + bob), (58, 46 + bob), (55, 63 - kneel // 2), (40, 57 - kneel // 2)], fill=LEATHER_D)
    d.rectangle((22, 44 + bob, 58, 48 + bob), fill=LEATHER_D)
    d.line((22, 44 + bob, 58, 44 + bob), fill=LEATHER)
    for x in (27, 35, 45, 53):
        d.rectangle((x, 45 + bob, x + 1, 47 + bob), fill=BONE)
        d.point((x, 47 + bob), fill=BONE_D)
    for x, drop in ((30, 8), (38, 12), (49, 9)):
        d.line((x, 49 + bob, x + 1, 49 + bob + drop), fill=TAIL)
    d = db
    d = db
    d.polygon([(16, 26 + bob), (64, 26 + bob), (59, 46 + bob), (21, 46 + bob)], fill=SKIN)
    d.polygon([(40, 27 + bob), (64, 26 + bob), (59, 46 + bob), (40, 46 + bob)], fill=SKIN_D)
    ellipse(d, 40, 40 + bob, 12, 8, BELLY)
    ellipse(d, 44, 42 + bob, 6, 5, SKIN_L)
    d.line((40, 33 + bob, 40, 46 + bob), fill=SKIN_L)
    d.line((29, 35 + bob, 51, 35 + bob), fill=SKIN_D)
    d.line((31, 38 + bob, 49, 38 + bob), fill=SKIN_D)
    for x, y in ((24, 36), (55, 33), (34, 29), (47, 30), (28, 42)):
        d.point((x, y + bob), fill=SKIN_L if x < 40 else SKIN)
    for x, y, w in ((42, 41, 3), (33, 43, 4), (50, 37, 3)):
        d.rectangle((x, y + bob, x + w, y + 1 + bob), fill=SKIN_D)
    chain = [(22 + i * 2, 29 + bob + int(i * 0.9)) for i in range(18)]
    for index, (x, y) in enumerate(chain):
        d.rectangle((x, y, x + 1, y + 1), fill=IRON if index % 2 else IRON_D)
    limb(d, (17, 29 + bob), (14, 52 + bob + larm), 10, SKIN)
    d.line((12, 32 + bob, 10, 50 + bob + larm), fill=SKIN_L, width=2)
    d.rectangle((8, 47 + bob + larm, 20, 50 + bob + larm), fill=IRON)
    d.line((8, 47 + bob + larm, 20, 47 + bob + larm), fill=IRON_D)
    for x in (10, 14, 18):
        d.point((x, 46 + bob + larm), fill=IRON_D)
    ellipse(d, 14, 57 + bob + larm, 7, 6, SKIN)
    d.rectangle((9, 60 + bob + larm, 19, 62 + bob + larm), fill=SKIN_D)
    for x in (9, 12, 15, 18):
        d.point((x, 63 + bob + larm), fill=BONE)
    if rat:
        hx, hy = 14, 52 + bob + larm
        d.rectangle((hx - 6, hy - 4, hx + 4, hy), fill=(132, 126, 135, 255))
        d.rectangle((hx - 6, hy - 1, hx + 4, hy), fill=(104, 100, 106, 255))
        d.rectangle((hx + 3, hy - 3, hx + 6, hy - 1), fill=(132, 126, 135, 255))
        d.point((hx + 5, hy - 3), fill=(209, 58, 17, 255))
        d.line((hx - 6, hy - 2, hx - 11, hy - 5), fill=(238, 195, 154, 255))
    ellipse(d, 17, 29 + bob, 9, 8, SKIN)
    ellipse(d, 14, 27 + bob, 5, 5, SKIN_L)
    d.polygon([(10, 23 + bob), (14, 15 + bob), (18, 22 + bob)], fill=IRON)
    d.polygon([(13, 23 + bob), (14, 18 + bob), (16, 22 + bob)], fill=IRON_D)
    ellipse(d, 63, 29 + bob, 9, 8, SKIN_D)
    d.polygon([(58, 23 + bob), (63, 14 + bob), (68, 23 + bob)], fill=IRON_D)
    limb(d, (62, 31 + bob), (grip[0], grip[1] + bob + rarm), 10, SKIN_D)
    d.rectangle((grip[0] - 8, grip[1] - 3 + bob + rarm, grip[0] + 8, grip[1] + bob + rarm), fill=IRON)
    ellipse(d, grip[0], grip[1] + 2 + bob + rarm, 7, 6, SKIN_D)
    for x in (grip[0] - 4, grip[0] - 1, grip[0] + 2, grip[0] + 5):
        d.point((x, grip[1] + 7 + bob + rarm), fill=BONE)
    d = dh
    d = dh
    ellipse(d, 40, 18 + bob, 12, 10, SKIN)
    ellipse(d, 36, 16 + bob, 8, 7, SKIN_L)
    d.polygon([(27, 22 + bob), (53, 22 + bob), (51, 33 + bob + jaw), (29, 33 + bob + jaw)], fill=SKIN)
    d.rectangle((31, 26 + bob, 49, 31 + bob + jaw), fill=MOUTH)
    for x in (32, 37, 42, 47):
        d.rectangle((x, 26 + bob, x + 1, 27 + bob), fill=BONE)
    d.polygon([(29, 32 + bob + jaw), (34, 32 + bob + jaw), (31, 21 + bob)], fill=BONE)
    d.polygon([(46, 32 + bob + jaw), (51, 32 + bob + jaw), (48, 21 + bob)], fill=BONE)
    d.line((31, 30 + bob + jaw, 31, 23 + bob), fill=BONE_D)
    d.line((48, 30 + bob + jaw, 48, 23 + bob), fill=BONE_D)
    for cx, sx in ((27, -1), (53, 1)):
        d.polygon([(cx, 14 + bob), (cx + sx * 6, 9 + bob), (cx, 21 + bob)], fill=SKIN_D)
        d.point((cx + sx * 2, 14 + bob), fill=SCAR)
    for x, flip in ((31, 1), (43, -1)):
        if blink:
            d.line((x, 17 + bob, x + 4, 17 + bob), fill=SKIN_D)
        else:
            d.rectangle((x, 16 + bob, x + 4, 18 + bob), fill=EYE)
            d.point((x + (3 if flip > 0 else 1), 17 + bob), fill=OUTLINE)
        d.line((x - 1, 14 + bob + (0 if flip > 0 else 3), x + 5, 14 + bob + (3 if flip > 0 else 0)), fill=SKIN_D)
    d.line((35, 8 + bob, 38, 14 + bob), fill=SCAR)
    d.line((36, 8 + bob, 39, 14 + bob), fill=SCAR)
    for x, y in ((46, 11), (33, 12), (44, 21)):
        d.point((x, y + bob), fill=SKIN_L)
    d.rectangle((38, 20 + bob, 42, 22 + bob), fill=SKIN_D)
    image = Image.new("RGBA", (W, H), CLEAR)
    image.alpha_composite(legs)
    image.alpha_composite(body, (sway, real_bob)) if False else None
    for src, dx, dy in ((body, sway, real_bob), (head, sway + hdx, real_bob + hdy)):
        shifted = Image.new("RGBA", (W, H), CLEAR)
        shifted.paste(src, (dx, dy))
        image.alpha_composite(shifted)
    image = Image.new("RGBA", (W, H), CLEAR)
    image.alpha_composite(legs)
    for src, dx, dy in ((body, sway, real_bob), (head, sway + hdx, real_bob + hdy)):
        shifted = Image.new("RGBA", (W, H), CLEAR)
        shifted.paste(src, (dx, dy))
        image.alpha_composite(shifted)
    return outline(image)


def walk():
    frames = []
    count = 8
    for index in range(count):
        phase = index / count * math.tau
        swing = math.sin(phase)
        frames.append(
            ogre(
                bob=round(abs(math.cos(phase)) * 2.0) - 1,
                sway=round(swing * 2.5),
                hdx=round(-swing * 1.5),
                hdy=round(abs(swing)) if abs(swing) > 0.6 else 0,
                lleg=round(max(0.0, swing) * 5),
                rleg=round(max(0.0, -swing) * 5),
                lstep=round(swing * 4),
                rstep=round(-swing * 4),
                larm=round(-swing * 5),
                rarm=round(swing * 3),
                grip=(57, 47),
                tip=(63 + round(swing * 2), 5),
                jaw=1 if abs(swing) > 0.8 else 0,
            )
        )
    return frames


def stomp():
    poses = [
        dict(bob=0, jaw=1),
        dict(bob=-2, sway=-2, hdx=-1, rleg=5, rstep=-2, larm=-4, rarm=-3, jaw=1),
        dict(bob=-5, sway=-3, hdx=-2, hdy=-1, rleg=12, rstep=-4, larm=-9, rarm=-6, jaw=2),
        dict(bob=-6, sway=-3, hdx=-2, hdy=-1, rleg=14, rstep=-4, larm=-10, rarm=-7, jaw=2),
        dict(bob=5, sway=2, hdx=1, hdy=3, larm=3, rarm=3, jaw=5),
        dict(bob=3, sway=1, hdy=2, larm=1, jaw=3),
        dict(bob=1, jaw=1),
        dict(bob=0),
    ]
    return [ogre(**pose) for pose in poses]


def throw():
    poses = [
        dict(bob=0),
        dict(bob=1, sway=-1, larm=-10, rat=True, jaw=1),
        dict(bob=2, sway=-3, hdx=-1, larm=-24, rat=True, jaw=2, rarm=-2),
        dict(bob=2, sway=-4, hdx=-2, larm=-30, rat=True, jaw=3, rarm=-3),
        dict(bob=-1, sway=3, hdx=2, larm=-10, jaw=4, rarm=2),
        dict(bob=1, sway=2, hdx=1, larm=0, jaw=2),
        dict(bob=0, sway=1),
        dict(bob=0),
    ]
    return [ogre(**pose) for pose in poses]


def slam():
    poses = [
        dict(grip=(57, 40), tip=(64, 4), bob=1, jaw=0),
        dict(grip=(58, 34), tip=(72, 6), bob=0, sway=-1, jaw=1, larm=-3),
        dict(grip=(58, 28), tip=(76, 14), bob=-3, sway=-3, hdx=-2, jaw=2, larm=-6, rarm=-2),
        dict(grip=(59, 36), tip=(74, 36), bob=1, sway=1, jaw=3, larm=1),
        dict(grip=(60, 54), tip=(66, 78), bob=4, sway=3, hdy=3, jaw=5, larm=3, rarm=2),
        dict(grip=(60, 56), tip=(68, 79), bob=3, sway=2, hdy=2, jaw=3, larm=2),
        dict(grip=(59, 50), tip=(65, 30), bob=1, jaw=1),
        dict(grip=(57, 44), tip=(64, 10), bob=0),
    ]
    return [ogre(**pose) for pose in poses]


MEAT = (150, 34, 46, 255)
MEAT_D = (104, 22, 34, 255)
MEAT_L = (196, 72, 78, 255)
BLOOD = (128, 18, 30, 255)
BLOOD_D = (84, 12, 24, 255)


def gore(base, cells, spread, fall, keep, seed, pool, drops, cracks_only=False):
    rng = random.Random(seed)
    box = base.getbbox() or (0, 0, W, H)
    seeds = [(rng.uniform(box[0], box[2]), rng.uniform(box[1], box[3])) for _ in range(cells)]
    kept = [rng.random() < keep or index < 4 for index in range(cells)]
    tones = [rng.choice((MEAT, MEAT, MEAT_D, MEAT_L)) for _ in range(cells)]
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
            colour = SKIN_D
        if cracks_only:
            if any(cell_of.get((x + ox, y + oy), index) != index for ox in (-1, 0, 1) for oy in (-1, 0, 1)):
                colour = MEAT
        elif not near_edge(x, y) or colour in (BONE, BONE_D):
            colour = BONE if colour in (BONE, BONE_D) else tones[index]
        dx, dy = shifts[index]
        nx, ny = x + dx, min(y + dy, H - 4)
        if 1 <= nx < W - 1 and 1 <= ny < H - 1:
            chunks.putpixel((nx, ny), colour)
    chunks = outline(chunks)
    for x in range(W):
        for y in range(H):
            if chunks.getpixel((x, y)) == OUTLINE and any(
                chunks.getpixel((min(W - 1, max(0, x + ox)), min(H - 1, max(0, y + oy))))[:3] in (MEAT[:3], MEAT_D[:3], MEAT_L[:3])
                for ox in (-1, 0, 1)
                for oy in (-1, 0, 1)
            ):
                chunks.putpixel((x, y), (60, 10, 20, 255))
    out = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(out)
    if pool:
        d.ellipse((40 - pool, H - 4 - pool // 4 - 1, 40 + pool, H - 3), fill=BLOOD)
        d.ellipse((40 - pool + 3, H - 4 - pool // 4 + 1, 40 + pool - 5, H - 4), fill=BLOOD_D)
        d.line((40 - pool // 2, H - 5 - pool // 5, 40 - pool // 4, H - 5 - pool // 5), fill=MEAT_L)
    out.alpha_composite(chunks)
    d = ImageDraw.Draw(out)
    for _ in range(drops):
        angle = rng.uniform(math.pi, math.tau) if rng.random() < 0.7 else rng.uniform(0, math.pi)
        reach = rng.uniform(4, spread * 1.4 + 6)
        x, y = round(centre[0] + math.cos(angle) * reach), round(centre[1] - 8 + math.sin(angle) * reach * 0.8 + fall * 0.4)
        if 1 <= x < W - 1 and 1 <= y < H - 3:
            size = rng.choice((1, 1, 2))
            d.rectangle((x, y, x + size - 1, y + size - 1), fill=rng.choice((BLOOD, BLOOD, MEAT)))
    return out


def death():
    fallen = ogre(bob=15, sway=2, hdx=2, hdy=9, kneel=12, jaw=1, blink=True, larm=6, grip=(54, 60), tip=(72, 56))
    frames = [
        ogre(bob=2, sway=-4, hdx=-2, hdy=2, jaw=3, blink=True, larm=-4, rarm=-3),
        ogre(bob=6, sway=3, hdx=2, hdy=4, kneel=5, jaw=2, blink=True, larm=-2, rarm=3),
        ogre(bob=11, sway=-1, hdy=6, kneel=9, jaw=1, blink=True, larm=4, grip=(56, 56), tip=(70, 40)),
        fallen,
    ]
    for cells, spread, fall, keep, pool, drops in ((9, 5, 0, 1.0, 0, 8), (16, 15, 5, 0.9, 0, 24), (22, 23, 14, 0.7, 0, 34), (28, 30, 26, 0.5, 0, 30)):
        frames.append(gore(fallen, cells, spread, fall, keep, 7 + cells, pool, drops, cracks_only=spread == 5))
    return frames


def jump():
    poses = [
        dict(bob=2, kneel=3, jaw=1, larm=2, rarm=2),
        dict(bob=5, kneel=6, sway=0, jaw=2, larm=4, rarm=4),
        dict(bob=-3, lleg=7, rleg=7, kneel=0, jaw=3, larm=-8, rarm=-5, hdy=1),
        dict(bob=-7, lleg=10, rleg=10, jaw=4, larm=-12, rarm=-8, hdy=1),
        dict(bob=-3, lleg=7, rleg=7, jaw=4, larm=-9, rarm=-6),
        dict(bob=5, kneel=5, jaw=6, larm=4, rarm=3, hdy=3),
        dict(bob=3, jaw=3, larm=1, rarm=1, hdy=1),
        dict(bob=0, jaw=1),
    ]
    return [ogre(**pose) for pose in poses]


def roar():
    poses = [
        dict(bob=0, jaw=1),
        dict(bob=-2, hdy=-2, larm=-5, rarm=-3, jaw=0),
        dict(bob=-3, sway=0, hdy=-4, larm=-12, rarm=-6, grip=(59, 40), tip=(67, 2), jaw=7),
        dict(bob=-3, sway=1, hdx=1, hdy=-4, larm=-13, rarm=-6, grip=(59, 40), tip=(67, 2), jaw=8),
        dict(bob=-3, sway=-1, hdx=-1, hdy=-3, larm=-12, rarm=-6, grip=(59, 40), tip=(67, 2), jaw=7),
        dict(bob=-2, sway=1, hdy=-2, larm=-8, rarm=-4, jaw=5),
        dict(bob=0, jaw=2),
        dict(bob=0, jaw=1),
    ]
    return [ogre(**pose) for pose in poses]


def main():
    rows = [walk(), stomp(), throw(), slam(), roar(), death(), jump()]
    sheet = Image.new("RGBA", (W * COLUMNS, H * len(rows)), CLEAR)
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * W, row * H))
    sheet.save(OUT / "ogre.tga")


main()
