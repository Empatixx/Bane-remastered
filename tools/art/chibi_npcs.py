"""The other people of Bane as chibis, in the hero's own proportions (tools/art/chibi_elian.py):
20x26 frames, a big head over a small body, shown three times over like the hero. Hand-drawn
parts are stamped per frame at whole-pixel offsets.

- PRIEST and PILGRIM (the order and the people of Velmor; Aldric is a priest), 4x4 frames:
  0 standing (the hem stirring), 1 running to the side (faces right), 2 praying (hands raised,
  the second frame with a glint), 3 asleep on his knees (two frames, sinking a little).
- KNEELING, Elian at prayer seen from behind, 4x3 frames: 0 praying (breathing), 1 nodding
  off (the head dips, jerks up and drops), 2 asleep (breathing).
- SCIENTIST, 6x1: standing, breathing, a blink and a glint on his goggles.
- MERCHANT, the shopkeeper, 6x2: 0 standing (breathing, a blink), 1 rubbing his hands with a
  smile.
"""
import chibi_elian as hero
import chibi_run as side

W, H, GROUND = 20, 26, 25
OUTLINE = "#1e1014"


def blank():
    return [["."] * W for _ in range(H)]


def stamp(grid, rows, x0, y0):
    for dy, line in enumerate(rows):
        for dx, ch in enumerate(line):
            if ch != "." and 0 <= y0 + dy < H and 0 <= x0 + dx < W:
                grid[y0 + dy][x0 + dx] = ch


def text(grid):
    return ["".join(r) for r in grid]


def check(name, rows, width=None):
    widths = {len(r) for r in rows}
    assert len(widths) == 1, (name, widths)
    if width is not None:
        assert widths == {width}, (name, widths)
    return rows


# ---------------------------------------------------------------------------------------------
# Priests: a white hood and robe with gold trim, the face in the hood's shadow.

PRIEST_PAL = {
    "a": OUTLINE,
    "w": "#e2daca", "s": "#aaa094", "d": "#766c68",  # robe
    "g": "#deb652", "G": "#96702e",  # gold trim
    "k": "#3a2e36",  # inside the hood
    "f": "#d6a076", "F": "#a8744e", "e": "#1e1014",  # face in shadow, eyes
    "h": "#eec39a",  # hands
    "y": "#fff0aa",  # glint
    "z": "#5e3826", "Z": "#3e2418",  # boots
}
PILGRIM_PAL = dict(PRIEST_PAL, w="#967454", s="#70543e", d="#4e3a2e", g="#5c4a3a", G="#40322a")

HOOD = check("hood", [
    "....aaaaa....",
    "..aawwwwwaa..",
    ".awwwwwwwwsa.",
    "awwwwwwwwwwsa",
    "awwwkkkkkwwsa",
    "awwkkfffkkwsa",
    "awwkfffffkssa",
    "awwkfefefkssa",
    "awwkfffffkssa",
    "aswkkFFFkksda",
    ".asswwwwwssa.",
    "..aaaaaaaaa..",
], 13)
# Bowed in sleep: the hood tips forward over the face.
HOOD_BOWED = check("hood bowed", [
    "....aaaaa....",
    "..aawwwwwaa..",
    ".awwwwwwwwsa.",
    "awwwwwwwwwwsa",
    "awwwwwwwwwwsa",
    "awwwwwwwwwssa",
    "awwwkkkkkwssa",
    "awwkkFFFkkssa",
    "aswkkkkkkksda",
    ".asswwwwwssa.",
    "..aaaaaaaaa..",
], 13)
HOOD_SIDE = check("hood side", [
    "....aaaaa....",
    "..aawwwwwaa..",
    ".awwwwwwwwwa.",
    "awwwwwwwwwwwa",
    "awwwwwwwwkkka",
    "aswwwwwwkkffa",
    "aswwwwwwkfffa",
    "aswwwwwwkffea",
    "aswwwwwwkfffa",
    "asswwwwwkkFFa",
    ".asswwwwwkka.",
    "..aaaaaaaaa..",
], 13)

ROBE = check("robe", [
    ".aswwgwwwsa.",
    "awwwwgwwwwsa",
    "awwwwgwwwwsa",
    "awwwwgwwwssa",
    "awswwgwwwssa",
    "awswwgwwwsda",
    "aggggGgggggа".replace("а", "a"),
    ".aaaaaaaaaa.",
], 12)
# The hem stirs: the gold band and the bottom outline shift a pixel left and right.
HEMS = [
    ["aggggGggggga", ".aaaaaaaaaa."],
    ["aggggGggggga", "..aaaaaaaaa."],
    ["aggggGggggga", ".aaaaaaaaaa."],
    ["aggggGggggga", ".aaaaaaaaa.."],
]
# Running: a shorter robe swinging over two boots.
ROBE_SIDE = check("robe side", [
    ".aswwwwwwsa.",
    "awwwwwwwwwsa",
    "awwwwwwwwwsa",
    "awswwwwwwssa",
    "aGgggggggGga",
    ".aaaaaaaaaa.",
], 12)
SLEEVE_SIDE = check("sleeve", [".aaaa.", "awwwha", "awwsha", ".aaaa."], 6)
HANDS_UP = check("hands up", [".aa.", "ahha", "ahha", "awwa", "awsa", ".aa."], 4)
KNEEL_ROBE = check("kneeling robe", [
    ".aswwgwwwsa.",
    "awwwwgwwwwsa",
    "awwwwgwwwssa",
    "awswhhhwwsda",
    "aggggGgggggа".replace("а", "a"),
    ".aaaaaaaaaa.",
], 12)


