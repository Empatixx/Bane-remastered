"""The hero, a little coarser than first drawn: the full-size frames kept in
tools/art/player_full.tga (16x38) brought down to 13x31, keeping his
palette and shape, for a look a little chunkier than now and nowhere near as coarse as the
8x19 and 11x25 tries. Each new pixel takes the colour most of the source pixels under it have
(the outline wins ties, so the silhouette stays closed). Shown at 32x76 on screen it is about
2.45x.

    uv run -q --with pillow python tools/make_player_softer.py [preview dir]
"""

import sys
from collections import Counter
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
SRC_W, SRC_H = 16, 38
W, H = 13, 31
COLUMNS, ROWS = 6, 4
OUTLINE_MAX = 60  # darkest colours count as outline


def shrink(frame):
    src = frame.load()
    out = Image.new("RGBA", (W, H), (0, 0, 0, 0))
    px = out.load()
    for y in range(H):
        for x in range(W):
            x0, x1 = x * SRC_W / W, (x + 1) * SRC_W / W
            y0, y1 = y * SRC_H / H, (y + 1) * SRC_H / H
            votes = Counter()
            for sy in range(int(y0), min(SRC_H, int(y1 + 0.999))):
                for sx in range(int(x0), min(SRC_W, int(x1 + 0.999))):
                    overlap = (min(x1, sx + 1) - max(x0, sx)) * (min(y1, sy + 1) - max(y0, sy))
                    if overlap > 0:
                        votes[src[sx, sy]] += overlap
            if not votes:
                continue
            colour, weight = votes.most_common(1)[0]
            solid = sum(w for c, w in votes.items() if c[3] > 0)
            empty = sum(w for c, w in votes.items() if c[3] == 0)
            if solid < empty * 0.8:
                continue
            if colour[3] == 0:
                colour = max(((c, w) for c, w in votes.items() if c[3] > 0), key=lambda cw: cw[1])[0]
            # A dark outline pixel with real weight wins, so edges stay closed.
            dark = [(c, w) for c, w in votes.items() if c[3] > 0 and sum(c[:3]) < OUTLINE_MAX * 3]
            if dark and max(w for _, w in dark) >= weight * 0.6:
                colour = max(dark, key=lambda cw: cw[1])[0]
            px[x, y] = colour
    return out


SKIN = (238, 195, 154, 255)
WHITE = (240, 236, 228, 255)
PUPIL = (12, 8, 12, 255)
BLUE = (120, 160, 214, 255)
EYE_COLOURS = {WHITE[:3], (0, 0, 0), (3, 3, 5), BLUE[:3], PUPIL[:3]}


def fix_eyes(small, frame):
    """The same eye in every frame: found by its blue in the source, put back at the matching
    spot as one fixed little block (white, pupil, blue), so the eyes move only with the head."""
    src = frame.load()
    px = small.load()
    anchors = [(round(x * W / SRC_W), round(y * H / SRC_H))
               for y in range(SRC_H) for x in range(SRC_W) if src[x, y][:3] == BLUE[:3]]
    if not anchors:
        return
    # Clear stray eye pixels the shrink left round where the eyes go.
    for ax, ay in anchors:
        for y in range(ay - 2, ay + 2):
            for x in range(ax - 2, ax + 2):
                if 0 <= x < W and 0 <= y < H and px[x, y][3] and px[x, y][:3] in EYE_COLOURS:
                    px[x, y] = SKIN
    for ax, ay in anchors:
        for (dx, dy), colour in (((-1, -1), WHITE), ((0, -1), PUPIL), ((-1, 0), PUPIL), ((0, 0), BLUE)):
            x, y = ax + dx, ay + dy
            if 0 <= x < W and 0 <= y < H:
                px[x, y] = colour


SCAR = (232, 150, 140)


def fix_face(small):
    """The scar (the cheek's pink pixel) comes out of the shrink as a red spot under the eye:
    skin there instead."""
    px = small.load()
    for y in range(H):
        for x in range(W):
            if px[x, y][3] and px[x, y][:3] == SCAR:
                px[x, y] = SKIN


