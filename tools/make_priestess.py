"""A priestess of the temple that raised the hero from the crypt, drawn at the player's own scale
and in his style (see polish_player.py): a chibi head with big eyes on a small body, a dark
outline round the whole figure, flat hand-picked palettes with one shade and one highlight per
material, light from the upper left, hair in chunky clusters.

She wears a long white temple robe to her feet with pale-gold trim, a royal-blue sash and stole
(the temple carpet's blue and gold, see make_carpets.py), a gold circlet with a blue stone, and
long dark plum hair falling over her shoulders.

Writes assets/textures/ingame/priestess.tga: six 16x38 frames in each of two rows,

- row 0, idle: hands folded at the sash, eyes open; a slow breath (head and chest rise a pixel
  on frames 2-4) and the hem swaying a pixel;
- row 1, praying: palms pressed together and raised under the chin, eyes closed; the same
  breath and sway, and a faint pale-gold glow at the hands on frames 2 and 3.

Previews (dark background, scaled with NEAREST) go to ~/Desktop/Bane-videa/priestess/ before
the sheet is written.

    uv run -q --with pillow python tools/make_priestess.py [out.tga]
"""

import sys
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "assets/textures/ingame/priestess.tga"
PREVIEWS = Path.home() / "Desktop/Bane-videa/priestess"
PLAYER = ROOT / "assets/textures/ingame/player.tga"
FW, FH = 16, 38
FRAMES = 6
BACKDROP = (24, 20, 28, 255)

PALETTE = {
    # The player's outline, lid and skin ramp.
    "o": (30, 16, 20),
    "k": (40, 20, 16),
    "s": (238, 195, 154),
    "S": (214, 160, 118),
    "l": (250, 216, 182),
    "m": (232, 150, 140),
    "M": (196, 112, 100),
    # Eyes: whites as the player's, pupils, violet irises.
    "w": (240, 236, 228),
    "p": (24, 14, 14),
    "i": (78, 60, 144),
    # Long hair, a dark plum (the hero's is red-brown).
    "d": (40, 24, 44),
    "h": (66, 40, 66),
    "H": (108, 70, 100),
    # The white robe (the temple's marble).
    "R": (196, 188, 176),
    "r": (230, 224, 212),
    "W": (250, 248, 240),
    # Gold trim and circlet (the temple carpet's gold), and the pale gold of the glow.
    "G": (178, 130, 36),
    "g": (234, 186, 56),
    "y": (255, 236, 150),
    # Royal blue sash and stole (the temple carpet's blue).
    "B": (10, 14, 70),
    "b": (28, 40, 150),
    "c": (64, 86, 200),
}

HEAD = [
    "................",
    "......oooo......",
    "....ooHHhhoo....",
    "..ooHHhhhhHhoo..",
    ".ohHhhhddhhHhdo.",
    "ohHhhhHddHhhhhdo",
    "ohHhhHdssdHhhhdo",
    "ohhhHdssssdHhhdo",
    "ohhHdSssssSdHhdo",
    "ohhygggcbgggGhdo",
    "oHhSssssssssShdo",
    "oHdsssssssssshdo",
]

EYES_OPEN = [
    "ohdkkksssskkkhdo",
    "oHhswpsssswpshdo",
    "oHdsiissssiishdo",
    "ohdsssssssssshdo",
]

FACE = [
    "oHhmssssssssmhdo",
    "oHdSssssMsssShdo",
    "ohdoSssssssSoddo",
    "oHhhdoSssSodhhdo",
]

IDLE_BODY = [
    "ohHhoWgSSgWohhdo",
    "oHhhWWrggrrRhhdo",
    "ohhdWrrrrrrRdhdo",
    "ohdWWrrrrrrRRdho",
    "odWWRrrrrrrRRRdo",
    "oWWRrylsssGRRRRo",
    "oWWRrySssSGRRRRo",
    "oWRcbbbyybbbBRRo",
    "oWRBBBBGGBBBBRRo",
]

# Praying: the eyes closed under lowered lids, and from the lashes down the forearms raised at
# her sides, palms up at the chin, bell sleeves hanging from them.
EYES_CLOSED = [
    "ohdsssssssssshdo",
    "oHhsSSssssSSshdo",
    "oHdskksssskkshdo",
]

PRAY_BODY = [
    "oooksssssssskooo",
    "olsomssssssmosSo",
    "olSossssMsssosSo",
    "osSoSssssssSosSo",
    "oyGohoSssSohoyGo",
    "oWRohWgSSgrhoWRo",
    "oWRohWrggrRhoWRo",
    "oWrRhWrrrrRRhWRo",
    "oWrRdWrrrrrRdWRo",
    "oWrRWrrrrrrRWrRo",
    "oWrRWrrrrrrRWrRo",
    "ooWRWrrrrrrRWRoo",
    "..ocbbbyybbbBo..",
    "..oBBBBGGBBBBo..",
]

