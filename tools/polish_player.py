"""Polishes the player's sprite sheet to sit with the rest of the game's art and gives him a
tougher look, keeping the character as drawn:

- a dark outline round the figure like the props and enemies have, and light from the upper left
  (a lighter rim on that side of each part, a darker one on the other);
- the hair throwing a shadow on the brow, narrowed eyes under heavy lids with brighter whites and
  irises, and a scar down one cheek;
- a plain, darker shirt (no print) with a darker collar, a leather bandolier of brass rounds
  across the chest (over the shoulder from the side), fingerless gloves;
- a belt with a buckle, the backpack shaded, trousers a shade lighter so they don't sink into the
  outline, and boots with lighter toes;
- the skin's half-transparent pixels made solid.

Reads tools/art/player_base.tga (the sheet as first drawn: six 16x38 frames in each of four rows:
standing, walking sideways, walking down, walking up) and writes
assets/textures/ingame/player.tga; run make_dodge_sheet.py after it, as the dodge roll is cut
from this sheet.

    polish_player.py [out.tga] [--hair ROWS,SIDE]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
BASE = ROOT / "tools/art/player_base.tga"
FW, FH = 16, 38

OUTLINE = (30, 16, 20, 255)
SKIN = (238, 195, 154)
SKIN_SHADE = (214, 160, 118)
TROUSERS = (20, 30, 27)
TROUSERS_NEW = (46, 50, 62)
TROUSERS_SHADE = (32, 34, 44)
TROUSERS_LIGHT = (62, 68, 82)
BOOT = (74, 44, 32)
BOOT_LIGHT = (104, 64, 44)
BOOT_ROWS = 2
HAIR = {(78, 36, 29), (115, 58, 48)}
SHIRT = (181, 58, 35)
SHIRTS = {(181, 58, 35), (207, 63, 36), (152, 37, 16)}
PRINT = {(152, 37, 16), (44, 21, 16)}
COLLAR = (140, 34, 20)
BELT = (58, 34, 24)
BUCKLE = (214, 178, 92)
BACKPACK = (138, 111, 48)
EYE_WHITE = (207, 192, 178)
EYE_WHITE_NEW = (240, 236, 228)
IRIS = (68, 92, 144)
IRIS_NEW = (120, 160, 214)
PUPILS = {(0, 0, 0), (3, 3, 5)}
LID = (40, 20, 16)
SCAR = (232, 150, 140)
STRAP = (66, 42, 30)
BRASS = (222, 182, 82)
GLOVE = (58, 40, 36)
# The rows: standing still, walking sideways, walking down the screen, walking up it.
STANDING, SIDEWAYS, DOWN, UP = 0, 1, 2, 3
FACING = {STANDING, DOWN}
# The shirt is dyed this much darker and duller, channel by channel.
SHIRT_DYE = (0.78, 0.72, 0.8)
# The hair is left as drawn; `--hair ROWS,SIDE` cuts it shorter (rows off the top, sides
# brought in to that many pixels off the face).
HAIR_CUT = (0, 0)

# Each colour's darker and lighter neighbour, for the shading; the rest keep theirs.
RAMPS = {
    (78, 36, 29): ((56, 24, 22), (115, 58, 48)),
    (115, 58, 48): ((78, 36, 29), (146, 80, 62)),
    SKIN: (SKIN_SHADE, (250, 216, 182)),
    (233, 189, 147): ((210, 156, 114), (246, 210, 176)),
    (181, 58, 35): ((140, 34, 20), (214, 80, 48)),
    (207, 63, 36): ((170, 48, 26), (232, 96, 58)),
    (152, 37, 16): ((110, 26, 14), (181, 58, 35)),
    TROUSERS_NEW: (TROUSERS_SHADE, TROUSERS_LIGHT),
    BOOT: ((50, 28, 22), BOOT_LIGHT),
    BACKPACK: ((100, 78, 34), (172, 142, 66)),
    BELT: ((40, 24, 18), (84, 52, 34)),
}
SKINS = {SKIN, SKIN_SHADE, (250, 216, 182), (233, 189, 147), (210, 156, 114), (246, 210, 176)}


def solid(px, x, y):
    return 0 <= x < FW and 0 <= y < FH and px[x, y][3] > 0


def colour_of(px):
    return lambda x, y: px[x, y][:3] if solid(px, x, y) else None


def cut_hair(frame, rows_off, side):
    """The top rows of the hair off and its sides brought in to a pixel or so off the face, the
    new top's corners rounded."""
    px = frame.load()
    hair = lambda x, y: solid(px, x, y) and px[x, y][:3] in HAIR
    face = [x for y in range(20) for x in range(FW) if solid(px, x, y) and px[x, y][0] > 200 and px[x, y][1] > 150]
    hairs = [y for y in range(FH) for x in range(FW) if hair(x, y)]
    if not face or not hairs:
        return
    left, right, top = min(face), max(face), min(hairs)
    for y in range(FH):
        for x in range(FW):
            if hair(x, y) and (y < top + rows_off or x < left - side or x > right + side):
                px[x, y] = (0, 0, 0, 0)
    for y, cut in ((top + rows_off, 2), (top + rows_off + 1, 1)):
        xs = [x for x in range(FW) if hair(x, y)]
        for x in xs[:cut] + xs[-cut:]:
            px[x, y] = (0, 0, 0, 0)