# --- the body, drawn by hand ----------------------------------------------------------------
# The shrink mangles the arms, the bandolier and the legs differently in every frame, so from the
# shoulders down each frame is drawn again from fixed pieces in player.tga's own colours: the
# torso, two clean 2-px arms with a gloved hand at the bottom, the belt and two legs with boots.

BODY = {
    ".": None,
    "k": (30, 16, 20),  # outline
    "S": (141, 41, 28),  # shirt
    "s": (109, 24, 16),  # shirt, shade
    "x": (166, 57, 38),  # shirt, lit
    "A": (180, 69, 46),  # sleeve, lit
    "y": (161, 45, 28),  # sleeve
    "I": (132, 34, 20),  # sleeve, shade
    "P": (66, 42, 30),  # bandolier leather
    "w": (222, 182, 82),  # bandolier's brass rounds
    "r": (250, 216, 182),  # skin, lit
    "e": (238, 195, 154),  # skin
    "f": (214, 160, 118),  # skin, shade
    "n": (58, 40, 36),  # glove
    "v": (233, 183, 135),  # fingers
    "E": (84, 52, 34),  # belt
    "p": (58, 34, 24),  # belt, shade
    "J": (214, 178, 92),  # buckle
    "F": (40, 24, 18),  # belt, dark end
    "l": (62, 68, 82),  # trousers, lit
    "g": (46, 50, 62),  # trousers
    "j": (32, 34, 44),  # trousers, shade
    "z": (104, 64, 44),  # boot, lit
    "q": (50, 28, 22),  # boot
    "G": (172, 142, 66),  # backpack, lit
    "m": (138, 111, 48),  # backpack
    "K": (100, 78, 34),  # backpack, shade
}

# Rows from the shoulders (row 0) to the belt (row 8). The arms are drawn over these.
FRONT_TORSO = [
    "..kwPsssssk..",
    "...kPPSSSk...",
    "...kSwPSSk...",
    "...kSSPPSk...",
    "...kSSSwPk...",
    "...kxSSSPk...",
    "...kxSSSsk...",
    "...kxSSSsk...",
    "..kEppJppFk..",
]

BACK_TORSO = [
    "..kxmsssmsk..",
    "...kmGGGmk...",
    "...kSGmGPk...",
    "...kSmmmPk...",
    "...kSKKPSk...",
    "...kSSPSSk...",
    "...kxPSSsk...",
    "...kxSSSsk...",
    "..kEpppppFk..",
]

SIDE_TORSO = [
    "...kxsssssk..",
    ".kkPPPPPPPk..",
    "kGGSSSSSSsk..",
    "kGmSSSSSSsk..",
    "kmmSSSSSSsk..",
    "kmKSSSSSSsk..",
    "kKKSSSSSSsk..",
    ".kksSSSSSsk..",
    "...kEppppFk..",
]

ARM_ROWS = 6  # sleeve, three rows of forearm, glove, fingers


def put(px, x, y, key):
    if 0 <= x < W and 0 <= y < H:
        colour = BODY[key]
        px[x, y] = (*colour, 255) if colour else (0, 0, 0, 0)


def front_arm(px, top, side, short):
    """A straight arm hanging at the torso's side: a sleeve, skin, a glove and fingers, with the
    outline round it. `short` rows fewer when it swings forward (towards the viewer, or away)."""
    inner, outer = (3, 2) if side == "left" else (9, 10)
    edge = 1 if side == "left" else 11
    lit, shade = ("r", "e") if side == "left" else ("e", "f")
    rows = ARM_ROWS - short
    pieces = [("A", "y") if side == "left" else ("I", "y")]
    pieces += [(lit, shade)] * (rows - 3)
    pieces += [("n", "n"), ("v", "v") if side == "left" else ("v", "f")]
    for i, (a, b) in enumerate(pieces):
        y = top + 1 + i
        put(px, outer, y, a if side == "left" else b)
        put(px, inner, y, b if side == "left" else a)
        put(px, edge, y, "k")
    y = top + 1 + len(pieces)
    put(px, outer, y, "k")
    put(px, inner, y, "k")


