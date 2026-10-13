import sys
from collections import Counter
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame/keyboard"
FW, FH = 11, 12
WIDE = 33


def wide_frame(frame):
    out = Image.new("RGBA", (WIDE, FH), (0, 0, 0, 0))
    out.paste(frame.crop((0, 0, 5, FH)), (0, 0))
    for x in range(5, WIDE - 5):
        out.paste(frame.crop((5, 0, 6, FH)), (x, 0))
    out.paste(frame.crop((6, 0, FW, FH)), (WIDE - 5, 0))
    interior = [frame.getpixel((x, y)) for y in range(3, 9) for x in range(3, 8)]
    ranked = Counter(interior).most_common(2)
    body = ranked[0][0]
    glyph = ranked[1][0] if len(ranked) > 1 else (255, 255, 255, 255)
    for y in range(2, FH - 2):
        for x in range(2, WIDE - 2):
            out.putpixel((x, y), body)
    for x in range(9, WIDE - 9):
        out.putpixel((x, 8), glyph)
    for y in (6, 7):
        out.putpixel((9, y), glyph)
        out.putpixel((WIDE - 10, y), glyph)
    return out


def main():
    source = Image.open(OUT / "w.tga").convert("RGBA")
    sheet = Image.new("RGBA", (WIDE * 2, FH), (0, 0, 0, 0))
    for index in range(2):
        sheet.paste(wide_frame(source.crop((index * FW, 0, (index + 1) * FW, FH))), (index * WIDE, 0))
    sheet.save(OUT / "space.tga")


main()