def robed(head, robe, lift=0, drop=0, hem=None):
    """A robed figure standing on the ground: the robe's bottom on GROUND, the head on the robe."""
    grid = blank()
    robe = list(robe)
    if hem is not None:
        robe[-2:] = hem
    robe_top = GROUND - len(robe) + 1 + drop
    stamp(grid, robe, 4, robe_top)
    stamp(grid, head, 3, robe_top - len(head) + 1 + lift)
    return grid


def priest_rows():
    stand = []
    for i, hem in enumerate(HEMS):
        lift = -1 if i in (1, 2) else 0
        grid = robed(HOOD, ROBE, hem=hem)
        if lift:
            # breathing: the head and shoulders rise a pixel, the hem stays on the ground
            grid = robed(HOOD, ROBE[:1] + ROBE, hem=hem)
            grid = [row for row in grid]
        stand.append(text(grid))
    run = []
    for legs, bob in [("stride", 1), ("pass", 0), ("stride2", 1), ("pass2", 0)]:
        grid = blank()
        side.feet(grid, legs, (7, 10), (side.BACK_BOOT, side.BOOT))
        robe_top = GROUND - side.FOOT_ROWS + 2 - len(ROBE_SIDE) + bob
        stamp(grid, ROBE_SIDE, 4, robe_top)
        stamp(grid, HOOD_SIDE, 4, robe_top - len(HOOD_SIDE) + 1)
        stamp(grid, SLEEVE_SIDE, 8 + (1 if legs.startswith("pass") else 0), robe_top + 1)
        run.append(text(grid))
    pray = []
    for glint in (False, True):
        grid = robed(HOOD, ROBE)
        top = GROUND - len(ROBE) + 1 - len(HOOD) + 1
        stamp(grid, HANDS_UP, 1, top + 6)
        stamp(grid, HANDS_UP, 15, top + 6)
        if glint:
            for x, y in ((2, top + 3), (17, top + 3), (1, top + 4), (18, top + 4)):
                grid[y][x] = "y"
        pray.append(text(grid))
    asleep = [text(robed(HOOD_BOWED, KNEEL_ROBE, lift=d)) for d in (0, 1)]
    blank_row = [text(blank())] * 2
    return [stand, run, pray + blank_row, asleep + blank_row]


# ---------------------------------------------------------------------------------------------
# Elian kneeling at prayer, from behind: the hero's back head and back body, his legs folded
# under him so only the soles of his boots show.

def kneeling_rows():
    soles = check("soles", [".aaa..aaa.", "aZZZaaZZZa", ".aaa..aaa."], 10)
    body = hero.BACK_BODY

    def kneel(head_drop=0, sink=0):
        grid = blank()
        body_top = GROUND - 2 - len(body) + 1 + sink
        stamp(grid, soles, 5, GROUND - 2)
        stamp(grid, body, 4, body_top)
        stamp(grid, hero.BACK_HEAD, 4, body_top - len(hero.BACK_HEAD) + 1 + head_drop)
        return text(grid)

    pray = [kneel(sink=s) for s in (0, 0, 1, 1)]
    nod = [kneel(head_drop=1), kneel(head_drop=2), kneel(head_drop=0), kneel(head_drop=3, sink=1)]
    asleep = [kneel(head_drop=3, sink=s) for s in (1, 1, 2, 2)]
    return [pray, nod, asleep]


KNEEL_PAL = dict(hero.PAL, Z="#3e2418")


# ---------------------------------------------------------------------------------------------
# The Scientist: wild grey hair, brass goggles pushed up, a white lab coat over a dark vest.

SCIENTIST_PAL = {
    "a": OUTLINE,
    "H": "#b8b4b0", "L": "#e4e0da", "S": "#7c7674",  # hair
    "o": "#c89a48", "O": "#7e5a2a", "q": "#7ed0c8", "y": "#f4fff8",  # goggles: brass, lens, glint
    "f": "#eec39a", "i": "#d6a076", "h": "#0c080c", "n": "#c47a64",  # skin, eye, nose
    "c": "#f2f0ea", "C": "#bcb8b4",  # lab coat
    "v": "#3e4a5a", "V": "#2a3240",  # vest
    "r": "#a63926",  # tie
    "w": "#3e4452", "x": "#2e323e",  # trousers
    "z": "#3a2a24", "A": "#5e463a",  # shoes
}
SCIENTIST_HEAD = check("scientist head", [
    "..a.aaaaa.a..",
    ".aLaHLLLHHaLa",
    "aLHHLLHHHHHHa",
    "aHOooooOoooOa",
    "aHoqqoOoqqoHa",
    "aSOooooOoooSa",
    "aSffffffffffa",
    "aSfhfffffhfSa",
    "aSfhfffffhfSa",
    ".aSffnfffffa.",
    "..aaffffffa..",
    "....aaaaaa...",
], 13)
SCIENTIST_BODY = check("scientist body", [
    "..accrrcca..",
    ".acvvrrvvca.",
    ".accvrrvcCa.",
    ".accvvvvcCa.",
    ".accCvvCcCa.",
    "..aaxaaxaa..",
], 12)


