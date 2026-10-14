import math
import random
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

CLEAR = (0, 0, 0, 0)
WHITE = (255, 255, 246, 255)
VOLT_L = (255, 246, 168, 255)
VOLT = (255, 222, 84, 255)
ARC_L = (150, 200, 255, 235)
ARC = (96, 128, 255, 200)
HAZE = (118, 70, 214, 130)
HAZE_D = (74, 40, 150, 80)

BOLT_W, BOLT_H, BOLT_FRAMES = 32, 16, 4
BURST, BURST_FRAMES = 32, 6
SPARK, SPARK_FRAMES = 8, 3

# Strongest colour wins when layers overlap.
RANK = {c: i for i, c in enumerate([HAZE_D, HAZE, ARC, ARC_L, VOLT, VOLT_L, WHITE])}


class Cell:
    def __init__(self, w, h):
        self.w, self.h = w, h
        self.px = {}

    def put(self, x, y, c):
        x, y = int(round(x)), int(round(y))
        if 0 <= x < self.w and 0 <= y < self.h:
            old = self.px.get((x, y))
            if old is None or RANK[c] > RANK[old]:
                self.px[(x, y)] = c

    def line(self, x0, y0, x1, y1, c):
        steps = max(abs(x1 - x0), abs(y1 - y0), 1)
        for i in range(int(steps) + 1):
            t = i / steps
            self.put(x0 + (x1 - x0) * t, y0 + (y1 - y0) * t, c)

    def glow(self, core, layers):
        """Rings of colour around the pixels in `core`, in 4-neighbour steps."""
        ring = set(core)
        seen = set(core)
        for c in layers:
            nxt = set()
            for x, y in ring:
                for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                    p = (x + dx, y + dy)
                    if p not in seen:
                        nxt.add(p)
            for p in nxt:
                self.put(p[0], p[1], c)
            seen |= nxt
            ring = nxt

    def image(self):
        img = Image.new("RGBA", (self.w, self.h), CLEAR)
        for (x, y), c in self.px.items():
            img.putpixel((x, y), c)
        return img


def jagged(rng, x0, y0, x1, y1, spread, kinks):
    pts = [(x0, y0)]
    for i in range(1, kinks):
        t = i / kinks
        side = 1 if i % 2 else -1
        off = side * rng.uniform(spread * 0.4, spread)
        pts.append((x0 + (x1 - x0) * t + rng.uniform(-1, 1), y0 + (y1 - y0) * t + off))
    pts.append((x1, y1))
    return pts


def trace(cell, pts, c):
    path = []
    for (ax, ay), (bx, by) in zip(pts, pts[1:]):
        ax, ay, bx, by = round(ax), round(ay), round(bx), round(by)
        steps = max(abs(bx - ax), abs(by - ay), 1)
        for i in range(steps + 1):
            p = (round(ax + (bx - ax) * i / steps), round(ay + (by - ay) * i / steps))
            path.append(p)
            cell.put(p[0], p[1], c)
    return path


def bolt_frame(seed):
    rng = random.Random(seed)
    cell = Cell(BOLT_W, BOLT_H)
    mid = BOLT_H // 2
    xs = sorted(rng.sample(range(3, BOLT_W - 3), rng.choice([4, 5, 6])))
    pts, side = [(0, mid)], rng.choice([-1, 1])
    for x in xs:
        side = -side if rng.random() < 0.8 else side
        pts.append((x, mid + side * rng.randint(1, 4)))
    pts.append((BOLT_W - 1, mid))
    path = trace(cell, pts, WHITE)
    branches = []
    for _ in range(rng.choice([1, 2, 2])):
        sx, sy = path[rng.randrange(5, len(path) - 6)]
        dirn = 1 if sy >= mid else -1
        ex = sx + rng.randint(4, 7)
        ey = min(max(sy + dirn * rng.randint(3, 5), 1), BOLT_H - 2)
        branches += trace(cell, jagged(rng, sx, sy, ex, ey, 1.0, 2), VOLT_L)
    cell.glow(path, [VOLT_L, ARC, HAZE])
    cell.glow(branches, [ARC, HAZE_D])
    # The very ends hug the centre line so pieces join seamlessly.
    for x in (0, BOLT_W - 1):
        for y in range(BOLT_H):
            cell.px.pop((x, y), None)
        for y, c in ((mid - 2, ARC_L), (mid - 1, VOLT_L), (mid, WHITE), (mid + 1, VOLT_L), (mid + 2, ARC_L)):
            cell.put(x, y, c)
    return cell.image()


