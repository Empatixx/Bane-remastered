import math
import random
import sys
from pathlib import Path

from PIL import Image, ImageDraw

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"

CLEAR = (0, 0, 0, 0)
OUTLINE = (26, 12, 20, 255)
WOOD_1 = (74, 44, 46, 255)
WOOD_2 = (104, 62, 56, 255)
WOOD_3 = (136, 88, 68, 255)
WOOD_4 = (176, 116, 82, 255)
WOOD_5 = (214, 150, 100, 255)
METAL_D = (46, 44, 66, 255)
METAL = (88, 92, 122, 255)
METAL_L = (140, 150, 186, 255)
METAL_H = (206, 216, 240, 255)
GOLD_D = (150, 104, 54, 255)
GOLD = (222, 170, 80, 255)
GOLD_L = (255, 226, 140, 255)
MAW_D = (24, 8, 12, 255)
MAW = (62, 20, 24, 255)
MAW_L = (112, 36, 38, 255)
TEETH_D = (150, 132, 120, 255)
TEETH = (226, 212, 188, 255)
TEETH_L = (255, 246, 226, 255)
TONGUE_D = (120, 26, 70, 255)
TONGUE = (190, 52, 100, 255)
TONGUE_L = (240, 110, 150, 255)
RUNE = (150, 230, 255, 255)
GLOW = (210, 34, 34, 255)
GLOW_D = (110, 14, 20, 255)
EYE = (255, 150, 90, 255)
EYE_A = (240, 70, 50, 255)
PUPIL = (24, 10, 30, 255)
TEAR = (176, 222, 255, 255)
TEAR_D = (70, 130, 200, 255)
BLOB = (122, 190, 92, 255)
BLOB_D = (66, 120, 60, 255)
BLOB_L = (190, 236, 150, 255)
SINEW = (176, 56, 66, 255)
SINEW_D = (110, 30, 42, 255)
SINEW_L = (232, 120, 120, 255)
SALIVA = (236, 226, 238, 255)
SALIVA_D = (170, 150, 176, 255)
W, H = 156, 148


def box(d, x0, y0, x1, y1, fill):
    d.rectangle((x0, y0, x1, y1), fill=fill)


def fang(d, x, y, length, width, up, lean=0.0):
    sign = -1 if up else 1
    tip = (x + width * 0.5 + lean, y + sign * length)
    d.polygon([(x, y), (x + width, y), (x + width - 1, y + sign * length * 0.4), tip, (x + 1, y + sign * length * 0.5)], fill=TEETH)
    d.line((x + 1, y + sign, x + 1 + lean * 0.3, y + sign * length * 0.55), fill=TEETH_L)
    d.line((x + width - 1, y + sign, x + width - 2 + lean * 0.4, y + sign * length * 0.4), fill=TEETH_D)
    d.point(tip, fill=TEETH_L)


def curved_fang(d, x, y, height, bend, width=6):
    def quad(p0, p1, p2, t):
        u = 1 - t
        return (u * u * p0[0] + 2 * u * t * p1[0] + t * t * p2[0], u * u * p0[1] + 2 * u * t * p1[1] + t * t * p2[1])

    tip = (x + width * 0.5 + bend, y - height)
    left = [quad((x, y), (x + 0.5, y - height * 0.6), tip, i / 10) for i in range(11)]
    right = [quad((x + width, y), (x + width + 0.5, y - height * 0.3), tip, i / 10) for i in range(11)]
    d.polygon(left + right[::-1], fill=TEETH)
    d.line(left, fill=TEETH_L, width=1)
    d.line([(px - 1, py) for px, py in right[:8]], fill=TEETH_D, width=1)
    d.point(tip, fill=TEETH_L)


