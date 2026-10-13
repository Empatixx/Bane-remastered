import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent / "rust/assets/tilesets"
OLD_COLUMNS = 21
FLOOR_EXTRA = 8
EDGE_VARIANTS = 8
PLAIN_EXTRA = 8
EDGE_RULES = [
    ("floor_below_wall", 0),
    ("floor_above_wall", 1),
    ("floor_left_of_wall", 2),
    ("floor_right_of_wall", 3),
]


def plain(rule):
    return f', "plain": {json.dumps(rule["plain"])}' if rule.get("plain") else ""


def main():
    columns = OLD_COLUMNS + FLOOR_EXTRA + EDGE_VARIANTS * len(EDGE_RULES) + PLAIN_EXTRA
    text = (ROOT / "dungeon.json").read_text()
    shift = columns - OLD_COLUMNS

    def move(match):
        numbers = [int(number) for number in match.group(1).split(",")]
        numbers = [number + shift if number >= OLD_COLUMNS else number for number in numbers]
        return '"tiles": [' + ", ".join(map(str, numbers)) + "]"

    text = re.sub(r'"tiles": \[([^\]]*)\]', move, text)
    text = text.replace(f'"columns": {OLD_COLUMNS}', f'"columns": {columns}')
    floor_extra = list(range(OLD_COLUMNS, OLD_COLUMNS + FLOOR_EXTRA))
    text = text.replace(
        '"tiles": [5, 6, 6, 6, 6, 6, 12, 13, 14, 15, 16, 17, 18, 19]',
        '"tiles": [12, 13, 14, 15, 16, 17, 18, 19, ' + ", ".join(map(str, floor_extra)) + "]",
    )
    data = json.loads(text)
    first = OLD_COLUMNS + FLOOR_EXTRA
    for name, index in EDGE_RULES:
        extras = list(range(first + index * EDGE_VARIANTS, first + (index + 1) * EDGE_VARIANTS))
        for rule in data["rules"]:
            if rule["name"] == name:
                rule["tiles"] = rule["tiles"] + extras
    plain_first = columns - PLAIN_EXTRA
    for rule in data["rules"]:
        if rule["name"] == "floor":
            rule["plain"] = [5, 6] + list(range(plain_first, columns))
    out = ROOT / "dungeon_wide.json"
    lines = ["{", f'  "columns": {data["columns"]},', '  "clustered": true,', f'  "rows": {data["rows"]},', f'  "solid_from_row": {data["solid_from_row"]},', '  "rules": [']
    for position, rule in enumerate(data["rules"]):
        comma = "," if position < len(data["rules"]) - 1 else ""
        lines.append(
            f'    {{ "name": "{rule["name"]}", "pattern": {json.dumps(rule["pattern"])}, "tiles": {json.dumps(rule["tiles"])}{plain(rule)} }}{comma}'
        )
    lines += ["  ]", "}", ""]
    out.write_text("\n".join(lines))


main()
