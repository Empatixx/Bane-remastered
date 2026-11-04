"""The shop's furniture, drawn at the tiles' pixel scale (one pixel is four world units):

- shoptable.tga: a plain wooden table under a green cloth, the goods in the middle (24x16,
  drawn coarser: shown six times over).
- bookshelf.tga: the lobby's bookshelf in the same style, books, scrolls, a candle and a skull
  (48x43).
- shop_counter.tga: the keeper's plain wooden counter, a few coins and a candle on it (80x28).
- shop_shelf.tga: two tall shelves of bottles, jars and scrolls standing against the wall
  (32x48 each, one above the other).
- shop_goods.tga: two piles of stock for the corners, crates with a coin sack and a barrel with
  rolled carpets (32x32 each, one above the other).
- shop_lamp.tga: a brass floor lamp, four frames of its flame flickering (16x32 each).
- slotmachine.tga: the slot machine, 48x48 frames in four rows of eight: spinning (the lever
  pulled, the reels blurring, the lights chasing), winning (three of a kind, the lights
  flashing, coins spilling into the tray), losing (mismatched reels, the lights dimmed) and
  idle (the lights running slowly along the marquee). Kept simple: a coin on the marquee, five
  bulbs, the reels, a tray and the lever.

Hand-picked palettes: worn dark wood, a green cloth, crimson velvet and old gold.
"""

import sys
from pathlib import Path

from PIL import Image

OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "assets/textures/ingame"
CLEAR = (0, 0, 0, 0)

# Wood.
W0 = (34, 20, 16)
W1 = (66, 38, 26)
W2 = (98, 60, 38)
W3 = (132, 86, 52)
W4 = (164, 114, 70)
# Crimson velvet.
V0 = (44, 10, 18)
V1 = (86, 18, 30)
V2 = (124, 30, 42)
V3 = (162, 48, 56)
V4 = (196, 78, 80)
# Old gold.
G0 = (92, 62, 20)
G1 = (150, 106, 34)
G2 = (204, 160, 54)
G3 = (244, 214, 110)
G4 = (255, 244, 190)
# Iron and glass.
I0 = (22, 22, 28)
I1 = (52, 52, 62)
I2 = (96, 98, 112)
I3 = (150, 154, 168)
GLASS = (210, 226, 236)
PAPER = (222, 206, 166)
PAPER_D = (170, 150, 112)
# Potions.
RED = (210, 50, 60)
RED_D = (130, 24, 40)
BLUE = (70, 140, 220)
BLUE_D = (34, 70, 140)
GREEN = (90, 190, 90)
GREEN_D = (40, 110, 56)
PURPLE = (160, 90, 200)
PURPLE_D = (90, 44, 120)
FLAME = (255, 200, 90)
FLAME_H = (255, 246, 200)
FLAME_D = (230, 110, 40)
SHADOW = (0, 0, 0)
# The tables' green cloth.
C0 = (26, 46, 34)
C1 = (44, 76, 52)
C2 = (62, 104, 70)
C3 = (92, 138, 96)


class Canvas:
    def __init__(self, width, height):
        self.image = Image.new("RGBA", (width, height), CLEAR)

    def put(self, x, y, colour, alpha=255):
        if 0 <= x < self.image.width and 0 <= y < self.image.height:
            self.image.putpixel((x, y), colour + (alpha,))

    def rect(self, x0, y0, x1, y1, colour, alpha=255):
        for y in range(y0, y1 + 1):
            for x in range(x0, x1 + 1):
                self.put(x, y, colour, alpha)

    def hline(self, x0, x1, y, colour):
        self.rect(x0, y, x1, y, colour)

    def vline(self, x, y0, y1, colour):
        self.rect(x, y0, x, y1, colour)

    def shadow(self, x0, y0, x1, y1, alpha):
        """Darken the floor showing round a thing, never the thing itself."""
        for y in range(y0, y1 + 1):
            for x in range(x0, x1 + 1):
                if 0 <= x < self.image.width and 0 <= y < self.image.height:
                    if self.image.getpixel((x, y))[3] == 0:
                        self.image.putpixel((x, y), SHADOW + (alpha,))

    def pattern(self, x, y, rows, palette):
        for dy, row in enumerate(rows):
            for dx, key in enumerate(row):
                if key in palette:
                    self.put(x + dx, y + dy, palette[key])