def mimic(open_amount=1.0, lick=None, hop=0, squash=0, eye=1.0, dx=0, clamp=False, blob=None, tear=None, cracks=False):
    image = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(image)
    teeth = 1.0 if clamp else min(1.0, max(0.0, (open_amount - 0.12) / 0.5))
    top = 108 + hop + squash
    bottom = 134 + hop
    left, right = 16 + dx, 70 + dx
    d.polygon([(left - 1, top - 1), (right + 1, top - 1), (right + 3, bottom + 1), (left - 3, bottom + 1)], fill=OUTLINE)
    height = bottom - top
    for y in range(top, bottom + 1):
        t = (y - top) / max(1, height)
        colour = WOOD_5 if t < 0.07 else WOOD_4 if t < 0.28 else WOOD_3 if t < 0.55 else WOOD_2 if t < 0.82 else WOOD_1
        inset = int(t * 2)
        box(d, left - 1 - inset, y, right + 1 + inset, y, colour)
    for py in (top + 9, top + 17):
        d.line((left - 1, py, right, py), fill=WOOD_1)
        d.line((left - 1, py + 1, right, py + 1), fill=WOOD_4)
    for bx in (left + 4, right - 11):
        box(d, bx, top, bx + 6, bottom, METAL)
        box(d, bx, top, bx + 1, bottom, METAL_L)
        d.line((bx + 5, top, bx + 5, bottom), fill=METAL_D)
        d.point((bx + 3, top + 4), fill=METAL_H)
        d.point((bx + 3, bottom - 4), fill=METAL_H)
    box(d, left + 22, top + 8, left + 33, bottom - 3, GOLD_D)
    box(d, left + 22, top + 8, left + 33, top + 9, GOLD_L)
    d.ellipse((left + 25, top + 11, left + 31, top + 17), fill=GLOW_D)
    d.ellipse((left + 26, top + 12, left + 30, top + 16), fill=GLOW)
    box(d, left, top - 2, right, top + 2, MAW_D)
    box(d, left + 1, top - 2, right - 1, top - 1, MAW)
    box(d, left + 14, top - 6, right - 2, top - 2, MAW)
    box(d, left + 20, top - 7, right - 6, top - 4, MAW_L)
    if teeth > 0.1:
        for tx in range(right - 28, right - 1, 4):
            d.polygon([(tx, top - 2), (tx + 3, top - 2), (tx + 1.5, top - 2 - 3 * teeth)], fill=TEETH_D)
        for tx, ln, ls in ((right - 26, 11, 1), (right - 6, 13, -1)):
            fang(d, tx, top - 2, ln * teeth, 6, True, ls)
    hinge = (left + 1, top - 3)
    open_amount = min(open_amount, 1.38)
    angle = -math.radians(65 * open_amount)
    c, s = math.cos(angle), math.sin(angle)

    def at(u, v):
        return (hinge[0] + u * c - v * s, hinge[1] + u * s + v * c)

    length, bulge = 52, 20
    arc = []
    for step in range(25):
        t = math.pi * step / 24
        u = length / 2 - (length / 2) * math.cos(t)
        v = -bulge * math.sin(t)
        arc.append(at(u, v))
    shell = arc + [at(length, 4), at(0, 4)]
    d.polygon(shell, fill=OUTLINE)
    inner_arc = []
    for step in range(25):
        t = math.pi * step / 24
        u = length / 2 - (length / 2 - 2) * math.cos(t)
        v = -(bulge - 3) * math.sin(t)
        inner_arc.append(at(u, v))
    d.polygon(inner_arc + [at(length - 2, 3), at(2, 3)], fill=WOOD_3)
    band = []
    for step in range(25):
        t = math.pi * step / 24
        u = length / 2 - (length / 2 - 2) * math.cos(t)
        v = -(bulge - 3) * math.sin(t) * 0.55
        band.append(at(u, v))
    d.polygon([at(0, 3)] + inner_arc[:13][::-1][::-1] + [at(length / 2, 3)], fill=WOOD_3)
    d.line([at(1, 1), at(length - 1, 1)], fill=WOOD_2, width=2)
    for u in (12, 40):
        a = at(u, -bulge * 0.9 * math.sin(math.pi * u / length))
        b = at(u, 2)
        d.line((a[0], a[1], b[0], b[1]), fill=METAL, width=4)
        d.line((a[0] - 1, a[1], b[0] - 1, b[1]), fill=METAL_L, width=1)
    d.line([at(2, -bulge * 0.8 * math.sin(math.pi * 2 / length)) for _ in range(1)] * 2, fill=WOOD_5)
    top_curve = [at(length / 2 - (length / 2 - 3) * math.cos(math.pi * k / 24), -(bulge - 4) * math.sin(math.pi * k / 24)) for k in range(25)]
    d.line(top_curve, fill=WOOD_5, width=1)
    inner_face = [at(3, 2), at(length - 3, 2), at(length - 3, 6), at(3, 6)]
    d.polygon(inner_face, fill=MAW_D)
    if teeth > 0.1:
        for u in (10, 18, 30, 38, 46):
            d.polygon([at(u - 1.5, 5), at(u + 2.5, 5), at(u + 0.5, 5 + 4 * teeth)], fill=TEETH_D)
        for u, ln in ((12, 11), (30, 14), (44, 12)):
            length_ = ln * teeth
            pts = [at(u - 3, 5), at(u + 3, 5), at(u + 2, 5 + length_ * 0.4), at(u + 0.3, 5 + length_), at(u - 2, 5 + length_ * 0.5)]
            d.polygon(pts, fill=TEETH)
            a_, b_ = at(u - 2, 5.5), at(u - 1, 5 + length_ * 0.55)
            d.line((a_[0], a_[1], b_[0], b_[1]), fill=TEETH_L)
            a_, b_ = at(u + 2, 5.5), at(u + 1.5, 5 + length_ * 0.4)
            d.line((a_[0], a_[1], b_[0], b_[1]), fill=TEETH_D)
            tip = at(u + 0.3, 5 + length_)
            d.point(tip, fill=TEETH_L)
    if open_amount > 0.25:
        spread = min(1.0, open_amount)
        for u, bx in ((4, left + 4), (9, left + 9)):
            top_pt = at(u, 3)
            bottom_pt = (bx, top - 2)
            d.polygon([top_pt, (top_pt[0] + 3, top_pt[1]), (bottom_pt[0] + 3, bottom_pt[1]), bottom_pt], fill=SINEW_D)
            d.line([top_pt, bottom_pt], fill=SINEW, width=2)
            mid = ((top_pt[0] + bottom_pt[0]) / 2, (top_pt[1] + bottom_pt[1]) / 2)
            d.point((mid[0] - 1, mid[1]), fill=SINEW_L)
        for u, bx, sag in ((20, left + 24, 5), (28, left + 31, 7), (36, left + 40, 6)):
            start = at(u, 5)
            end = (bx, top - 3)
            points = []
            for i in range(9):
                t = i / 8
                x = start[0] + (end[0] - start[0]) * t + math.sin(t * math.pi) * 1.0
                y = start[1] + (end[1] - start[1]) * t + math.sin(t * math.pi) * sag * (1.1 - spread)
                points.append((x, y))
            d.line(points, fill=SALIVA_D, width=1)
            d.line([(px - 1, py) for px, py in points[1:-1]], fill=SALIVA, width=1)
            for i in (3, 5):
                d.point((points[i][0], points[i][1] + 1), fill=SALIVA)
    ex, ey = at(40, -7)
    cx, cy = int(round(ex)), int(round(ey))
    for dy in range(-7, 8):
        for dx in range(-7, 8):
            r = math.hypot(dx, dy)
            if r > 6.4:
                continue
            if r > 5.4:
                colour = GLOW_D
            elif r > 4.4:
                colour = GLOW
            elif r > 3.2:
                colour = EYE_A
            else:
                colour = EYE
            if abs(dx) <= 0 and abs(dy) <= 4:
                colour = PUPIL
            elif dx == 1 and abs(dy) <= 2:
                colour = (140, 20, 20, 255)
            image.putpixel((cx + dx, cy + dy), colour)
    lid_y = -7 + (1.0 - eye) * 14.0
    for dy in range(-7, 8):
        for dx in range(-7, 8):
            r = math.hypot(dx, dy)
            if r > 6.4:
                continue
            curve = max(lid_y + (dx * dx) / 14.0, -3.5 + 0.55 * dx)
            if dy < curve:
                image.putpixel((cx + dx, cy + dy), WOOD_3 if dy < curve - 1.5 else WOOD_2)
            elif dy < curve + 1:
                image.putpixel((cx + dx, cy + dy), OUTLINE)
    if cracks:
        for pts in (((24, top - 18), (28, top - 12), (25, top - 6), (30, top)), ((50, top - 20), (46, top - 13), (51, top - 7)), ((20, top + 4), (26, top + 10), (23, top + 17), (29, top + 24)), ((56, top + 2), (52, top + 9), (58, top + 15))):
            d.line(pts, fill=OUTLINE, width=2)
            d.line([(x + 1, y) for x, y in pts], fill=COIN, width=1)
    if tear is not None:
        tx0, ty0 = cx + 3, cy + 6
        d.line((tx0, ty0 - 4, tx0, ty0 + tear), fill=TEAR_D)
        d.ellipse((tx0 - 2, ty0 + tear - 1, tx0 + 2, ty0 + tear + 4), fill=TEAR_D)
        d.ellipse((tx0 - 1, ty0 + tear, tx0 + 1, ty0 + tear + 3), fill=TEAR)
        d.point((tx0 - 1, ty0 + tear), fill=(255, 255, 255, 255))
    if lick is not None:
        tip = (right + 8 + 7 * math.sin(lick), top - 24 + 30 * (0.5 - 0.5 * math.cos(lick)) - 10 * math.sin(lick) ** 2)
        p0, p3 = (right - 22, top - 4), tip
        p1, p2 = (right - 4, top - 14), (right + 10 + 8 * math.sin(lick), (p0[1] + p3[1]) / 2 + 4)
        path = []
        for index in range(14):
            t = index / 13
            u_ = 1 - t
            x = u_**3 * p0[0] + 3 * u_**2 * t * p1[0] + 3 * u_ * t**2 * p2[0] + t**3 * p3[0]
            y = u_**3 * p0[1] + 3 * u_**2 * t * p1[1] + 3 * u_ * t**2 * p2[1] + t**3 * p3[1]
            path.append((x, y, 5.2 - t * 2.0))
        for x, y, r in path:
            d.ellipse((x - r - 1, y - r - 1, x + r + 1, y + r + 1), fill=OUTLINE)
        for x, y, r in path:
            d.ellipse((x - r, y - r, x + r, y + r), fill=TONGUE_D)
        for x, y, r in path:
            d.ellipse((x - r + 0.5, y - r + 0.5, x + r - 0.5, y + r - 1.5), fill=TONGUE)
        for x, y, r in path:
            d.ellipse((x - r + 1.5, y - r + 0.8, x + r - 2.5, y - 0.5), fill=TONGUE_L)
        for x, y, r in path[2:-1:2]:
            d.line((x - 1, y - r + 2, x - 1, y + r - 2), fill=TONGUE_D)
        for index in (3, 6, 9, 11):
            x, y, r = path[index]
            d.point((x + 1, y - r + 1), fill=SALIVA)
            d.point((x, y - r + 1), fill=TONGUE_L)
        tx, ty, tr = path[-1]
        d.line((tx, ty + tr, tx, ty + tr + 5), fill=SALIVA_D)
        d.line((tx - 1, ty + tr + 1, tx - 1, ty + tr + 4), fill=SALIVA)
        d.ellipse((tx - 2, ty + tr + 4, tx + 1, ty + tr + 8), fill=SALIVA)
        d.point((tx - 1, ty + tr + 5), fill=(255, 255, 255, 255))
        for index, length in ((5, 3), (8, 4)):
            x, y, r = path[index]
            d.line((x + 1, y + r, x + 1, y + r + length), fill=SALIVA_D)
            d.ellipse((x, y + r + length - 1, x + 2, y + r + length + 2), fill=SALIVA)
    if blob is not None:
        rng_ = random.Random(blob + 3)
        for k in range(5):
            u = 14 + k * 6
            a_ = at(u, 5)
            b_ = (right - 40 + k * 8, top - 3)
            pts = []
            for q in range(10):
                t = q / 9
                pts.append((a_[0] + (b_[0] - a_[0]) * t, a_[1] + (b_[1] - a_[1]) * t + math.sin(t * math.pi) * (3 + k) * (1.2 - min(1.0, blob / 44))))
            d.line(pts, fill=SALIVA_D, width=2)
            d.line([(x - 1, y) for x, y in pts[1:-1]], fill=SALIVA, width=1)
        for k in range(9):
            fx = right - 6 + blob * 0.9 + rng_.uniform(-6, 18) * (0.4 + blob / 44)
            fy = top - 12 + rng_.uniform(-10, 10) + (blob / 44) ** 2 * 18
            r_ = rng_.choice((1.5, 2, 2.5))
            d.ellipse((fx - r_, fy - r_, fx + r_, fy + r_ + 1), fill=SALIVA_D)
            d.ellipse((fx - r_ + 0.5, fy - r_ + 0.5, fx + r_ - 0.5, fy + r_), fill=SALIVA)
        if blob >= 30:
            sx = right + 30 + blob * 0.5
            d.ellipse((sx - 10, top + 20, sx + 10, top + 26), fill=SALIVA_D)
            d.ellipse((sx - 8, top + 20, sx + 8, top + 24), fill=SALIVA)
    box(d, left, top - 1, right, top + 2, MAW_D)
    if teeth > 0.1:
        for tx, ln, bend in ((right - 24, 13, 9), (right - 14, 17, 10), (right - 3, 12, 8)):
            curved_fang(d, tx, top - 1, ln * teeth, bend * teeth)
    return image


