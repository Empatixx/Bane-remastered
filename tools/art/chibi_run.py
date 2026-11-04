"""Mockup: Elian as a chibi (big head, small body) with a bouncy side run. Hand-drawn parts,
placed per frame at whole-pixel offsets: the body bobs, the head leads the body into the run,
short legs cycle stride / lift / pass, the near arm pumps. Writes ops for lsprite per frame."""
import json, sys

PAL = {
    "a": "#1e1014",  # outline
    "H": "#733a30", "L": "#94523a", "S": "#4e241d",  # hair
    "f": "#eec39a", "i": "#d6a076", "h": "#0c080c", "g": "#f0ece4",  # skin, eye
    "l": "#a63926", "p": "#8d291c", "m": "#6d1810", "n": "#422a1e", "q": "#d8b45a",  # shirt, strap, brass
    "o": "#ac8e42", "r": "#7a6230",  # backpack
    "w": "#3e4452", "x": "#2e323e",  # trousers
    "z": "#5e3826", "A": "#8e5c3e",  # boots
}

HEAD = [
    "....aaaaa....",
    "..aaHHHHHaa..",
    ".aHHLLLHHHHa.",
    "aHHLLHHHHHHHa",
    "aHHHHHHHHHHHa",
    "aSHHHHSHHSHHa",
    "aSHHHSfSffSfa",
    "aSHHSffhffhfa",
    "aSHHSffhffhfa",
    ".aSHSffffffia",
    "..aaSfffffia.",
    "....aaaaaaa..",
]
HEAD_BLINK_EYES = None

BODY = [
    "aaa.aalllaa.",
    "aoraplnpppa.",
    "aorapplnppa.",
    "aorappplqpa.",
    "aaramppppma.",
    ".aa.annnnna.",
]

# No drawn legs: two small boots under the body, as in a Gungeon-like run. Each boot is a
# 5x4 stamp (outline round a 3x2 boot, toe lit). Per pose: (dx, lift) of the back boot, then the
# front one, from where they stand at rest.
BOOT = [
    ".aaa.",
    "azzAa",
    "azzza",
    ".aaa.",
]
BACK_BOOT = [
    ".aaa.",
    "azzza",
    "azzza",
    ".aaa.",
]
FEET = {
    "stand": [(0, 0), (0, 0)],
    "stride": [(-2, 0), (2, 0)],
    "lift": [(-1, 1), (1, 0)],
    "pass": [(0, 2), (0, 0)],
    "stride2": [(2, 0), (-2, 0)],
    "lift2": [(1, 0), (-1, 1)],
    "pass2": [(0, 0), (0, 2)],
}
BACK_FOOT_X, FRONT_FOOT_X = 6, 9
FOOT_ROWS = len(BOOT)


def feet(grid, pose, bases=(BACK_FOOT_X, FRONT_FOOT_X), boots=(BACK_BOOT, BOOT)):
    for (dx, lift), base, boot in zip(FEET[pose], bases, boots):
        stamp(grid, boot, base + dx, GROUND - FOOT_ROWS + 1 - lift)


# near arm: (stamp, x and y of its top left from the shoulder)
ARMS = {
    "fwd": ([".aaaa.", "alllffa", ".aaaaa."], 0, -1),
    "mid": ([".aa.", "alla", "alfa", ".aa."], -1, -1),
    "back": ([".aaaa.", "affllla", ".aaaaa."], -7, -1),
}

W, Hh = 20, 26
GROUND = 25  # bottom row of the boots' outline
# per frame: legs, arm, body bob (+down), head lead (+forward)
FRAMES = [
    ("stride", "back", 1, 1),
    ("lift", "mid", 0, 1),
    ("pass", "fwd", 0, 1),
    ("stride2", "fwd", 1, 1),
    ("lift2", "mid", 0, 1),
    ("pass2", "back", 0, 1),
]


def stamp(grid, rows, x0, y0):
    for dy, line in enumerate(rows):
        for dx, ch in enumerate(line):
            if ch != "." and 0 <= y0 + dy < Hh and 0 <= x0 + dx < W:
                grid[y0 + dy][x0 + dx] = ch


def frame(legs, arm, bob, lead):
    grid = [["."] * W for _ in range(Hh)]
    # the body sits on the boots, a row lower as they land
    body_top = GROUND - FOOT_ROWS + 2 - len(BODY) + bob
    feet(grid, legs)
    stamp(grid, HEAD, 3 + lead, body_top - len(HEAD) + 1)
    stamp(grid, BODY, 4, body_top)
    if arm is None:
        return ["".join(r) for r in grid]
    rows, dx, dy = ARMS[arm]
    # over the shirt the arm's edge is the shirt's shade, outline only where it leaves the body
    x0, y0 = 10 + dx, body_top + 2 + dy
    for j, line in enumerate(rows):
        for k, ch in enumerate(line):
            y, x = y0 + j, x0 + k
            if ch == "." or not (0 <= y < Hh and 0 <= x < W):
                continue
            under = grid[y][x]
            if ch == "a" and under in "lpmnq":
                ch = "m"
            elif ch == "a" and under in "or":
                ch = "r"
            grid[y][x] = ch
    return ["".join(r) for r in grid]


if __name__ == "__main__":
    frames = [frame(*f) for f in FRAMES]
    json.dump({"map": PAL, "frames": frames}, open(sys.argv[1], "w"))
    for y in range(Hh):
        print("  ".join(f[y] for f in frames))
