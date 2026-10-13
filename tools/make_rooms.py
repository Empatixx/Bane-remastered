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
        assert rows[row][column] == ".", (name, column, row, rows[row][column])
        rows[row][column] = "E"
    data["rows"] = ["".join(row) for row in rows]
    save(name, data)


def big_room(name, columns, rows, pillars, block, spawns):
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
    save(name, {"name": name, "kind": "Normal", "rows": ["".join(row) for row in grid]})


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
    mark("normal_01", [(2, 1), (13, 1), (3, 4), (12, 5), (2, 8), (13, 8)])
    mark("normal_02", [(3, 1), (14, 1), (3, 5), (14, 6), (3, 10), (14, 10)])
    mark("normal_14", [(3, 2), (12, 2), (3, 9), (12, 9), (7, 5), (8, 6)])
    mark("normal_15", [(2, 2), (17, 2), (9, 3), (2, 9), (17, 9), (9, 8)])
    big_room(
        "normal_17",
        32,
        22,
        pillars(32, 22, [(6, 4), (11, 8)]),
        (14, 9, 4, 4),
        mirrored(32, 22, [(3, 3), (9, 2), (3, 8), (13, 5), (7, 10)]),
    )
    big_room(
        "normal_18",
        38,
        24,
        pillars(38, 24, [(6, 4), (12, 8), (4, 9)]),
        (16, 10, 6, 4),
        mirrored(38, 24, [(3, 3), (10, 2), (3, 10), (14, 6), (8, 10), (15, 3)]),
    )
    big_room(
        "normal_19",
        44,
        28,
        pillars(44, 28, [(7, 5), (14, 9), (5, 11), (17, 3)]),
        (19, 12, 6, 4),
        mirrored(44, 28, [(3, 3), (11, 2), (3, 12), (16, 6), (10, 11), (19, 4), (17, 10), (7, 8)]),
    )


main()