def idle():
    angles = [0.0, 0.7, 1.5, 2.4, 3.14, 2.4, 1.2, 0.3]
    eyes = [1.0, 1.0, 1.0, 0.5, 0.0, 0.5, 1.0, 1.0]
    opens = [1.0, 0.92, 0.8, 0.62, 0.5, 0.62, 0.8, 0.95]
    return [mimic(lick=a, eye=e, open_amount=o) for a, e, o in zip(angles, eyes, opens)]


def hop():
    poses = [
        dict(squash=4, open_amount=0.6, lick=0.3, eye=0.6),
        dict(squash=9, open_amount=0.35, lick=None, eye=0.3),
        dict(hop=-16, open_amount=1.35, lick=2.4, squash=-2),
        dict(hop=-34, open_amount=1.7, lick=2.9, squash=-3, eye=1.0),
        dict(hop=-30, open_amount=1.55, lick=2.4, squash=-2),
        dict(hop=-12, open_amount=1.0, lick=1.2),
        dict(squash=10, open_amount=0.3, lick=None, eye=0.4),
        dict(squash=3, open_amount=0.8, lick=0.4),
    ]
    return [mimic(**pose) for pose in poses]


def bite():
    poses = [
        dict(open_amount=1.0, lick=0.5),
        dict(open_amount=1.4, lick=None, dx=-5, squash=2, eye=0.8),
        dict(open_amount=1.65, lick=None, dx=-8, squash=3, eye=1.0),
        dict(open_amount=1.45, lick=None, dx=6, squash=-1, hop=-3),
        dict(open_amount=0.12, lick=None, dx=11, clamp=True, squash=1),
        dict(open_amount=0.1, lick=None, dx=10, clamp=True, squash=2, eye=0.7),
        dict(open_amount=0.7, lick=0.6, dx=4),
        dict(open_amount=1.0, lick=0.3),
    ]
    return [mimic(**pose) for pose in poses]


