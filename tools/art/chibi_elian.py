"""The hero as a chibi: 6x6 frames of 20x26, no arms (in play a floating hand holds the gun).
Rows: 0 stand facing down (breathing, a blink), 1 run to the side (from chibi_run.py, faces
right), 2 run down (towards the viewer), 3 run up (away), 4 stand facing the side, 5 stand
facing up. Hand-drawn parts placed at whole-pixel offsets per frame."""
import json, sys
import chibi_run as side

PAL = dict(side.PAL)
W, H, GROUND = side.W, side.Hh, side.GROUND

FRONT_HEAD = [
    "...aaaaaaa...",
    ".aaHHHHHHHaa.",
    "aHHLLLHHHHHHa",
    "aHLLHHHHHHHHa",
    "aHHHHHHHHHHHa",
    "aSHHSHHHSHHSa",
    "aSHSffSffSfSa",
    "aSSfhffffhfSa",
    "aSSfhffffhfSa",
    ".aSffffffffa.",
    "..aaffiffaa..",
    "....aaaaa....",
]
BLINK = {7: "aSSfSffffSfSa", 8: "aSSffffffffSa"}

BACK_HEAD = [
    "...aaaaaaa...",
    ".aaHHHHHHHaa.",
    "aHHHHLLLHHHHa",
    "aHHHLLHHHHHHa",
    "aHHHHHHHHHHHa",
    "aHHHHHHHHHHHa",
    "aSHHHHHHHHHSa",
    "aSHHSHHHSHHSa",
    "aSSHHSHSHHSSa",
    ".aSSSSSSSSSa.",
    "..aaSSSSSaa..",
    "....aaaaa....",
]

# No arms: in play a floating hand holds the gun, so the body is head, shirt, belt and legs.
FRONT_BODY = [
    "..aallllaa..",
    ".ammnlpppma.",
    ".ampnppppma.",
    ".amppnqppma.",
    ".amppppnpma.",
    "..aannnnaa..",
]
BACK_BODY = [
    "..aallllaa..",
    ".aaooooooaa.",
    ".aorooooroa.",
    ".aooooooooa.",
    ".amrrrrrrma.",
    "..aannnnaa..",
]

# Boots seen from the front or the back, side by side; lifting one by one or two rows.
side.FEET.update({
    "left1": [(0, 1), (0, 0)],
    "left": [(0, 2), (0, 0)],
    "right1": [(0, 0), (0, 1)],
    "right": [(0, 0), (0, 2)],
})
FRONT_FEET_X = (6, 10)


def stamp(grid, rows, x0, y0):
    for dy, line in enumerate(rows):
        for dx, ch in enumerate(line):
            if ch != "." and 0 <= y0 + dy < H and 0 <= x0 + dx < W:
                grid[y0 + dy][x0 + dx] = ch


def upright(head, body, legs, bob, blink=False):
    grid = [["."] * W for _ in range(H)]
    body_top = GROUND - side.FOOT_ROWS + 2 - len(body) + bob
    side.feet(grid, legs, FRONT_FEET_X, (side.BACK_BOOT, side.BACK_BOOT))
    head = list(head)
    if blink:
        for y, line in BLINK.items():
            head[y] = line
    stamp(grid, head, 4, body_top - len(head) + 1)
    stamp(grid, body, 4, body_top)
    return ["".join(r) for r in grid]


STAND = [upright(FRONT_HEAD, FRONT_BODY, "stand", b, blink=(i == 4))
         for i, b in enumerate([0, 0, 0, 1, 1, 1])]
RUN_PLAN = [("stand", 1), ("left1", 0), ("left", 0), ("stand", 1), ("right1", 0), ("right", 0)]
DOWN = [upright(FRONT_HEAD, FRONT_BODY, legs, bob) for legs, bob in RUN_PLAN]
UP = [upright(BACK_HEAD, BACK_BODY, legs, bob) for legs, bob in RUN_PLAN]
SIDE = [side.frame(legs, None, bob, lead) for legs, _arm, bob, lead in side.FRAMES]

SIDE_BLINK = {7: "aSHHSffSffSfa"}


def side_stand(bob, blink):
    head = side.HEAD
    if blink:
        side.HEAD = [SIDE_BLINK.get(y, line) for y, line in enumerate(head)]
    rows = side.frame("stand", None, bob, 0)
    side.HEAD = head
    return rows


BREATH = [0, 0, 0, 1, 1, 1]
STAND_SIDE = [side_stand(b, i == 4) for i, b in enumerate(BREATH)]
STAND_UP = [upright(BACK_HEAD, BACK_BODY, "stand", b) for b in BREATH]
ROWS = [STAND, SIDE, DOWN, UP, STAND_SIDE, STAND_UP]

if __name__ == "__main__":
    json.dump({"map": PAL, "rows": ROWS}, open(sys.argv[1], "w"))