SKIRT = [
    "..oWrrrcbrrrRo..",
    "..oWrrrcbrrrRo..",
    "..oWrrrcbrrrRo..",
    ".oWrrRrcbrRrrRo.",
    ".oWWrRryGrRrrRo.",
    ".oWrrRrrrrRrrRo.",
    ".oWrrRrrrrrRrRo.",
    ".oyggggggggggGo.",
    "..oooooooooooo..",
]

# The praying hands' palms (rows and columns), lit by the glow.
PALM_ROWS = range(16, 19)
PALM_COLUMNS = (1, 2, 13, 14)

# Frame by frame: is the breath drawn in (head and chest a pixel up), where the hem has swung.
BREATH = [0, 0, 1, 1, 1, 0]
SWAY = [0, 0, 0, 1, 1, 0]
# The rows that rise with the breath (above the chest row that stretches), and the hem's rows.
CHEST = 24
HEM = 34
# The glow at the praying hands: none, faint (palms and fingertips), full (their outline and a mote).
GLOW = [0, 0, 1, 2, 0, 0]


def grid(rows):
    for row in rows:
        assert len(row) == FW, (row, len(row))
    return [list(row) for row in rows]


def pose(praying):
    if praying:
        rows = grid(HEAD + EYES_CLOSED + PRAY_BODY + SKIRT)
    else:
        rows = grid(HEAD + EYES_OPEN + FACE + IDLE_BODY + SKIRT)
    assert len(rows) == FH, len(rows)
    return rows


def breathe(rows, up):
    if not up:
        return rows
    return rows[1 : CHEST + 1] + rows[CHEST:]


def sway(rows, shift):
    out = [row[:] for row in rows]
    for y in range(HEM, FH):
        row = rows[y]
        out[y] = (["."] * shift + row)[:FW] if shift >= 0 else (row[-shift:] + ["."] * -shift)
    return out


def glow(rows, level):
    """Lights the raised hands: the palms their light skin tone and the outline over the
    fingertips pale gold; at the brightest, the whole outline round them and a mote above each."""
    if not level:
        return rows
    out = [row[:] for row in rows]
    for y in PALM_ROWS:
        for x in PALM_COLUMNS:
            out[y][x] = "l"
            around = ((x - 1, y), (x + 1, y), (x, y - 1)) if level > 1 else ((x, y - 1),)
            for nx, ny in around:
                if 0 <= nx < FW and rows[ny][nx] == "o":
                    out[ny][nx] = "y"
    if level > 1:
        top = PALM_ROWS.start - 2
        for x in (1, 14):
            out[top][x] = "y"
    return out


def frame(rows):
    image = Image.new("RGBA", (FW, FH), (0, 0, 0, 0))
    px = image.load()
    for y, row in enumerate(rows):
        for x, key in enumerate(row):
            if key != ".":
                px[x, y] = PALETTE[key] + (255,)
    return image


def sheet():
    out = Image.new("RGBA", (FW * FRAMES, FH * 2), (0, 0, 0, 0))
    for r, praying in enumerate((False, True)):
        for f in range(FRAMES):
            rows = pose(praying)
            if praying:
                rows = glow(rows, GLOW[f])
            rows = sway(breathe(rows, BREATH[f]), SWAY[f])
            out.paste(frame(rows), (f * FW, r * FH))
    return out


def on_dark(image, scale):
    bg = Image.new("RGBA", image.size, BACKDROP)
    bg.alpha_composite(image)
    return bg.resize((image.width * scale, image.height * scale), Image.NEAREST)


def previews(out):
    PREVIEWS.mkdir(parents=True, exist_ok=True)
    on_dark(out, 8).save(PREVIEWS / "priestess_sheet.png")
    # Beside the player's standing frame at the same scale, and a little ring of her sisters.
    player = Image.open(PLAYER).convert("RGBA").crop((0, 0, FW, FH))
    pair = Image.new("RGBA", (FW * 4 + 12, FH), (0, 0, 0, 0))
    pair.alpha_composite(player, (0, 0))
    pair.alpha_composite(out.crop((0, 0, FW, FH)), (FW + 4, 0))
    pair.alpha_composite(out.crop((3 * FW, FH, 4 * FW, 2 * FH)), (2 * FW + 8, 0))
    pair.alpha_composite(out.crop((0, FH, FW, 2 * FH)), (3 * FW + 12, 0))
    on_dark(pair, 8).save(PREVIEWS / "priestess_beside_player.png")
    frames = []
    for r in range(2):
        for f in range(FRAMES):
            frames.append(on_dark(out.crop((f * FW, r * FH, (f + 1) * FW, (r + 1) * FH)), 8).convert("P"))
    frames[0].save(
        PREVIEWS / "priestess.gif", save_all=True, append_images=frames[1:], duration=180, loop=0
    )


def main():
    out = sheet()
    previews(out)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    out.save(OUT)


if __name__ == "__main__":
    main()
