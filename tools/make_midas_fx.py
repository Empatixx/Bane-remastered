import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/artefacts"

CELL = 32
FRAMES = 6
MID = CELL // 2
GOLD_D = (138, 88, 28, 255)
GOLD = (214, 160, 54, 255)
GOLD_L = (250, 222, 120, 255)
GOLD_S = (255, 250, 214, 255)

ARM_LENGTHS = (3, 7, 12, 9, 5, 2)
GLINTS = (
    ((-9, -8),),
    ((-9, -8), (9, -6)),
    ((-10, -9), (10, -7), (8, 9)),
    ((-10, -9), (11, -6), (9, 10), (-8, 8)),
    ((-11, -10), (10, 9), (-9, 9)),
    ((10, -9),),
)


def star(img, length):
    for step in range(1, length + 1):
        color = GOLD_L if step <= length * 0.45 else GOLD if step <= length * 0.8 else GOLD_D
        for dx, dy in ((step, 0), (-step, 0), (0, step), (0, -step)):
            img.putpixel((MID + dx, MID + dy), color)
        if step <= length // 2 + 1:
            for dx, dy in ((1, 1), (-1, 1), (1, -1), (-1, -1)):
                if step == 1:
                    img.putpixel((MID + dx, MID + dy), GOLD_L)
    for dx, dy in ((0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)):
        img.putpixel((MID + dx, MID + dy), GOLD_S)
    diagonal = max(1, length // 3)
    for step in range(2, diagonal + 2):
        for sx, sy in ((1, 1), (-1, 1), (1, -1), (-1, -1)):
            img.putpixel((MID + sx * step, MID + sy * step), GOLD)


def glint(img, cx, cy, strong):
    center = GOLD_S if strong else GOLD_L
    arm = GOLD_L if strong else GOLD
    img.putpixel((MID + cx, MID + cy), center)
    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
        img.putpixel((MID + cx + dx, MID + cy + dy), arm)


def frame(index):
    img = Image.new("RGBA", (CELL, CELL), (0, 0, 0, 0))
    for position, (cx, cy) in enumerate(GLINTS[index]):
        glint(img, cx, cy, (position + index) % 2 == 0)
    star(img, ARM_LENGTHS[index])
    return img


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    sheet = Image.new("RGBA", (CELL * FRAMES, CELL), (0, 0, 0, 0))
    for index in range(FRAMES):
        sheet.paste(frame(index), (index * CELL, 0))
    sheet.save(OUT / "midas_sparkle.tga")
    if len(sys.argv) > 1:
        sheet.resize((sheet.width * 8, sheet.height * 8), Image.NEAREST).save(OUT / "midas_sparkle_preview.png")


if __name__ == "__main__":
    main()
