"""A mine track across a room: a piece of rail that repeats, the buffer stop at each end, and the
mine cart in two pictures (its wheels turned a little between them), one look per floor.

rails.tga has a row per floor, in the order of `Theme::style`, of five 16x16 cells: rail, buffer
stop, cart, cart rolling, and a quarter turn of rail from the left edge round to the bottom edge.
The game turns the rail, the stop and the turn to whichever way the track runs. Each floor's
track has its own sleepers, rails and cart, and the cart its own load: coal (stone), sludge
(sewer), mushrooms (moss), a cocoon (spider), ice (ice), glowing ore (lava) and gold (temple).
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
W = H = 16
CLEAR = (0, 0, 0, 0)

PALETTE = {
    # Sleepers.
    "w": (92, 62, 40), "W": (128, 88, 56), "q": (58, 38, 26),
    # Rails.
    "r": (150, 146, 156), "R": (196, 192, 200), "e": (70, 66, 78),
    # Cart body: rusty iron with a lighter rim.
    "d": (34, 28, 30), "m": (96, 64, 52), "l": (132, 88, 64), "h": (176, 124, 84), "u": (150, 74, 46),
    # Wheels.
    "k": (40, 38, 46), "g": (110, 108, 120),
    # Buffer stop: a timber with an iron plate.
    "b": (120, 82, 50), "B": (160, 114, 70), "p": (84, 82, 96), "P": (140, 138, 150),
    "o": (28, 22, 20),
}

RAIL = [
    "................",
    "................",
    "..WW......WW....",
    "..ww......ww....",
    "RRRRRRRRRRRRRRRR",
    "rrrrrrrrrrrrrrrr",
    "eewweeeeeewweeee",
    "..ww......ww....",
    "..ww......ww....",
    "..ww......ww....",
    "RRRRRRRRRRRRRRRR",
    "rrrrrrrrrrrrrrrr",
    "eewweeeeeewweeee",
    "..qq......qq....",
    "................",
    "................",
]

BUFFER = [
    "................",
    "....oooooooo....",
    "...oBBBBBBBBo...",
    "...obbbbbbbbo...",
    "RRRobPPPPPPbo...",
    "rrrobpPPPPpbo...",
    "eeeobbbbbbbbo...",
    "...obbbbbbbbo...",
    "...obbbbbbbbo...",
    "...obpPPPPpbo...",
    "RRRobPPPPPPbo...",
    "rrrobbbbbbbbo...",
    "eeeoqqqqqqqqo...",
    "....oooooooo....",
    "................",
    "................",
]

CART = [
    "................",
    "..dddddddddddd..",
    ".dhhhhhhhhhhhhd.",
    ".dhllllllllllhd.",
    ".dlmdddddddmmld.",
    ".dlmdcCcCcdmmld.",
    ".dlmdCccCcdmuld.",
    ".dlmdddddddmmld.",
    ".dlmmmmmmmmmmld.",
    ".dlllulllllllld.",
    ".dmmmmmmmmmmmmd.",
    "..dddddddddddd..",
    "..kgk......kgk..",
    "..kkk......kkk..",
    "................",
    "................",
]
CART_ROLLING = CART[:12] + [
    "..gkk......gkk..",
    "..kkg......kkg..",
    "................",
    "................",
]


# Each floor's own colours over the stone floor's: sleepers, rails, the cart's body and its load.
LOOKS = [
    {"c": (50, 48, 56), "C": (84, 82, 92)},
    {"w": (60, 62, 40), "W": (84, 88, 56), "q": (38, 40, 26), "r": (130, 110, 96), "R": (170, 150, 130),
     "e": (70, 56, 50), "d": (24, 40, 34), "m": (52, 86, 70), "l": (76, 118, 98), "h": (112, 156, 130),
     "u": (88, 140, 64), "c": (70, 110, 50), "C": (110, 150, 70)},
    {"w": (70, 62, 36), "W": (100, 90, 52), "q": (46, 40, 24), "d": (40, 28, 18), "m": (90, 64, 38),
     "l": (120, 88, 52), "h": (156, 118, 72), "u": (92, 134, 52), "c": (200, 90, 70), "C": (232, 200, 170)},
    {"w": (52, 40, 44), "W": (74, 58, 60), "q": (32, 24, 28), "d": (20, 12, 16), "m": (56, 40, 42),
     "l": (78, 58, 58), "h": (104, 84, 82), "u": (214, 212, 196), "c": (214, 212, 196), "C": (164, 162, 146)},
    {"w": (110, 120, 130), "W": (150, 166, 180), "q": (70, 78, 90), "r": (180, 200, 220), "R": (230, 240, 250),
     "e": (90, 110, 130), "d": (18, 30, 48), "m": (70, 96, 128), "l": (110, 140, 172), "h": (176, 206, 232),
     "u": (224, 240, 252), "c": (150, 200, 240), "C": (220, 240, 255)},
    {"w": (40, 30, 26), "W": (60, 46, 40), "q": (24, 18, 16), "r": (110, 90, 86), "R": (160, 120, 100),
     "e": (50, 40, 40), "d": (14, 10, 10), "m": (40, 32, 32), "l": (60, 48, 46), "h": (90, 74, 70),
     "u": (255, 140, 50), "c": (255, 140, 50), "C": (255, 220, 120)},
    {"w": (150, 110, 60), "W": (190, 150, 90), "q": (100, 70, 40), "r": (200, 160, 70), "R": (240, 210, 120),
     "e": (110, 80, 40), "d": (40, 20, 14), "m": (110, 62, 30), "l": (146, 88, 44), "h": (190, 128, 64),
     "u": (228, 184, 82), "c": (236, 196, 84), "C": (255, 236, 150)},
]


def cell(rows, palette):
    image = Image.new("RGBA", (W, H), CLEAR)
    for y, row in enumerate(rows):
        assert len(row) == W, row
        for x, key in enumerate(row):
            if key in palette:
                image.putpixel((x, y), palette[key] + (255,))
    return image


def turn(PALETTE):
    """A quarter circle of rail around the bottom-left corner, entering at the left edge and
    leaving at the bottom edge, with the sleepers fanning out between the two rails."""
    import math

    image = Image.new("RGBA", (W, H), CLEAR)
    corner = (0.0, float(H))
    # The straight rail has its rails on rows 4-5 and 10-11, the same distance from the middle
    # either way, so it lines up with the turn whichever way it is turned.
    outer, inner = 11.0, 5.0
    for degrees in (16, 45, 74):
        angle = math.radians(degrees)
        for y in range(H):
            for x in range(W):
                dx, dy = x + 0.5 - corner[0], corner[1] - (y + 0.5)
                radius = math.hypot(dx, dy)
                # Distance from the sleeper's centre line, a ray out of the corner.
                across = abs(dx * math.sin(angle) - dy * math.cos(angle))
                if 3.0 <= radius <= 13.0 and across <= 1.0:
                    image.putpixel((x, y), PALETTE["w"] + (255,))
    for y in range(H):
        for x in range(W):
            d = math.hypot(x + 0.5 - corner[0], y + 0.5 - corner[1])
            for radius in (outer, inner):
                if radius <= d < radius + 1.0:
                    image.putpixel((x, y), PALETTE["R"] + (255,))
                elif radius - 1.0 <= d < radius:
                    image.putpixel((x, y), PALETTE["r"] + (255,))
                elif radius - 1.6 <= d < radius - 1.0:
                    image.putpixel((x, y), PALETTE["e"] + (255,))
    return image


def main():
    sheet = Image.new("RGBA", (5 * W, H * len(LOOKS)), CLEAR)
    for row, look in enumerate(LOOKS):
        palette = {**PALETTE, **look}
        # The buffer stop's timber and plate follow the sleepers and the rails.
        palette.update({"b": palette["w"], "B": palette["W"], "p": palette["e"], "P": palette["r"]})
        for column, rows in enumerate([RAIL, BUFFER, CART, CART_ROLLING]):
            sheet.paste(cell(rows, palette), (column * W, row * H))
        sheet.paste(turn(palette), (4 * W, row * H))
    sheet.save(OUT / "rails.tga")


if __name__ == "__main__":
    main()