def burst_frame(index, seed):
    rng = random.Random(seed)
    cell = Cell(BURST, BURST)
    c = (BURST - 1) / 2
    t = index / (BURST_FRAMES - 1)
    core_r = [3.5, 5.5, 4.0, 2.5, 1.2, 0.0][index]
    if core_r > 0:
        core = []
        for y in range(BURST):
            for x in range(BURST):
                d = math.hypot(x - c, y - c)
                if d <= core_r:
                    core.append((x, y))
                    cell.put(x, y, WHITE if d <= core_r * 0.55 else VOLT_L)
        cell.glow(core, [VOLT, ARC_L, ARC, HAZE][: 4 - index // 2])
    ring_r = [0, 7, 10, 12, 13, 14][index]
    if ring_r:
        ring_c = [None, ARC_L, ARC_L, ARC, HAZE, HAZE_D][index]
        gaps = rng.random() * math.tau
        for a in range(0, 360, 4):
            r = math.radians(a)
            if index >= 3 and math.sin(r * 3 + gaps) > (0.35 if index == 3 else -0.2):
                continue
            cell.put(c + math.cos(r) * ring_r, c + math.sin(r) * ring_r, ring_c)
    arms = [0, 6, 7, 6, 4, 3][index]
    base = rng.random() * math.tau
    for k in range(arms):
        a = base + k * math.tau / max(arms, 1) + rng.uniform(-0.35, 0.35)
        inner = [2, 3, 5, 8, 11, 13][index]
        outer = inner + [0, 7, 9, 6, 4, 2][index] + rng.uniform(-1, 1)
        x0, y0 = c + math.cos(a) * inner, c + math.sin(a) * inner
        x1, y1 = c + math.cos(a) * outer, c + math.sin(a) * outer
        pts = jagged(rng, x0, y0, x1, y1, 1.6, 3)
        hot = [WHITE, WHITE, VOLT_L, VOLT, ARC_L, ARC][index]
        arm = trace(cell, pts, hot)
        if index <= 3:
            cell.glow(arm, [ARC, HAZE_D] if index < 3 else [HAZE_D])
    if index >= 3:
        for _ in range([0, 0, 0, 5, 7, 5][index]):
            a = rng.random() * math.tau
            d = rng.uniform(6, 15)
            cell.put(c + math.cos(a) * d, c + math.sin(a) * d, [VOLT, VOLT, VOLT, VOLT_L, ARC_L, ARC][index])
    return cell.image()


def spark_frame(index):
    cell = Cell(SPARK, SPARK)
    c = SPARK // 2
    if index == 0:
        for d in range(-3, 4):
            cell.put(c + d, c, ARC_L if abs(d) == 3 else VOLT)
            cell.put(c, c + d, ARC_L if abs(d) == 3 else VOLT)
        for p in ((c - 1, c - 1), (c + 1, c - 1), (c - 1, c + 1), (c + 1, c + 1)):
            cell.put(*p, ARC)
        cell.put(c, c, WHITE)
        cell.put(c - 1, c, VOLT_L)
        cell.put(c + 1, c, VOLT_L)
        cell.put(c, c - 1, VOLT_L)
        cell.put(c, c + 1, VOLT_L)
    elif index == 1:
        for d in range(-2, 3):
            cell.put(c + d, c, ARC if abs(d) == 2 else VOLT)
            cell.put(c, c + d, ARC if abs(d) == 2 else VOLT)
        cell.put(c, c, VOLT_L)
    else:
        cell.put(c, c, ARC_L)
        cell.put(c - 1, c, ARC)
        cell.put(c, c - 1, ARC)
        cell.put(c + 1, c, HAZE)
        cell.put(c, c + 1, HAZE)
    return cell.image()


def sheet(frames, w, h):
    img = Image.new("RGBA", (w * len(frames), h), CLEAR)
    for i, f in enumerate(frames):
        img.paste(f, (i * w, 0))
    return img


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    bolts = sheet([bolt_frame(11 + i * 7) for i in range(BOLT_FRAMES)], BOLT_W, BOLT_H)
    bursts = sheet([burst_frame(i, 300) for i in range(BURST_FRAMES)], BURST, BURST)
    sparks = sheet([spark_frame(i) for i in range(SPARK_FRAMES)], SPARK, SPARK)
    bolts.save(OUT / "lightning_bolt.tga")
    bursts.save(OUT / "lightning_burst.tga")
    sparks.save(OUT / "lightning_spark.tga")
    if len(sys.argv) > 2:
        preview(Path(sys.argv[2]), bolts, bursts, sparks)


def preview(path, bolts, bursts, sparks):
    scale = 8
    bg = (28, 26, 34, 255)
    w = max(bolts.width, bursts.width, sparks.width) * scale + 32
    # A chain made of pieces, as the game lays them out.
    chain = Image.new("RGBA", (BOLT_W * 4, BOLT_H), CLEAR)
    for i, f in enumerate((2, 0, 3, 1)):
        chain.alpha_composite(bolts.crop((f * BOLT_W, 0, (f + 1) * BOLT_W, BOLT_H)), (i * BOLT_W, 0))
    rows = [bolts, chain, bursts, sparks]
    h = sum(r.height * scale + 16 for r in rows) + 16
    out = Image.new("RGBA", (max(w, chain.width * scale + 32), h), bg)
    y = 16
    for r in rows:
        big = r.resize((r.width * scale, r.height * scale), Image.NEAREST)
        out.alpha_composite(big, (16, y))
        y += big.height + 16
    out.save(path)


if __name__ == "__main__":
    main()