def side_arm(px, top, swing):
    """The near arm in front of the body: down, forward (towards his facing) or back."""
    shift = {"down": (0, 0, 0), "fwd": (0, 1, 2), "back": (0, -1, -2)}[swing]
    cells = [(5, 2, "e", "e"), (5, 3, "e", "f")]
    cells += [(5 + shift[1], 4, "e", "f"), (5 + shift[2], 5, "n", "n"), (5 + shift[2], 6, "n", "v")]
    for x, y, a, b in cells:
        put(px, x, top + y, a)
        put(px, x + 1, top + y, b)
    put(px, 5 + shift[2], top + 7, "k")
    put(px, 6 + shift[2], top + 7, "k")


def front_legs(rows, lift):
    """Two legs seen from the front or back, `rows` tall: the hips, then each leg on its own
    down to its boot. `lift` is (side, rows): that boot is raised, an outline closing under it."""
    out = [list("..klgggggjk.."), list("..klgjjjgjk..")]
    out += [list("......k......") for _ in range(rows - 2)]
    side, up = lift
    for x0, leg in ((2, "left"), (7, "right")):
        u = up if leg == side else 0
        strip = ["kljk"] * (rows - 4 - u) + ["kzqk", "kqqk"] + [".kk."] * (u > 0) + ["...."] * (u - 1)
        for i, piece in enumerate(strip):
            out[2 + i][x0 : x0 + 4] = list(piece)
    # the middle outline runs only as far down as the crotch
    for i in range(3, rows):
        out[i][6] = "."
    return ["".join(r) for r in out]


SIDE_LEGS = {
    # near leg lit (l), the far one in shade (j); he faces right, boot toes that way
    "pass": [
        "....klggjk...",
        "....klggjk...",
        "....kjlggk...",
        "....kjlggk...",
        "....kjlggk...",
        "....kjzzqqk..",
        "....kqqqqqk..",
    ],
    "near_fwd": [
        "....klggjk...",
        "....kjjlggk..",
        "...kjjkklgk..",
        "...kjjk.klgk.",
        "...kjjk.klgk.",
        "..kqqqk.kzzqk",
        "..kqqqk.kqqqk",
    ],
    "near_back": [
        "....klggjk...",
        "....klgjjjk..",
        "...klgkkjjk..",
        "...klgk.kjjk.",
        "...klgk.kjjk.",
        "..kzzqk.kqqqk",
        "..kqqqk.kqqqk",
    ],
    "near_fwd_half": [
        "....klggjk...",
        "....kjlggk...",
        "....kjjlggk..",
        "....kjjklgk..",
        "....kjjklgk..",
        "...kqqqkzzqk.",
        "...kqqqkqqqk.",
    ],
    "near_back_half": [
        "....klggjk...",
        "....klgjjk...",
        "....klgjjjk..",
        "....klgkjjk..",
        "....klgkjjk..",
        "...kzzqkqqqk.",
        "...kqqqkqqqk.",
    ],
}

# Per frame: (arms, legs). Front and back: how much shorter each arm is as it swings forward,
# and which boot is raised by how much; the arm swings forward opposite the raised leg.
WALK_DOWN = [((0, 1), ("left", 1)), ((0, 2), ("left", 2)), ((0, 1), ("left", 1)),
             ((1, 0), ("right", 1)), ((2, 0), ("right", 2)), ((1, 0), ("right", 1))]
WALK_UP = [((1, 0), ("right", 1)), ((2, 0), ("right", 2)), ((1, 0), ("right", 1)),
           ((0, 1), ("left", 1)), ((0, 2), ("left", 2)), ((0, 1), ("left", 1))]
STAND = [((0, 0), ("left", 0))] * COLUMNS
SIDE_WALK = [("back", "near_fwd"), ("back", "near_fwd_half"), ("down", "pass"),
             ("fwd", "near_back"), ("fwd", "near_back_half"), ("down", "pass")]

COLLAR = (109, 24, 16)


def shoulders(frame):
    """The row the shoulders sit on, from the first row of the collar in the source frame."""
    src = frame.load()
    for y in range(SRC_H):
        if any(src[x, y][3] and src[x, y][:3] == COLLAR for x in range(SRC_W)):
            return int(y * H / SRC_H + 0.4)
    raise ValueError("no collar")