def dress(px, row):
    """Solid skin, plain shirt, lighter trousers and boots, a darker collar, the belt and buckle,
    the hair's shadow on the brow, lighter toes."""
    colour = colour_of(px)
    for y in range(FH):
        for x in range(FW):
            r, g, b, a = px[x, y]
            if a == 0:
                continue
            if abs(r - SKIN[0]) + abs(g - SKIN[1]) + abs(b - SKIN[2]) < 30:
                px[x, y] = SKIN + (255,)
            elif (r, g, b) in (TROUSERS, (18, 30, 26)):
                px[x, y] = (BOOT if y >= FH - BOOT_ROWS else TROUSERS_NEW) + (255,)
            elif (r, g, b) in PRINT and y > 18 and row in FACING:
                px[x, y] = SHIRT + (255,)
            elif a < 255:
                px[x, y] = (r, g, b, 255)
    for y in range(FH - 1, 0, -1):
        for x in range(FW):
            if colour(x, y) == SKIN and colour(x, y - 1) in HAIR:
                px[x, y] = SKIN_SHADE + (255,)
            elif colour(x, y) in SHIRTS and colour(x, y - 1) in (SKIN, SKIN_SHADE):
                px[x, y] = COLLAR + (255,)
    top = next((y for y in range(FH) if any(colour(x, y) == TROUSERS_NEW for x in range(FW))), None)
    if top is not None:
        for x in range(FW):
            if colour(x, top) in SHIRTS | {TROUSERS_NEW}:
                px[x, top] = BELT + (255,)
        belt = [x for x in range(FW) if colour(x, top) == BELT]
        if belt and row in FACING:
            px[(belt[0] + belt[-1]) // 2, top] = BUCKLE + (255,)
    for x in range(FW):
        if colour(x, FH - 1) == BOOT and colour(x, FH - 2) == BOOT and colour(x - 1, FH - 1) != BOOT:
            px[x, FH - 2] = BOOT_LIGHT + (255,)


def shade(frame):
    """Light from the upper left: the rim of the figure facing it lighter, the far rim darker."""
    px = frame.load()
    shaded = frame.copy()
    out = shaded.load()
    for y in range(FH):
        for x in range(FW):
            if not solid(px, x, y) or px[x, y][:3] not in RAMPS:
                continue
            dark, light = RAMPS[px[x, y][:3]]
            if not solid(px, x + 1, y) or not solid(px, x, y + 1):
                out[x, y] = dark + (255,)
            elif not solid(px, x - 1, y) or not solid(px, x, y - 1):
                out[x, y] = light + (255,)
    return shaded


def toughen(frame, row):
    """Narrowed eyes and a scar, gloves, a bandolier, a darker shirt."""
    px = frame.load()
    colour = colour_of(px)
    eye = [(x, y) for y in range(20) for x in range(FW) if colour(x, y) in {EYE_WHITE, IRIS} | PUPILS]
    for x, y in eye:
        if colour(x, y) == EYE_WHITE:
            px[x, y] = EYE_WHITE_NEW + (255,)
        elif colour(x, y) == IRIS:
            px[x, y] = IRIS_NEW + (255,)
    if eye:
        groups = []
        for x in sorted({x for x, _ in eye}):
            if groups and x - groups[-1][-1] <= 1:
                groups[-1].append(x)
            else:
                groups.append([x])
        for group in groups:
            cells = [(x, y) for x, y in eye if group[0] <= x <= group[-1]]
            top = min(y for _, y in cells)
            inner = min(group, key=lambda x: abs(x - FW / 2))
            if colour(inner, top) == EYE_WHITE_NEW:
                px[inner, top] = LID + (255,)
            for x in group:
                if colour(x, top - 1) in SKINS:
                    px[x, top - 1] = LID + (255,)
        group = groups[-1]
        bottom = max(y for x, y in eye if group[0] <= x <= group[-1])
        start = group[-1] if row != SIDEWAYS else group[0]
        for dx, dy in ((0, 1), (1, 2), (1, 3)):
            if colour(start + dx, bottom + dy) in SKINS:
                px[start + dx, bottom + dy] = SCAR + (255,)
    for y in range(20, FH):
        for x in range(FW):
            if colour(x, y) in SKINS and colour(x, y + 1) not in SKINS:
                px[x, y] = GLOVE + (255,)
    shirts = set(RAMPS[(181, 58, 35)]) | set(RAMPS[(207, 63, 36)]) | set(RAMPS[(152, 37, 16)]) | SHIRTS | {COLLAR}
    shirt = [(x, y) for y in range(FH) for x in range(FW) if colour(x, y) in shirts]
    if shirt and row != SIDEWAYS:
        left, right = min(x for x, _ in shirt), max(x for x, _ in shirt)
        top, bottom = min(y for _, y in shirt), max(y for _, y in shirt)
        for step, y in enumerate(range(top, bottom + 1)):
            t = (y - top) / max(1, bottom - top)
            x = round(left + 1 + t * (right - left - 2)) if row != UP else round(right - 1 - t * (right - left - 2))
            for strap in (x, x + 1):
                if colour(strap, y) in shirts:
                    px[strap, y] = STRAP + (255,)
            if colour(x, y) == STRAP and step % 2 == 0 and row != UP:
                px[x, y] = BRASS + (255,)
    elif shirt:
        top = min(y for _, y in shirt)
        for x in range(FW):
            if colour(x, top + 1) in shirts:
                px[x, top + 1] = STRAP + (255,)
    for x, y in shirt:
        if colour(x, y) in shirts:
            r, g, b = colour(x, y)
            px[x, y] = (int(r * SHIRT_DYE[0]), int(g * SHIRT_DYE[1]), int(b * SHIRT_DYE[2]), 255)


def outline(frame):
    px = frame.load()
    out = frame.copy()
    final = out.load()
    for y in range(FH):
        for x in range(FW):
            if not solid(px, x, y) and any(
                solid(px, x + dx, y + dy) for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))
            ):
                final[x, y] = OUTLINE
    return out


def polish(frame, row, hair=HAIR_CUT):
    frame = frame.copy()
    if hair[0] or hair[1]:
        cut_hair(frame, *hair)
    dress(frame.load(), row)
    frame = shade(frame)
    toughen(frame, row)
    return outline(frame)


def sheet(hair=HAIR_CUT):
    base = Image.open(BASE).convert("RGBA")
    out = Image.new("RGBA", base.size, (0, 0, 0, 0))
    for top in range(0, base.height, FH):
        for left in range(0, base.width, FW):
            frame = base.crop((left, top, left + FW, top + FH))
            out.paste(polish(frame, top // FH, hair), (left, top))
    return out


def main():
    args = sys.argv[1:]
    hair = HAIR_CUT
    if "--hair" in args:
        at = args.index("--hair")
        hair = tuple(int(n) for n in args[at + 1].split(","))
        del args[at:at + 2]
    path = Path(args[0]) if args else ROOT / "assets/textures/ingame/player.tga"
    sheet(hair).save(path)


if __name__ == "__main__":
    main()