def spit():
    poses = [
        dict(open_amount=1.0, lick=0.3),
        dict(open_amount=1.35, lick=None, dx=-4, squash=3),
        dict(open_amount=1.7, lick=None, dx=-7, squash=4, eye=0.9),
        dict(open_amount=1.7, lick=None, dx=-6, squash=3, blob=0),
        dict(open_amount=1.45, lick=None, dx=3, squash=0, blob=22),
        dict(open_amount=1.2, lick=None, dx=5, squash=-1, blob=44),
        dict(open_amount=1.0, lick=0.8, dx=1),
        dict(open_amount=1.0, lick=0.3),
    ]
    return [mimic(**pose) for pose in poses]


def rest():
    poses = [
        dict(open_amount=0.8, lick=0.4),
        dict(open_amount=0.5, lick=None),
        dict(open_amount=0.2, lick=None, eye=0.5),
        dict(open_amount=0.0, lick=None, eye=0.0),
        dict(open_amount=0.0, lick=None, eye=0.0),
        dict(open_amount=0.0, lick=None, eye=0.2),
        dict(open_amount=0.0, lick=None, eye=0.0),
        dict(open_amount=0.0, lick=None, eye=0.0),
    ]
    return [mimic(**pose) for pose in poses]


COIN = (255, 226, 140, 255)
COIN_D = (222, 170, 80, 255)
BITS = (WOOD_3, WOOD_2, WOOD_4, MAW_L, COIN_D)


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