def scientist_rows():
    frames = []
    for i, bob in enumerate([0, 0, 0, 1, 1, 1]):
        grid = blank()
        side.feet(grid, "stand", hero.FRONT_FEET_X, (side.BACK_BOOT, side.BACK_BOOT))
        body_top = GROUND - side.FOOT_ROWS + 2 - len(SCIENTIST_BODY) + bob
        head = list(SCIENTIST_HEAD)
        if i == 4:  # blink
            head[7] = "aSfffffffffSa"
            head[8] = "aSfhhfffhhfSa"
        if i == 1:  # a glint runs across the goggles
            head[4] = "aHoyqoOoqqoHa"
        if i == 2:
            head[4] = "aHoqqoOoyqoHa"
        stamp(grid, SCIENTIST_BODY, 4, body_top)
        stamp(grid, head, 4, body_top - len(head) + 1)
        frames.append(text(grid))
    return [frames]


# ---------------------------------------------------------------------------------------------
# The merchant, drawn like the Scientist: a green hat with a red feather, a moustache, a
# burgundy waistcoat over a cream shirt and a coin pouch on his belt.

MERCHANT_PAL = {
    "a": OUTLINE,
    "G": "#3e6a4a", "g": "#5a8a5e", "b": "#c89a48", "r": "#c8463a", "R": "#8a2a24",  # hat
    "H": "#6e4a30", "f": "#eec39a", "i": "#d6a076", "h": "#0c080c", "m": "#5a3a26",  # face
    "v": "#8a3a32", "V": "#5e2420", "s": "#e8dcc0", "o": "#deb652",  # waistcoat, shirt
    "n": "#4a3226", "p": "#a07a46", "P": "#6e5030",  # belt, pouch
    "w": "#3e4452", "x": "#2e323e", "z": "#3a2a24", "A": "#5e463a",  # trousers, shoes
}
MERCHANT_HEAD = check("merchant head", [
    ".....aaaaa.....",
    "....agGGGGa....",
    "....aGGGGGar...",
    "....abbbbbarR..",
    "..aaaGGGGGaaa..",
    ".agGGGGGGGGGGa.",
    ".aaHHHHHHHHHaa.",
    "..aHffffffffHa.",
    "..aHfhffffhfHa.",
    "..aHfhffffhfHa.",
    "..aHffmmmmffHa.",
    "...afffffffa...",
    "....aaffffaa...",
    "......aaa......",
], 15)
MERCHANT_BLINK = {8: "..aHffffffffHa.", 9: "..aHhhffffhhHa."}
MERCHANT_SMILE = {11: "...affiiiffa..."}
MERCHANT_BODY = check("merchant body", [
    "..aavssvaa..",
    ".aVvvsovvVa.",
    ".aVvvsovvVa.",
    ".annnnnnnna.",
    ".aVvpPpvvVa.",
    "..aawaawaa..",
], 12)
MERCHANT_HANDS = [
    check("hands together", ["...aaaa...", "..affffa..", "...aaaa..."], 10),
    check("hands rubbed", ["...aaaa...", "..afiffa..", "...aaaa..."], 10),
]


def merchant(bob, head_edits=(), hands=None):
    grid = blank()
    side.feet(grid, "stand", hero.FRONT_FEET_X, (side.BACK_BOOT, side.BACK_BOOT))
    body_top = GROUND - side.FOOT_ROWS + 2 - len(MERCHANT_BODY) + bob
    head = list(MERCHANT_HEAD)
    for edits in head_edits:
        for y, line in edits.items():
            head[y] = line
    stamp(grid, MERCHANT_BODY, 4, body_top)
    if hands is not None:
        stamp(grid, MERCHANT_HANDS[hands], 5, body_top + 2)
    stamp(grid, head, 3, body_top - len(head) + 1)
    return text(grid)


def merchant_rows():
    stand = [merchant(b, [MERCHANT_BLINK] if i == 4 else []) for i, b in enumerate([0, 0, 0, 1, 1, 1])]
    rub = [merchant(b, [MERCHANT_SMILE] + ([MERCHANT_BLINK] if i == 3 else []), hands=i % 2)
           for i, b in enumerate([0, 1, 0, 1, 0, 1])]
    return [stand, rub]


SHEETS = {
    "priest": (PRIEST_PAL, priest_rows),
    "pilgrim": (PILGRIM_PAL, priest_rows),
    "elian": (KNEEL_PAL, kneeling_rows),
    "scientist": (SCIENTIST_PAL, scientist_rows),
    "shopkeeper": (MERCHANT_PAL, merchant_rows),
}
