import math
import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

CELL = 24
FLOAT_FRAMES = 4
HIT_FRAMES = 3
COLUMNS = 4
ROWS = 2

CLEAR = (0, 0, 0, 0)
INK = (24, 18, 26, 255)
SPIRIT_D = (40, 140, 70, 240)
SPIRIT = (90, 220, 110, 240)
SPIRIT_L = (170, 250, 170, 245)
SPIRIT_S = (226, 255, 214, 255)
FLASH = (236, 255, 232, 255)
FLASH_D = (168, 236, 178, 255)
EYE = (20, 44, 34, 255)
SPARK = (255, 255, 240, 255)


def body_mask(phase, squash=0.0):
    cx, cy, r = 11.5, 10.0 + squash, 7.0 + squash * 0.3
    mask = set()
    for y in range(CELL):
        for x in range(CELL):
            dx = x + 0.5 - cx
            dy = y + 0.5 - cy
            if dy <= 0:
                inside = dx * dx + dy * dy <= r * r
            else:
                half = r - dy * 0.12 - max(0.0, dy - 5.0) * 0.25
                bottom = cy + 7.0 + 1.6 * math.sin(x * 0.95 + phase)
                inside = abs(dx) <= half and y + 0.5 <= bottom
            if inside:
                mask.add((x, y))
    return mask


def shade(x, y, flash):
    light = (x - 7.0) + (y - 5.0) * 0.8
    dark = (x - 11.5) + (y - 10.0) * 0.8
    if flash:
        return FLASH if light < 6.0 else FLASH_D
    if light < 2.5:
        return SPIRIT_L
    if dark > 5.5:
        return SPIRIT_D
    return SPIRIT


def draw_face(img, hit):
    if not hit:
        for ex in (8, 13):
            for dy in range(3):
                img.putpixel((ex, 8 + dy), EYE)
                img.putpixel((ex + 1, 8 + dy), EYE)
            img.putpixel((ex, 8), SPIRIT_S)
        img.putpixel((11, 12), EYE)
        img.putpixel((12, 12), EYE)
    else:
        for step in range(3):
            img.putpixel((7 + step, 8 + step % 2), EYE)
            img.putpixel((15 - step, 8 + step % 2), EYE)
        for x in (10, 11, 12, 13):
            img.putpixel((x, 12), EYE)
            img.putpixel((x, 13), EYE)
        img.putpixel((11, 12), SPIRIT_S)


def outline(img):
    src = img.copy()
    for y in range(CELL):
        for x in range(CELL):
            if src.getpixel((x, y))[3]:
                continue
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < CELL and 0 <= ny < CELL and src.getpixel((nx, ny))[3]:
                    img.putpixel((x, y), INK)
                    break


def frame(index, hit):
    img = Image.new("RGBA", (CELL, CELL), CLEAR)
    phase = index * math.pi / 2
    squash = (0.0, 1.2, 0.4)[index] if hit else 0.0
    bob = 0 if hit else (0, -1, 0, 1)[index % 4]
    mask = body_mask(phase + (1.0 if hit else 0.0), squash)
    for x, y in mask:
        ny = y + bob
        if 0 <= ny < CELL:
            img.putpixel((x, ny), shade(x, y, hit and index == 0))
    if bob:
        face = Image.new("RGBA", (CELL, CELL), CLEAR)
        draw_face(face, hit)
        img.alpha_composite(face, (0, bob))
    else:
        draw_face(img, hit)
    if hit:
        spark_sets = (
            ((3, 3), (20, 4), (4, 17), (19, 16)),
            ((2, 2), (21, 3), (3, 18), (20, 18)),
            ((1, 1), (22, 2)),
        )
        for sx, sy in spark_sets[index]:
            img.putpixel((sx, sy), SPARK)
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                if 0 <= sx + dx < CELL and 0 <= sy + dy < CELL and index < 2:
                    img.putpixel((sx + dx, sy + dy), SPIRIT_L)
    outline(img)
    return img


def main():
    sheet = Image.new("RGBA", (CELL * COLUMNS, CELL * ROWS), CLEAR)
    for i in range(FLOAT_FRAMES):
        sheet.paste(frame(i, False), (i * CELL, 0))
    for i in range(HIT_FRAMES):
        sheet.paste(frame(i, True), (i * CELL, CELL))
    OUT.mkdir(parents=True, exist_ok=True)
    sheet.save(OUT / "ghost.tga")
    print("wrote", OUT / "ghost.tga")


if __name__ == "__main__":
    main()