def burst(base, cells, spread, fall, keep, seed, cracks_only=False, coins=12):
    rng = random.Random(seed)
    box_ = base.getbbox() or (0, 0, W, H)
    seeds = [(rng.uniform(box_[0], box_[2]), rng.uniform(box_[1], box_[3])) for _ in range(cells)]
    kept = [rng.random() < keep or index < 4 for index in range(cells)]
    tones = [rng.choice(BITS) for _ in range(cells)]
    centre = ((box_[0] + box_[2]) / 2, (box_[1] + box_[3]) / 2 + 6)
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
        if cracks_only:
            if any(cell_of.get((x + ox, y + oy), index) != index for ox in (-1, 0, 1) for oy in (-1, 0, 1)):
                colour = COIN
        elif not near_edge(x, y) and colour not in (TEETH, TEETH_L, TEETH_D):
            colour = tones[index]
        dx, dy = shifts[index]
        nx, ny = x + dx, min(y + dy, H - 3)
        if 1 <= nx < W - 1 and 1 <= ny < H - 1:
            chunks.putpixel((nx, ny), colour)
    d = ImageDraw.Draw(chunks)
    for _ in range(coins):
        angle = rng.uniform(math.pi, math.tau) if rng.random() < 0.7 else rng.uniform(0, math.pi)
        reach = rng.uniform(4, spread * 1.4 + 6)
        x, y = round(centre[0] + math.cos(angle) * reach), round(centre[1] - 10 + math.sin(angle) * reach * 0.8 + fall * 0.4)
        if 2 <= x < W - 3 and 2 <= y < H - 4:
            d.ellipse((x, y, x + 2, y + 2), fill=COIN)
            d.point((x, y + 2), fill=COIN_D)
    return chunks


