import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

CELL = 16
FRAMES = 6
INK = (24, 18, 26, 255)
BLOOD_D = (92, 8, 16, 255)
BLOOD = (160, 18, 30, 255)
BLOOD_L = (226, 64, 64, 255)
SHINE = (255, 170, 160, 255)


def blank():
    return Image.new("RGBA", (CELL, CELL), (0, 0, 0, 0))


def put(img, pixels, color):
    for x, y in pixels:
        img.putpixel((x, y), color)


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
    return img


def drop_tiny():
    img = blank()
    put(img, [(7, 7), (8, 7), (7, 8), (8, 8)], BLOOD)
    put(img, [(7, 8), (8, 8)], BLOOD_D)
    put(img, [(7, 7)], BLOOD_L)
    return outline(img)


def drop_small():
    img = blank()
    put(img, [(7, 4), (8, 4), (7, 5), (8, 5), (6, 6), (7, 6), (8, 6), (9, 6), (6, 7), (7, 7), (8, 7), (9, 7), (7, 8), (8, 8)], BLOOD)
    put(img, [(8, 7), (9, 7), (7, 8), (8, 8), (9, 6)], BLOOD_D)
    put(img, [(7, 6), (7, 5)], BLOOD_L)
    return outline(img)


def drop_big():
    img = blank()
    put(img, [(7, 2), (8, 2), (7, 3), (8, 3), (7, 4), (8, 4)], BLOOD)
    for y in range(5, 11):
        put(img, [(x, y) for x in range(5, 11)], BLOOD)
    put(img, [(6, 11), (7, 11), (8, 11), (9, 11)], BLOOD)
    put(img, [(10, 7), (10, 8), (10, 9), (9, 10), (9, 11), (8, 11), (6, 11), (10, 6)], BLOOD_D)
    put(img, [(6, 6), (6, 7), (7, 5), (7, 4)], BLOOD_L)
    put(img, [(6, 6)], SHINE)
    return outline(img)


def drop_stretched():
    img = blank()
    put(img, [(7, 2), (8, 2)], BLOOD)
    for y in range(3, 7):
        put(img, [(7, y), (8, y)], BLOOD)
    for y in range(7, 11):
        put(img, [(x, y) for x in range(6, 10)], BLOOD)
    put(img, [(7, 11), (8, 11)], BLOOD)
    put(img, [(9, 8), (9, 9), (9, 10), (8, 11), (8, 6), (8, 5)], BLOOD_D)
    put(img, [(6, 8), (7, 7), (7, 5), (7, 4)], BLOOD_L)
    return outline(img)


def splash():
    img = blank()
    put(img, [(7, 8), (8, 8), (7, 9), (8, 9)], BLOOD)
    put(img, [(8, 9)], BLOOD_D)
    put(img, [(4, 6), (5, 6), (4, 7)], BLOOD)
    put(img, [(10, 5), (11, 5), (11, 6)], BLOOD)
    put(img, [(7, 3), (7, 4)], BLOOD)
    put(img, [(3, 10), (12, 10)], BLOOD_D)
    put(img, [(4, 6), (10, 5), (7, 3), (7, 8)], BLOOD_L)
    return outline(img)


def splash_fade():
    img = blank()
    put(img, [(3, 5), (12, 4), (6, 2), (10, 11), (4, 11)], BLOOD)
    put(img, [(8, 9), (2, 8), (13, 8)], BLOOD_D)
    put(img, [(3, 5), (12, 4)], BLOOD_L)
    return img


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    sheet = Image.new("RGBA", (CELL * FRAMES, CELL), (0, 0, 0, 0))
    for index, frame in enumerate((drop_tiny, drop_small, drop_big, drop_stretched, splash, splash_fade)):
        sheet.paste(frame(), (index * CELL, 0))
    sheet.save(OUT / "blood_drops.tga")
    if len(sys.argv) > 1:
        sheet.resize((sheet.width * 8, sheet.height * 8), Image.NEAREST).save(OUT / "blood_drops_preview.png")


if __name__ == "__main__":
    main()