def fix_body(small, frame, row, column):
    px = small.load()
    top = shoulders(frame)
    for y in range(top, H):
        for x in range(W):
            px[x, y] = (0, 0, 0, 0)
    torso = (FRONT_TORSO, SIDE_TORSO, FRONT_TORSO, BACK_TORSO)[row]
    for i, line in enumerate(torso):
        for x, key in enumerate(line):
            if key != ".":
                put(px, x, top + i, key)
    legs_top = top + len(torso)
    rows = H - legs_top
    if row == 1:
        swing, legs = SIDE_WALK[column]
        side_arm(px, top, swing)
        lines = SIDE_LEGS[legs]
        lines = lines[:1] + lines[len(lines) - rows + 1 :] if rows < len(lines) else lines
    else:
        (left, right), lift = (STAND, None, WALK_DOWN, WALK_UP)[row][column]
        front_arm(px, top, "left", left)
        front_arm(px, top, "right", right)
        lines = front_legs(rows, lift)
    for i, line in enumerate(lines):
        for x, key in enumerate(line):
            if key != ".":
                put(px, x, legs_top + i, key)


def main():
    sheet = Image.open(ROOT / "tools/art/player_full.tga").convert("RGBA")
    out = Image.new("RGBA", (W * COLUMNS, H * ROWS), (0, 0, 0, 0))
    for row in range(ROWS):
        for column in range(COLUMNS):
            frame = sheet.crop((column * SRC_W, row * SRC_H, (column + 1) * SRC_W, (row + 1) * SRC_H))
            small = shrink(frame)
            fix_eyes(small, frame)
            fix_face(small)
            fix_body(small, frame, row, column)
            out.alpha_composite(small, (column * W, row * H))
    if len(sys.argv) > 1:
        dest = Path(sys.argv[1])
        dest.mkdir(parents=True, exist_ok=True)
        bg = (30, 28, 36, 255)
        # Side by side at their size on screen (32x76), then tripled to look at.
        pairs = Image.new("RGBA", (4 * 40 + 8, 80), bg)
        for i, row in enumerate((0, 1, 2, 3)):
            old = sheet.crop((0, row * SRC_H, SRC_W, (row + 1) * SRC_H)).resize((32, 76), Image.NEAREST)
            new = out.crop((0, row * H, W, (row + 1) * H)).resize((32, 76), Image.NEAREST)
            pairs.alpha_composite(old, (i * 40, 2)) if False else None
        board = Image.new("RGBA", (2 * (4 * 40) + 24, 84), bg)
        for i, row in enumerate((0, 1, 2, 3)):
            old = sheet.crop((0, row * SRC_H, SRC_W, (row + 1) * SRC_H)).resize((32, 76), Image.NEAREST)
            new = out.crop((0, row * H, W, (row + 1) * H)).resize((32, 76), Image.NEAREST)
            board.alpha_composite(old, (4 + i * 40, 4))
            board.alpha_composite(new, (4 * 40 + 20 + i * 40, 4))
        board.resize((board.width * 3, board.height * 3), Image.NEAREST).save(dest / "compare.png")
        frames = []
        for column in range(COLUMNS):
            strip = Image.new("RGBA", (4 * 40 * 2 + 24, 84), bg)
            for i, row in enumerate((0, 1, 2, 3)):
                old = sheet.crop((column * SRC_W, row * SRC_H, (column + 1) * SRC_W, (row + 1) * SRC_H)).resize((32, 76), Image.NEAREST)
                new = out.crop((column * W, row * H, (column + 1) * W, (row + 1) * H)).resize((32, 76), Image.NEAREST)
                strip.alpha_composite(old, (4 + i * 40, 4))
                strip.alpha_composite(new, (4 * 40 + 20 + i * 40, 4))
            frames.append(strip.resize((strip.width * 3, strip.height * 3), Image.NEAREST).convert("RGB"))
        frames[0].save(dest / "walk.gif", save_all=True, append_images=frames[1:], duration=120, loop=0)
        big = Image.new("RGBA", out.size, bg)
        big.alpha_composite(out)
        big.resize((out.width * 8, out.height * 8), Image.NEAREST).save(dest / "sheet_8x.png")
    out.save(ROOT / "assets/textures/ingame/player.tga")


if __name__ == "__main__":
    main()
