import json
import os

ROOMS = "rust/assets/rooms"


def load(name):
    with open(f"{ROOMS}/{name}.json") as handle:
        return json.load(handle)


def save(name, data):
    with open(f"{ROOMS}/{name}.json", "w") as handle:
        json.dump(data, handle, indent=2)
        handle.write("\n")


def mark(name, spots):
    data = load(name)
    rows = [list(row) for row in data["rows"]]
    for column, row in spots:
        assert rows[row][column] in ".E", (name, column, row, rows[row][column])
        rows[row][column] = "E"
    data["rows"] = ["".join(row) for row in rows]
    save(name, data)


def big_room(name, columns, rows, pillars, block, spawns, kind="Normal"):
    grid = [["." for _ in range(columns)] for _ in range(rows)]
    for column in range(columns):
        grid[0][column] = grid[rows - 1][column] = "#"
    for row in range(rows):
        grid[row][0] = grid[row][columns - 1] = "#"
    for column in (columns // 2 - 1, columns // 2):
        grid[0][column] = grid[rows - 1][column] = "D"
    for row in (rows // 2 - 1, rows // 2):
        grid[row][0] = grid[row][columns - 1] = "D"
    for column, row in pillars:
        for dx in (0, 1):
            for dy in (0, 1):
                grid[row + dy][column + dx] = "#"
    left, top, width, height = block
    for dx in range(width):
        for dy in range(height):
            grid[top + dy][left + dx] = "#"
    for column, row in spawns:
        assert grid[row][column] == ".", (name, column, row)
        grid[row][column] = "E"
    save(name, {"name": name, "kind": kind, "rows": ["".join(row) for row in grid]})


def mirrored(columns, rows, quarter):
    points = set()
    for column, row in quarter:
        for x in (column, columns - 1 - column):
            for y in (row, rows - 1 - row):
                points.add((x, y))
    return sorted(points)


def pillars(columns, rows, quarter):
    result = set()
    for column, row in quarter:
        for x in (column, columns - 2 - column):
            for y in (row, rows - 2 - row):
                result.add((x, y))
    return sorted(result)


def main():
    big_room(
        "normal_17",
        30,
        20,
        pillars(30, 20, [(5, 3), (9, 7)]),
        (13, 8, 4, 4),
        mirrored(30, 20, [(3, 3), (8, 2), (3, 8), (11, 4), (7, 8)]),
    )
    big_room(
        "normal_18",
        34,
        22,
        pillars(34, 22, [(6, 4), (10, 8)]),
        (14, 9, 6, 4),
        mirrored(34, 22, [(3, 3), (9, 2), (3, 9), (13, 6), (8, 10)]),
    )
    big_room(
        "normal_19",
        38,
        24,
        pillars(38, 24, [(6, 4), (11, 8), (4, 9)]),
        (16, 10, 6, 4),
        mirrored(38, 24, [(3, 3), (9, 2), (3, 10), (14, 6), (8, 10), (14, 3)]),
    )
    big_room(
        "normal_20",
        28,
        18,
        pillars(28, 18, [(5, 4), (10, 6)]),
        (11, 8, 6, 2),
        mirrored(28, 18, [(3, 3), (8, 2), (3, 7), (12, 4), (8, 8)]),
    )
    big_room(
        "normal_21",
        32,
        20,
        pillars(32, 20, [(6, 4), (12, 5)]),
        (14, 8, 4, 4),
        mirrored(32, 20, [(3, 3), (9, 2), (3, 8), (14, 3), (9, 8)]),
    )
    big_room(
        "normal_22",
        36,
        22,
        pillars(36, 22, [(6, 4), (12, 6), (5, 10)]),
        (16, 9, 4, 4),
        mirrored(36, 22, [(3, 3), (9, 2), (3, 8), (14, 3), (9, 9), (16, 5)]),
    )
    big_room(
        "ambush_01",
        34,
        22,
        pillars(34, 22, [(7, 4), (13, 9)]),
        (15, 9, 4, 4),
        mirrored(34, 22, [(3, 3), (10, 2), (3, 9), (14, 5), (9, 8), (6, 6)]),
        "Ambush",
    )
    big_room(
        "ambush_02",
        38,
        24,
        pillars(38, 24, [(8, 5), (13, 9), (5, 10)]),
        (17, 10, 4, 4),
        mirrored(38, 24, [(3, 3), (11, 2), (3, 8), (15, 4), (10, 9), (7, 7), (16, 8)]),
        "Ambush",
    )
    big_room(
        "ambush_03",
        32,
        22,
        pillars(32, 22, [(6, 5), (12, 8)]),
        (13, 9, 6, 4),
        mirrored(32, 22, [(3, 3), (9, 2), (3, 8), (13, 4), (8, 9), (5, 6)]),
        "Ambush",
    )


main()
