import math
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

W, H = 20, 10
FRAMES = 4
CLEAR = (0, 0, 0, 0)
INK = (24, 18, 26, 255)
GOLD_D = (168, 108, 30, 255)
GOLD = (255, 209, 64, 255)
GOLD_L = (255, 240, 150, 255)
GLINT = (255, 252, 226, 255)


def lemniscate():
    points = set()
    a = 8.4
    for step in range(720):
        t = step / 720 * math.tau
        k = 1 + math.sin(t) ** 2
        x = a * math.cos(t) / k + (W - 1) / 2
        y = a * 1.15 * math.sin(t) * math.cos(t) / k + (H - 1) / 2
        points.add((round(x), round(y)))
    return {(x, y) for x, y in points if 1 <= x < W - 1 and 1 <= y < H - 1}


def frame(index, body):
    img = Image.new("RGBA", (W, H), CLEAR)
    px = img.load()
    for x, y in body:
        px[x, y] = GOLD_L if (x, y - 1) not in body else GOLD_D if (x, y + 1) not in body else GOLD
    for x, y in [(x, y) for y in range(H) for x in range(W)]:
        if (x, y) in body:
            continue
        if any((x + dx, y + dy) in body for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))):
            px[x, y] = INK
    glint_x = [3, 7, 12, 16][index]
    spots = sorted(body, key=lambda p: abs(p[0] - glint_x) + abs(p[1] - 4.5))
    cx, cy = spots[0]
    px[cx, cy] = GLINT
    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        if (cx + dx, cy + dy) in body:
            px[cx + dx, cy + dy] = GOLD_L
    return img


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    body = lemniscate()
    sheet = Image.new("RGBA", (W * FRAMES, H), CLEAR)
    for index in range(FRAMES):
        sheet.paste(frame(index, body), (W * index, 0))
    sheet.save(OUT / "infinity_glint.tga")
    print(OUT / "infinity_glint.tga")


if __name__ == "__main__":
    main()