def coin_pile(count, seed, width=34, height=14, bits=3):
    rng = random.Random(seed)
    image = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(image)
    cx, floor = 52, H - 8
    for index in range(count):
        t = rng.random()
        x = cx + (rng.random() - 0.5) * 2 * width * (1 - t * 0.7)
        y = floor - t * height - rng.random() * 3
        d.ellipse((x - 3, y - 2, x + 3, y + 2), fill=OUTLINE)
    for index in range(count):
        t = rng.random()
        x = cx + (rng.random() - 0.5) * 2 * width * (1 - t * 0.7)
        y = floor - t * height - rng.random() * 3
        d.ellipse((x - 2.4, y - 1.6, x + 2.4, y + 1.6), fill=COIN_D)
        d.ellipse((x - 2, y - 1.6, x + 1.6, y + 0.6), fill=COIN)
        d.point((x - 1, y - 1), fill=(255, 250, 214, 255))
    for index in range(bits):
        x = cx + rng.uniform(-width, width)
        y = floor - rng.uniform(0, 4)
        d.rectangle((x, y, x + 4, y + 1), fill=WOOD_3)
    return image


def stave(d, x, y, length, width, angle, fill, light):
    c, s_ = math.cos(angle), math.sin(angle)
    shape = [(-length / 2, 0), (-length / 4, -width), (length / 4, -width * 0.8), (length / 2, -width * 0.2), (length / 2.4, width * 0.7), (length / 5, width), (-length / 3, width * 0.8)]
    pts = [(x + px * c - py * s_, y + px * s_ + py * c) for px, py in shape]
    d.polygon(pts, fill=fill, outline=OUTLINE)
    a = (x + (-length / 3) * c - (-width * 0.5) * s_, y + (-length / 3) * s_ + (-width * 0.5) * c)
    b = (x + (length / 4) * c - (-width * 0.5) * s_, y + (length / 4) * s_ + (-width * 0.5) * c)
    d.line((a[0], a[1], b[0], b[1]), fill=light)


