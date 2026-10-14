import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/ingame"

PALETTE = {
    (136, 203, 89, 255): (224, 212, 236, 255),
    (51, 112, 8, 255): (118, 98, 142, 255),
    (160, 216, 121, 255): (248, 244, 252, 255),
    (93, 173, 38, 255): (186, 168, 204, 255),
    (109, 186, 57, 255): (204, 188, 220, 255),
}
STRAND = (236, 228, 244, 255)
STRAND_D = (176, 156, 196, 255)
BUBBLE = (250, 248, 255, 255)
SRC_W, SRC_H = 32, 24
CELL_W, CELL_H = 32, 32
LIFT = 6


def recolor(cell):
    out = Image.new("RGBA", cell.size, (0, 0, 0, 0))
    for y in range(cell.height):
        for x in range(cell.width):
            pixel = cell.getpixel((x, y))
            if pixel[3]:
                out.putpixel((x, y), PALETTE.get(pixel, pixel))
    return out


def place(cell, dy=0):
    image = Image.new("RGBA", (CELL_W, CELL_H), (0, 0, 0, 0))
    image.paste(cell, (0, LIFT + dy), cell)
    return image


def bottom_edge(image, x):
    for y in range(CELL_H - 1, -1, -1):
        if image.getpixel((x, y))[3]:
            return y
    return None


def drool(image, columns, length):
    for x in columns:
        edge = bottom_edge(image, x)
        if edge is None or edge + length >= CELL_H:
            continue
        for step in range(1, length + 1):
            image.putpixel((x, edge + step), STRAND if step % 2 else STRAND_D)
        if edge + length + 1 < CELL_H:
            image.putpixel((x, edge + length + 1), BUBBLE)


def bubbles(image, spots):
    for x, y in spots:
        if 0 < x < CELL_W - 1 and 0 < y < CELL_H - 1 and image.getpixel((x, y))[3]:
            for dx, dy in ((0, 0),):
                image.putpixel((x + dx, y + dy), BUBBLE)
            for dx, dy in ((1, 0), (0, 1)):
                if image.getpixel((x + dx, y + dy))[3]:
                    image.putpixel((x + dx, y + dy), (214, 200, 230, 255))


def main():
    source = Image.open(ROOT / "assets/textures/ingame/slime.tga").convert("RGBA")
    cells = [[recolor(source.crop((c * SRC_W, r * SRC_H, (c + 1) * SRC_W, (r + 1) * SRC_H))) for c in range(6)] for r in range(2)]
    idle = [place(cells[0][i]) for i in range(4)]
    for index, frame in enumerate(idle):
        drool(frame, (9 + index % 2, 22 - index % 3), 1 + index % 2)
        bubbles(frame, ((12 + index, 17), (21 - index, 19), (17, 14 + index % 2)))
    death = [place(cells[1][i]) for i in range(6)]
    for index in (0, 1):
        drool(death[index], (10, 21), 1)
        bubbles(death[index], ((13, 17), (20, 18)))
    hop = [
        place(cells[0][1], 1),
        place(cells[0][3], -2),
        place(cells[0][3], -5),
        place(cells[0][3], -6),
        place(cells[0][1], 0),
        place(cells[0][0], 0),
    ]
    for index, frame in enumerate(hop):
        drool(frame, (11, 20), 2 + (1 if index in (2, 3) else 0))
        bubbles(frame, ((14, 15), (20, 17)))
    reform = list(reversed(death))
    rows = [idle, death, hop, reform]
    sheet = Image.new("RGBA", (CELL_W * 6, CELL_H * 4), (0, 0, 0, 0))
    for row, frames in enumerate(rows):
        for column, frame in enumerate(frames):
            sheet.paste(frame, (column * CELL_W, row * CELL_H))
    sheet.save(OUT / "saliva_slime.tga")


main()
