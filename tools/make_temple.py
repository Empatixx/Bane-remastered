"""What the temple floor gets besides its traps, drawn at the tiles' pixel scale (16x16 cells,
shown four times over):

- sand_crack.tga: two cracks in a wall's face in the sandstone's own colours, sand spilling over
  their lips, in the same layout as the sewer's pipes (see make_sewer.py) so the game draws the
  sand falls like the water.
- sand_fall.tga: four frames of sand pouring down the wall, grains glinting as they fall (top
  row), and four of it reaching the floor and sinking into the sand, a few grains flicking aside
  (bottom row).
- fire_bowl.tga: a bronze bowl of fire on a low sandstone pedestal, three frames of its flames.
- statue_pieces.tga: what is left of a toppled statue lying on the floor (one per row): its
  head, the rubble of its plinth, and a broken column drum.

Palettes from the temple tileset.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
S = 16
CLEAR = (0, 0, 0, 0)

WALL0 = (36, 24, 18)
WALL1 = (92, 77, 62)
WALL2 = (130, 106, 74)
WALL3 = (168, 138, 92)
WALL4 = (206, 176, 120)
SAND0 = (150, 118, 70)
SAND1 = (196, 160, 98)
SAND2 = (226, 196, 132)
SAND3 = (246, 226, 170)
BRONZE0 = (60, 36, 20)
BRONZE1 = (120, 76, 36)
BRONZE2 = (170, 116, 52)
BRONZE3 = (220, 168, 84)
FIRE_Y = (255, 236, 150)
FIRE_O = (255, 160, 50)
FIRE_R = (214, 72, 26)
COAL = (60, 30, 20)
MARBLE0 = (70, 64, 60)
MARBLE1 = (150, 142, 132)
MARBLE2 = (196, 188, 176)
MARBLE3 = (230, 224, 212)


class Cell:
    def __init__(self):
        self.image = Image.new("RGBA", (S, S), CLEAR)

    def put(self, x, y, colour, alpha=255):
        if 0 <= x < S and 0 <= y < S:
            self.image.putpixel((x, y), colour + (alpha,))

    def pattern(self, x, y, rows, palette):
        for dy, row in enumerate(rows):
            for dx, key in enumerate(row):
                if key in palette:
                    colour = palette[key]
                    if len(colour) == 4:
                        self.put(x + dx, y + dy, colour[:3], colour[3])
                    else:
                        self.put(x + dx, y + dy, colour)


def sheet(rows):
    out = Image.new("RGBA", (S * max(len(row) for row in rows), S * len(rows)), CLEAR)
    for r, row in enumerate(rows):
        for c, cell in enumerate(row):
            out.paste(cell.image, (c * S, r * S))
    return out


def crack(variant):
    """A jagged crack in the wall's face, dark inside, its rim lit from above and sand spilling
    over its lower lip and staining the stone beneath."""
    c = Cell()
    shapes = [
        [
            ".....ww.........",
            "....wkkw..w.....",
            "...wkkkkwwkw....",
            "..wkkkkkkkkkw...",
            "..wkkkkkkkkkkw..",
            "..wkkkkkkkkkw...",
            "...SsshssshsS...",
            "....SsssssssS...",
            ".....tSssSt.....",
            "......tsst......",
            "......tsst......",
        ],
        [
            ".........ww.....",
            "....w..wwkkw....",
            "...wkwwkkkkw....",
            "..wkkkkkkkkkw...",
            ".wkkkkkkkkkkw...",
            "..wkkkkkkkkkw...",
            "..SsshsssshsS...",
            "...SsssssssS....",
            ".....tSssSt.....",
            "......tsst......",
            "......tsst......",
        ],
    ]
    c.pattern(0, 2, shapes[variant], {"w": WALL4, "k": WALL0, "s": SAND1, "S": SAND0, "h": SAND3,
                                      "t": WALL1})
    return c


def fall(frame):
    """Sand pouring down: a loose, grainy stream, thicker in the middle, with grains scattering
    out of its edges as they fall."""
    c = Cell()
    for y in range(S):
        for x in range(5, 11):
            core = x in (7, 8)
            seed = (x * 7 + (y + frame * 4) * 13) % 11
            if core:
                c.put(x, y, SAND2 if seed < 4 else SAND1, 230)
            elif x in (6, 9):
                if seed < 7:
                    c.put(x, y, SAND1 if seed < 4 else SAND0, 200)
            elif seed < 3:
                c.put(x, y, SAND2, 170)
        # Loose grains drifting out wider.
        for x, offset in ((3, 1), (12, 4), (4, 9)):
            if (y + frame * 4 + offset) % 9 == 0:
                c.put(x, y, SAND2, 200)
    for g in range(3):
        c.put(7 + g % 2, (frame * 4 + g * 5) % S, SAND3)
    return c


def landing(frame):
    """Where the sand reaches the floor: the stream runs on down and sinks in, thinning out,
    a few grains flicking aside."""
    c = fall(frame)
    for y in range(S):
        fade = max(0, 255 - y * 22)
        for x in range(S):
            r, g, b, alpha = c.image.getpixel((x, y))
            if alpha:
                c.image.putpixel((x, y), (r, g, b, min(alpha, fade)))
    spray = [[(4, 9), (11, 8)], [(3, 8), (12, 9)], [(4, 7), (11, 10)], [(5, 9), (10, 7)]][frame]
    for x, y in spray:
        c.put(x, y, SAND2, 160)
    return c


def fire_bowl(frame):
    c = Cell()
    flames = [
        [
            ".......y........",
            "......yoy.......",
            ".....yoroy..y...",
            "....yoorroyyoy..",
            "...yorrrrroory..",
        ],
        [
            "........y.......",
            ".......yoy......",
            "...y..yoroy.....",
            "..yoy.yorrooy...",
            "...yoorrrrrooy..",
        ],
        [
            "......y.........",
            ".....yoy...y....",
            ".....yroy.yoy...",
            "....yorroyoroy..",
            "...yoorrrrrroy..",
        ],
    ][frame]
    c.pattern(0, 0, flames, {"y": FIRE_Y, "o": FIRE_O, "r": FIRE_R})
    c.pattern(0, 5, [
        "..dhhhhhhhhhhd..",
        "..dbcrcccrcbbd..",
        "...dbbbbbbbbd...",
        "....ddmmmmdd....",
        ".....wWWWWw.....",
        ".....wWwwWw.....",
        ".....wWwwWw.....",
        ".....wWwwWw.....",
        "....wWWWWWWw....",
        "....kkkkkkkk....",
    ], {"d": BRONZE0, "b": BRONZE2, "h": BRONZE3, "m": BRONZE1, "c": COAL, "r": FIRE_R,
        "w": WALL2, "W": WALL4, "k": WALL0})
    return c


STATUE = {"a": MARBLE0, "b": MARBLE1, "c": MARBLE2, "d": MARBLE3, "e": (40, 34, 30)}

PIECES = [
    # The head on its side, its face to the sky, a crack across the brow.
    [
        "................",
        "................",
        ".....aaaaa......",
        "...aacdddcaa....",
        "..acddddddca....",
        "..acdecdecdba...",
        ".acddddddddba...",
        ".acdddcddddba...",
        ".acdcddddcdba...",
        "..acddeedcba....",
        "..abcdddcbba....",
        "...abbbbbba.....",
        "....aaaaaa......",
        "................",
        "................",
        "................",
    ],
    # Rubble of a broken plinth: a carved block and a few chips.
    [
        "................",
        "................",
        "................",
        "......aaaa......",
        ".....acddba.....",
        "....acdddbba....",
        "...acdcddbbba...",
        "...abcddcbbba.aa",
        "....abbbbbba.acd",
        ".aa..aaaaaa..abb",
        "acda..........aa",
        "abba.....aa.....",
        ".aa.....acba....",
        ".........aa.....",
        "................",
        "................",
    ],
]


def column():
    """A fluted column drum on its side, its broken end towards the viewer's right."""
    rows = [["."] * S for _ in range(S)]
    for y in range(4, 12):
        for x in range(2, 12):
            edge = y in (4, 11) or x == 2
            rows[y][x] = "a" if edge else ("d" if y % 2 else "c")
    for y in range(3, 13):
        for x in range(10, 15):
            dx, dy = (x - 12) / 2.9, (y - 7.5) / 4.6
            if dx * dx + dy * dy <= 1.0:
                inner = dx * dx + dy * dy <= 0.45
                rows[y][x] = "b" if inner else ("a" if dx * dx + dy * dy > 0.8 else "c")
    return ["".join(row) for row in rows]


PIECES.append(column())


def piece(rows):
    """The piece and its shadow on the floor, down and to the right of it."""
    c = Cell()
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            below = rows[y + 1][x + 1] if y + 1 < S and x + 1 < S else "."
            if key != "." and below == ".":
                c.put(x + 1, y + 1, (0, 0, 0), 90)
    c.pattern(0, 0, rows, STATUE)
    return c


def main():
    sheet([[crack(0), crack(1)]]).save(OUT / "sand_crack.tga")
    sheet([[fall(f) for f in range(4)], [landing(f) for f in range(4)]]).save(OUT / "sand_fall.tga")
    sheet([[fire_bowl(f) for f in range(3)]]).save(OUT / "fire_bowl.tga")
    sheet([[piece(rows)] for rows in PIECES]).save(OUT / "statue_pieces.tga")


if __name__ == "__main__":
    main()