def planks(stage, seed, pile=False):
    rng = random.Random(seed)
    image = Image.new("RGBA", (W, H), CLEAR)
    d = ImageDraw.Draw(image)
    cx, cy = 52, 120
    spread = (14, 28, 40, 0)[stage]
    fall = (0, 6, 18, 0)[stage]
    big = 8
    for index in range(big):
        angle = rng.uniform(0, math.tau)
        reach = rng.uniform(0.35, 1.0) * spread
        x = cx + math.cos(angle) * reach * 1.25
        y = cy - 8 + math.sin(angle) * reach * 0.7 + fall * rng.uniform(0.7, 1.1)
        rot = rng.uniform(-1.2, 1.2) + (0.5 if x > cx else -0.5)
        length = rng.randint(13, 19)
        if pile:
            x = cx + (index - 3.5) * 9 + rng.uniform(-2, 2)
            y = H - 9 - (index % 2) * 4
            rot = rng.uniform(-0.3, 0.3) + (0.15 if index % 2 else -0.15)
            if index > 4:
                continue
        d_fill = (WOOD_3, WOOD_4, WOOD_2)[index % 3]
        stave(d, x, min(y, H - 8), length, rng.choice((3, 4)), rot, d_fill, WOOD_5)
    shard_count = (22, 20, 14, 0)[stage]
    for index in range(shard_count):
        angle = rng.uniform(0, math.tau)
        reach = rng.uniform(0.2, 1.25) * spread
        x = cx + math.cos(angle) * reach * 1.3
        y = cy - 8 + math.sin(angle) * reach * 0.8 + fall * rng.uniform(0.5, 1.2)
        size = rng.choice((1, 2, 2, 3))
        colour = (WOOD_3, WOOD_4, WOOD_2, WOOD_5)[index % 4]
        d.polygon([(x, y), (x + size, y + 1), (x + size * 0.4, y + size)], fill=colour)
        if index % 5 == 0:
            d.point((x + 1, y - 1), fill=METAL_L)
    for index in range((8, 8, 6, 3)[stage]):
        x = cx + rng.uniform(-40, 40) * (0.4 + stage * 0.4)
        y = (H - 9 - rng.uniform(0, 3)) if pile else cy - 8 - rng.uniform(-6, 24) + fall * rng.uniform(0.5, 1.0)
        d.ellipse((x - 2, y - 2, x + 2, y + 2), fill=OUTLINE)
        d.ellipse((x - 1.4, y - 1.4, x + 1.4, y + 1.4), fill=COIN)
        d.point((x - 1, y - 1), fill=(255, 250, 214, 255))
    for index in range((10, 14, 18, 0)[stage]):
        x = cx + rng.uniform(-1, 1) * (30 + stage * 14)
        y = cy - 6 + rng.uniform(-1, 1) * (16 + stage * 8)
        d.point((x, y), fill=(120, 100, 96, 255) if stage < 2 else (96, 80, 76, 255))
    return image


def death():
    return [
        mimic(open_amount=0.8, lick=None, eye=0.6, tear=2, squash=2),
        mimic(open_amount=0.4, lick=None, eye=0.2, tear=12, squash=3),
        mimic(open_amount=0.0, lick=None, eye=0.0, squash=3),
        mimic(open_amount=0.0, lick=None, eye=0.0, squash=1, hop=-2, cracks=True),
        planks(0, 5),
        planks(1, 5),
        planks(2, 5),
        planks(3, 5, pile=True),
    ]


def main():
    rows = [idle(), hop(), bite(), spit(), rest(), death()]
    sheet = Image.new("RGBA", (W * 8, H * len(rows)), CLEAR)
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * W, row * H))
    sheet.save(OUT / "mimic.tga")


main()
