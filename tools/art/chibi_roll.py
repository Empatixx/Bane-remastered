"""The hero's dodge roll, Gungeon-like: he crouches, leaps forward into the air, tucks and turns a
full somersault at the top (whole quarter turns, so no pixel is lost), and lands in a crouch.
Six 24x32 frames facing right; the ground is row 28, so his feet stay where they are in his
20x26 frames when both are drawn centred on him, and the eight rows above give the jump room.
"""
import chibi_run as side

W, H = 24, 32
GROUND = 28
X0 = 2  # where his 20-wide frame sits in the wider cell

# Curled up in the air, facing right: head forward, knees and boots pulled to the chest.
TUCK = [
    "....aaaaaa....",
    "..aaHHHHHHaa..",
    ".aHHLLHHHHHHa.",
    "aHHLHHHHHHHHa.",
    "aHHHHHHHHSffa.",
    "aHHHHHHHSffhfa",
    "aoaHHHHHSfffia",
    "aoraaSSSaffaa.",
    "aorapplpaAzza.",
    "aaramppmazzza.",
    ".aaannnnaaaa..",
    "...aaaaaa.....",
    "..............",
    "..............",
]


def turned(rows, quarters):
    """`rows` turned a quarter clockwise `quarters` times."""
    for _ in range(quarters % 4):
        rows = ["".join(rows[len(rows) - 1 - y][x] for y in range(len(rows))) for x in range(len(rows[0]))]
    return rows


def blank():
    return [["."] * W for _ in range(H)]


def place_side(grid, legs, bob, lead, lift):
    """One of his own side frames, lifted `lift` rows off the ground."""
    rows = side.frame(legs, None, bob, lead)
    top = GROUND - side.GROUND - lift
    for y, line in enumerate(rows):
        for x, ch in enumerate(line):
            yy, xx = top + y, X0 + x
            if ch != "." and 0 <= yy < H and 0 <= xx < W:
                grid[yy][xx] = ch


def place_tuck(grid, quarters, lift, dx):
    rows = turned(TUCK, quarters)
    top = GROUND - len(rows) + 1 - lift
    left = X0 + 3 + dx
    for y, line in enumerate(rows):
        for x, ch in enumerate(line):
            yy, xx = top + y, left + x
            if ch != "." and 0 <= yy < H and 0 <= xx < W:
                grid[yy][xx] = ch


side.FEET.update({
    "crouch": [(-1, 0), (1, 0)],
    "leap": [(-4, 1), (-2, 0)],
})


def frames():
    out = []
    # 0 crouch to spring: low, leaning in
    g = blank(); place_side(g, "crouch", 2, 1, 0); out.append(g)
    # 1 leap: up and stretched forward, boots trailing
    g = blank(); place_side(g, "leap", 0, 2, 4); out.append(g)
    # 2-4 tucked, turning over at the top of the jump
    for quarters, lift, dx in ((1, 7, 1), (2, 8, 2), (3, 5, 2)):
        g = blank(); place_tuck(g, quarters, lift, dx); out.append(g)
    # 5 landing in a crouch
    g = blank(); place_side(g, "crouch", 2, 0, 0); out.append(g)
    return [["".join(r) for r in g] for g in out]


ROLL = frames()

if __name__ == "__main__":
    for y in range(H):
        print("  ".join(f[y] for f in ROLL))