def sheet(cells, columns):
    width, height = cells[0].image.size
    rows = (len(cells) + columns - 1) // columns
    out = Image.new("RGBA", (width * columns, height * rows), CLEAR)
    for index, cell in enumerate(cells):
        out.paste(cell.image, ((index % columns) * width, (index // columns) * height))
    return out


# --- The display table -----------------------------------------------------------------------


def shop_table():
    """A plain wooden table under a green cloth, the goods in the middle. Drawn coarser than the
    rest (24x16, shown six times over), so the goods on it stand out."""
    c = Canvas(24, 16)
    c.shadow(0, 14, 23, 15, 110)
    for x in (2, 20):
        c.rect(x, 12, x + 1, 14, W2)
        c.vline(x + 1, 12, 14, W1)
    # The cloth over the top, a lighter edge where it turns down.
    c.rect(0, 1, 23, 6, C2)
    c.hline(0, 23, 0, C0)
    c.hline(1, 22, 1, C3)
    c.vline(0, 1, 6, C1)
    c.vline(23, 1, 6, C1)
    # The drape down the front, a hem and two soft folds.
    c.rect(0, 7, 23, 11, C1)
    c.hline(0, 23, 7, C3)
    c.hline(0, 23, 11, C0)
    for x in (7, 16):
        c.vline(x, 8, 10, C0)
        c.vline(x + 1, 8, 10, C2)
    return c


# --- The counter -----------------------------------------------------------------------------


def counter():
    """The keeper's counter: a plain wooden top, three panels in front, a candle and a few coins."""
    c = Canvas(80, 28)
    c.shadow(1, 25, 78, 27, 120)
    c.rect(0, 4, 79, 11, W3)
    c.hline(0, 79, 4, W4)
    c.hline(0, 79, 8, W2)
    c.hline(0, 79, 11, W1)
    c.vline(0, 4, 11, W1)
    c.vline(79, 4, 11, W1)
    c.rect(0, 12, 79, 24, W2)
    c.hline(0, 79, 12, W0)
    c.hline(0, 79, 24, W0)
    for x0 in (2, 28, 54):
        x1 = x0 + 23
        c.rect(x0, 14, x1, 22, W1)
        c.rect(x0 + 1, 15, x1 - 1, 21, W2)
        c.hline(x0 + 1, x1 - 1, 15, W3)
    c.vline(0, 12, 24, W0)
    c.vline(79, 12, 24, W0)
    # A few coins and a candle.
    for x, height in ((14, 2), (18, 3)):
        for step in range(height):
            c.hline(x, x + 2, 9 - step, G2 if step % 2 else G1)
        c.put(x + 1, 9 - height + 1, G3)
    c.rect(66, 3, 68, 9, PAPER)
    c.vline(68, 3, 9, PAPER_D)
    c.put(67, 2, FLAME)
    c.put(67, 1, FLAME_H)
    c.hline(64, 70, 10, G1)
    return c


# --- Shelves ---------------------------------------------------------------------------------


def bottle(c, x, y, colour, dark, tall=True):
    """A potion bottle standing with its base on row y."""
    height = 6 if tall else 4
    c.rect(x, y - height + 2, x + 2, y, colour)
    c.vline(x + 2, y - height + 2, y, dark)
    c.put(x, y - height + 2, GLASS)
    c.put(x + 1, y - height + 1, GLASS)
    c.put(x + 1, y - height, W2)


def jar(c, x, y, colour):
    c.rect(x, y - 3, x + 3, y, colour)
    c.hline(x, x + 3, y - 4, PAPER_D)
    c.put(x, y - 3, GLASS)


def scroll(c, x, y):
    c.rect(x, y - 2, x + 5, y, PAPER)
    c.hline(x, x + 5, y, PAPER_D)
    c.vline(x, y - 2, y, PAPER_D)
    c.put(x + 3, y - 1, RED)


def books(c, x, y, colours):
    for index, colour in enumerate(colours):
        tall = 5 if index % 2 else 4
        c.rect(x + index * 2, y - tall + 1, x + index * 2 + 1, y, colour)
        c.put(x + index * 2, y - tall + 1, G2)


def shelf(stock):
    c = Canvas(32, 48)
    c.shadow(1, 45, 30, 47, 120)
    # The frame and back.
    c.rect(1, 2, 30, 44, W1)
    c.rect(3, 4, 28, 43, W0)
    c.rect(1, 1, 30, 3, W3)
    c.hline(0, 31, 1, W4)
    c.hline(0, 31, 0, W2)
    c.vline(1, 2, 44, W2)
    c.vline(2, 2, 44, W3)
    c.vline(30, 2, 44, W0)
    c.vline(29, 2, 44, W2)
    # The boards, each lit on its top edge.
    boards = (14, 26, 38)
    for y in boards:
        c.hline(3, 28, y, W3)
        c.hline(3, 28, y + 1, W2)
        c.hline(3, 28, y + 2, W1)
    c.hline(1, 30, 44, W0)
    c.rect(2, 44, 4, 46, W1)
    c.rect(27, 44, 29, 46, W1)
    for shelf_index, items in enumerate(stock):
        y = boards[shelf_index] - 1
        for draw in items:
            draw(c, y)
    # A small gold trim on the top.
    for x in (6, 15, 16, 25):
        c.put(x, 2, G2)
    return c


def left_shelf():
    return shelf([
        [lambda c, y: bottle(c, 5, y, RED, RED_D), lambda c, y: bottle(c, 9, y, RED, RED_D, False),
         lambda c, y: bottle(c, 14, y, BLUE, BLUE_D), lambda c, y: jar(c, 20, y, GREEN_D),
         lambda c, y: bottle(c, 25, y, PURPLE, PURPLE_D, False)],
        [lambda c, y: books(c, 5, y, [V2, BLUE_D, GREEN_D, W3, V1]), lambda c, y: scroll(c, 17, y),
         lambda c, y: jar(c, 24, y, PURPLE_D)],
        [lambda c, y: bottle(c, 5, y, GREEN, GREEN_D), lambda c, y: bottle(c, 9, y, GREEN, GREEN_D, False),
         lambda c, y: scroll(c, 14, y), lambda c, y: bottle(c, 22, y, BLUE, BLUE_D),
         lambda c, y: bottle(c, 25, y, RED, RED_D, False)],
    ])


def right_shelf():
    return shelf([
        [lambda c, y: jar(c, 5, y, RED_D), lambda c, y: bottle(c, 11, y, PURPLE, PURPLE_D),
         lambda c, y: bottle(c, 15, y, PURPLE, PURPLE_D, False), lambda c, y: bottle(c, 21, y, GREEN, GREEN_D),
         lambda c, y: bottle(c, 25, y, BLUE, BLUE_D, False)],
        [lambda c, y: scroll(c, 5, y), lambda c, y: bottle(c, 13, y, RED, RED_D),
         lambda c, y: books(c, 18, y, [W3, V2, PURPLE_D, BLUE_D, G1])],
        [lambda c, y: jar(c, 5, y, BLUE_D), lambda c, y: jar(c, 10, y, GREEN_D),
         lambda c, y: bottle(c, 16, y, RED, RED_D), lambda c, y: bottle(c, 20, y, BLUE, BLUE_D, False),
         lambda c, y: bottle(c, 24, y, GREEN, GREEN_D)],
    ])


def candle(c, x, y):
    c.rect(x, y - 3, x + 1, y, PAPER)
    c.put(x + 1, y - 3, PAPER_D)
    c.put(x, y - 4, FLAME)
    c.put(x, y - 5, FLAME_H)
    c.hline(x - 1, x + 2, y, G1)


def skull(c, x, y):
    c.pattern(x, y - 3, [".ppp.", "pkpkp", ".ppp.", ".p.p."], {"p": PAPER, "k": W0})


def globe(c, x, y):
    c.pattern(x, y - 6, [".bbb.", "bgbbb", "bbgbb", ".bbb.", "..d..", ".ddd."], {"b": BLUE, "g": GREEN, "d": G1})


def bookshelf():
    """The lobby's bookshelf, in the shop shelves' style: books, scrolls, a candle and a skull."""
    c = Canvas(48, 43)
    c.shadow(1, 41, 46, 42, 120)
    c.rect(1, 2, 46, 40, W1)
    c.rect(3, 4, 44, 39, W0)
    c.rect(1, 1, 46, 3, W3)
    c.hline(0, 47, 1, W4)
    c.hline(0, 47, 0, W2)
    c.vline(1, 2, 40, W2)
    c.vline(2, 2, 40, W3)
    c.vline(46, 2, 40, W0)
    c.vline(45, 2, 40, W2)
    boards = (14, 26, 38)
    for y in boards:
        c.hline(3, 44, y, W3)
        c.hline(3, 44, y + 1, W2)
    c.hline(1, 46, 40, W0)
    c.rect(2, 40, 4, 42, W1)
    c.rect(43, 40, 45, 42, W1)
    for x in (8, 23, 24, 39):
        c.put(x, 2, G2)
    top, middle, bottom = (y - 1 for y in boards)
    books(c, 5, top, [V2, BLUE_D, G1, GREEN_D, V1, PURPLE_D])
    candle(c, 20, top)
    scroll(c, 25, top)
    books(c, 33, top, [W3, V2, BLUE_D, GREEN_D, V3])
    skull(c, 5, middle)
    books(c, 12, middle, [PURPLE_D, G1, V2, BLUE_D, W3, GREEN_D, V1, BLUE])
    bottle(c, 31, middle, GREEN, GREEN_D)
    jar(c, 37, middle, RED_D)
    scroll(c, 4, bottom)
    books(c, 12, bottom, [GREEN_D, V2, BLUE_D])
    globe(c, 20, bottom)
    books(c, 28, bottom, [V1, G1, PURPLE_D, W3, BLUE_D, V2, GREEN_D])
    return c


# --- Piles of stock --------------------------------------------------------------------------


def crate(c, x0, y0, x1, y1):
    """A wooden crate, its top showing above its front."""
    top = y0 + (y1 - y0) // 3
    c.rect(x0, y0, x1, top, W3)
    c.hline(x0, x1, y0, W4)
    c.rect(x0, top + 1, x1, y1, W2)
    for y in range(top + 3, y1, 3):
        c.hline(x0 + 1, x1 - 1, y, W1)
    c.vline(x0, y0, y1, W0)
    c.vline(x1, y0, y1, W0)
    c.hline(x0, x1, y1, W0)
    c.hline(x0, x1, top, W1)
    c.vline(x0 + 1, top + 1, y1 - 1, W3)
    c.vline(x1 - 1, top + 1, y1 - 1, W3)
    for x, y in ((x0 + 1, top + 1), (x1 - 1, top + 1), (x0 + 1, y1 - 1), (x1 - 1, y1 - 1)):
        c.put(x, y, I2)


def sack(c, x0, y0):
    """A sack of coins, tied at the neck, a few coins spilt in front."""
    c.pattern(x0, y0, [
        "....hh....",
        "...dggd...",
        "....dd....",
        "..LLLLLL..",
        ".LLllllLd.",
        "LLllllllLd",
        "LllllllLLd",
        "LlllllLLdd",
        ".LLLLLLdd.",
        "..dddddd..",
    ], {"h": G3, "g": G2, "d": (110, 86, 54), "L": (196, 170, 120), "l": (170, 144, 98)})
    for x, y in ((x0 + 11, y0 + 9), (x0 - 2, y0 + 9), (x0 + 12, y0 + 8)):
        c.put(x, y, G2)
        c.put(x + 1, y, G1)


def barrel(c, x0, y0):
    c.pattern(x0, y0, [
        ".dddddddd.",
        "dlhhhhhhld",
        "dmllllllmd",
        "iiiiiiiiii",
        "dmlllllmmd",
        "dmlllllmmd",
        "dmlllllmmd",
        "iiiiiiiiii",
        "dmllllmmmd",
        ".dddddddd.",
    ], {"d": W0, "l": W3, "h": W4, "m": W2, "i": I2})


def rolled_carpets(c, x0, y0):
    for index, (light, dark) in enumerate(((V3, V1), (BLUE, BLUE_D), (G2, G0))):
        x = x0 + index * 4
        c.rect(x, y0, x + 3, y0 + 14, light)
        c.vline(x + 3, y0, y0 + 14, dark)
        c.rect(x, y0, x + 3, y0, G3 if index != 2 else V3)
        c.put(x + 1, y0 + 5, dark)
        c.put(x + 2, y0 + 10, dark)


def goods_crates():
    c = Canvas(32, 32)
    crate(c, 2, 14, 17, 29)
    crate(c, 15, 18, 29, 30)
    crate(c, 5, 3, 16, 14)
    sack(c, 18, 8)
    c.shadow(1, 30, 30, 31, 110)
    return c


def goods_barrel():
    c = Canvas(32, 32)
    rolled_carpets(c, 3, 9)
    barrel(c, 15, 15)
    crate(c, 2, 22, 13, 30)
    c.shadow(1, 30, 30, 31, 110)
    c.shadow(25, 24, 26, 30, 90)
    return c


# --- The lamp --------------------------------------------------------------------------------


def lamp(frame):
    c = Canvas(16, 32)
    c.shadow(4, 29, 11, 31, 110)
    # Foot, stem and the dish.
    c.pattern(4, 26, ["..gggg..", ".gGGGGg.", "gGGGGGGg", "dddddddd"], {"g": G1, "G": G2, "d": G0})
    c.vline(7, 10, 25, G1)
    c.vline(8, 10, 25, G0)
    c.put(7, 14, G3)
    c.put(7, 20, G3)
    c.rect(6, 17, 9, 17, G1)
    c.pattern(3, 8, ["gGGGGGGGGg", ".gGGGGGGg.", "..dddddd.."], {"g": G1, "G": G2, "d": G0})
    c.rect(6, 6, 9, 7, PAPER)
    c.vline(9, 6, 7, PAPER_D)
    # The flame, a little different each frame.
    flames = [
        ["..f..", ".fFf.", ".fHf.", "..o.."],
        ["...f.", "..Ff.", ".fHf.", "..o.."],
        [".....", "..f..", ".fHf.", ".oFo."],
        [".f...", ".fF..", ".fHf.", "..o.."],
    ]
    c.pattern(5, 1, flames[frame], {"f": FLAME, "F": FLAME, "H": FLAME_H, "o": FLAME_D})
    return c


# --- The slot machine ------------------------------------------------------------------------

SEVEN = ["rrrrr", "...rR", "..rR.", ".rR..", ".rR..", ".rR.."]
CHERRY = ["...g.", "..g.g", ".g..g", "rr.rr", "rRrrR", ".r..r"]
BELL = ["..y..", ".yyy.", ".yYy.", "yyYyy", "yyyyy", "..o.."]
BAR = [".....", "kkkkk", "kWWWk", "kkkkk", ".....", "....."]
GEM = [".bbb.", "bBBBb", "bBbBb", ".bBb.", "..b..", "....."]
SYMBOLS = [SEVEN, CHERRY, BELL, BAR, GEM]
SYMBOL_PALETTE = {
    "r": (210, 34, 46), "R": (120, 14, 30), "g": (60, 150, 60), "y": G2, "Y": G4, "o": G0,
    "k": (30, 30, 36), "W": (240, 240, 240), "b": (90, 170, 240), "B": (40, 90, 190),
}
REEL = (238, 232, 216)
REEL_D = (196, 186, 166)
# Where each reel's window is.
REEL_X = (13, 20, 27)
REEL_TOP = 17
REEL_HEIGHT = 10
# Five bulbs along the top of the marquee.
BULBS = [(13 + 5 * i, 5) for i in range(5)]
BULB_ON = (255, 236, 150)
BULB_ON_H = (255, 255, 230)
BULB_OFF = (120, 70, 40)


def machine_body(c, lever_down):
    c.shadow(5, 45, 42, 47, 120)
    # The cabinet: crimson with gold edges, a rounded marquee on top.
    c.rect(8, 3, 39, 15, V1)
    c.hline(10, 37, 2, G1)
    c.hline(9, 38, 3, G2)
    c.rect(9, 4, 38, 15, V2)
    c.rect(11, 6, 36, 13, V0)
    # The sign in the marquee: one gold coin.
    c.pattern(21, 7, [".ggg.", "gGcGg", "gcGcg", "gGcGg", ".ggg."], {"g": G1, "G": G2, "c": G3})
    c.vline(8, 4, 44, G1)
    c.vline(39, 4, 44, G0)
    c.rect(9, 16, 38, 44, V2)
    c.vline(9, 16, 44, V3)
    c.vline(38, 16, 44, V1)
    # The reel window, framed in gold.
    c.rect(11, 15, 34, 28, G1)
    c.hline(11, 34, 15, G2)
    c.rect(12, 16, 33, 27, I0)
    # The payout tray.
    c.rect(13, 35, 34, 40, G0)
    c.rect(14, 36, 33, 39, I0)
    c.hline(13, 34, 35, G2)
    c.hline(13, 34, 40, G1)
    # The plinth.
    c.rect(7, 42, 40, 45, W1)
    c.hline(7, 40, 42, W3)
    c.hline(7, 40, 45, W0)
    # The lever on the right side, its knob up or pulled down.
    c.rect(39, 22, 41, 27, I2)
    c.put(41, 22, I3)
    if lever_down:
        c.vline(42, 24, 33, I3)
        c.vline(43, 24, 33, I1)
        knob_y = 33
    else:
        c.vline(42, 6, 24, I3)
        c.vline(43, 6, 24, I1)
        knob_y = 4
    c.pattern(41, knob_y, [".rr.", "rRrr", "rrrd", ".dd."], {"r": (220, 40, 50), "R": (255, 150, 150), "d": (120, 16, 30)})


def reel(c, index, symbols, offset=0, blur=False):
    """One reel: the symbols on its strip, scrolled down by `offset` pixels."""
    x = REEL_X[index]
    c.rect(x, REEL_TOP, x + 5, REEL_TOP + REEL_HEIGHT - 1, REEL)
    c.vline(x + 5, REEL_TOP, REEL_TOP + REEL_HEIGHT - 1, REEL_D)
    strip = []
    for symbol in symbols:
        strip += symbol + ["....."] * 2
    total = len(strip)
    for row in range(REEL_HEIGHT - 2):
        line = strip[(row - offset - 1) % total]
        for dx, key in enumerate(line):
            if key in SYMBOL_PALETTE:
                colour = SYMBOL_PALETTE[key]
                if blur:
                    colour = tuple((channel + 2 * REEL[i]) // 3 for i, channel in enumerate(colour))
                c.put(x + dx, REEL_TOP + 1 + row, colour)
    if blur:
        for row in range(0, REEL_HEIGHT, 3):
            c.hline(x, x + 4, REEL_TOP + (row + offset) % REEL_HEIGHT, REEL_D)
    # The glass: a shine over the top of each reel and a shade at the bottom.
    c.hline(x, x + 5, REEL_TOP, (255, 255, 255))
    c.hline(x, x + 5, REEL_TOP + REEL_HEIGHT - 1, REEL_D)


def bulbs(c, lit):
    for index, (x, y) in enumerate(BULBS):
        if lit(index):
            c.put(x, y, BULB_ON_H)
            c.put(x + 1, y, BULB_ON)
        else:
            c.put(x, y, BULB_OFF)
            c.put(x + 1, y, BULB_OFF)


def coins_in_tray(c, count, frame):
    spots = [(16, 38), (20, 37), (24, 38), (28, 37), (31, 38), (18, 36), (26, 36), (22, 39)]
    for x, y in spots[:count]:
        c.put(x, y, G3)
        c.put(x + 1, y, G2)
    # A glint that moves along the tray.
    gx = 15 + (frame * 3) % 18
    c.put(gx, 37, G4)


def spin_frame(frame):
    c = Canvas(48, 48)
    machine_body(c, lever_down=True)
    for index in range(3):
        # The reels run at different speeds, so they never line up while spinning.
        reel(c, index, [SYMBOLS[(index + k) % 5] for k in range(5)], offset=frame * (5 + 2 * index), blur=True)
    bulbs(c, lambda i: (i + frame) % 4 == 0)
    return c


def win_frame(frame):
    c = Canvas(48, 48)
    machine_body(c, lever_down=False)
    for index in range(3):
        reel(c, index, [SEVEN, CHERRY])
    # Every light flashes, then the bulbs alternate, then all stay lit.
    if frame < 4:
        lit = (lambda i: True) if frame % 2 == 0 else (lambda i: False)
    elif frame < 7:
        lit = lambda i: (i + frame) % 2 == 0
    else:
        lit = lambda i: True
    bulbs(c, lit)
    coins_in_tray(c, min(8, 2 + frame), frame)
    return c


def lose_frame(frame):
    c = Canvas(48, 48)
    machine_body(c, lever_down=False)
    for index, symbol in enumerate((CHERRY, BAR, GEM)):
        reel(c, index, [symbol, SEVEN])
    bulbs(c, lambda i: (i + frame) % 6 == 0)
    return c


def idle_frame(frame):
    c = Canvas(48, 48)
    machine_body(c, lever_down=False)
    for index, symbol in enumerate((SEVEN, BELL, CHERRY)):
        reel(c, index, [symbol, GEM])
    bulbs(c, lambda i: (i + frame) % 4 in (0, 1))
    return c


def main():
    single = [
        ("shoptable.tga", shop_table()),
        ("bookshelf.tga", bookshelf()),
        ("shop_counter.tga", counter()),
    ]
    for name, canvas in single:
        canvas.image.save(OUT / name)
    sheet([left_shelf(), right_shelf()], 1).save(OUT / "shop_shelf.tga")
    sheet([goods_crates(), goods_barrel()], 1).save(OUT / "shop_goods.tga")
    sheet([lamp(frame) for frame in range(4)], 4).save(OUT / "shop_lamp.tga")
    rows = (
        [spin_frame(f) for f in range(8)]
        + [win_frame(f) for f in range(8)]
        + [lose_frame(f) for f in range(8)]
        + [idle_frame(f) for f in range(8)]
    )
    sheet(rows, 8).save(OUT / "slotmachine.tga")


if __name__ == "__main__":
    main()
