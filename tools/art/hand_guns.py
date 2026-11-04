"""The guns as the hero holds them: small sprites at the chibi hero's pixel scale, pointing right,
one per gun, after the big HUD/ground pictures (same shapes and colours, far fewer pixels).
Each has a grip (where the hand holds it, the pivot it turns on) and a muzzle (where rounds
leave). Writes guns.json for drawing them with lsprite.

    uv run -q python tools/art/hand_guns.py OUT.json
"""
import json
import sys

PAL = {
    "a": "#141218",  # outline
    "G": "#4a4a56", "l": "#8a8a9a", "D": "#2a2a32",  # gun metal
    "V": "#9a5a3e", "W": "#7a3e2a", "w": "#552818",  # wood
    "S": "#d4d8e0", "s": "#9aa0ac", "h": "#ffffff",  # silver
    "Y": "#d8b45a", "y": "#a8843a",  # brass, yellow
    "R": "#d0403a", "r": "#8a2820",  # red
    "E": "#7a8a34", "e": "#4e5a1e",  # olive
    "B": "#4aa8ff", "b": "#bfe6ff",  # blue glow
    "N": "#4ae06a", "n": "#2a9a40",  # green glow
    "O": "#e0822e", "o": "#a8561c",  # orange coil
    "P": "#a85af0", "p": "#6a2ab0",  # violet
    "C": "#4ae0f0", "c": "#1a9aaa",  # cyan
}

GUNS = {
    "pistol": dict(grip=(2, 4), muzzle=(8, 2), rows=[
        "aaaaaaaa.",
        "alllllGGa",
        "aGGGGGGGa",
        "aaDGaaaaa",
        ".aDGa....",
        ".aaaa....",
    ]),
    "luger": dict(grip=(2, 4), muzzle=(10, 2), rows=[
        "..aaa......",
        "aaGlGaaaaaa",
        "aGGGGGGGGGa",
        "aawVaaaaaaa",
        ".awVa......",
        ".aaaa......",
    ]),
    "revolver": dict(grip=(2, 4), muzzle=(10, 2), rows=[
        "...aaaa....",
        "aaaShSSaaaa",
        "asSSsSSSSSa",
        "aaDSaSaaaaa",
        ".aDDaa.....",
        ".aaaa......",
    ]),
    "uzi": dict(grip=(3, 4), muzzle=(10, 2), rows=[
        "aaaaaaaaa..",
        "alGlllGGaaa",
        "aGGGGGGGGGa",
        "aaaDGaaaaaa",
        "..aDGa.....",
        "..aDGa.....",
        "..aaaa.....",
    ]),
    "blaster": dict(grip=(2, 4), muzzle=(9, 2), rows=[
        "aaaaaaaa..",
        "aShSSSSSaa",
        "aSRRRRSSRa",
        "aasRaaaaaa",
        ".asRa.....",
        ".aaaa.....",
    ]),
    "shotgun": dict(grip=(5, 4), muzzle=(17, 2), rows=[
        "aaa....aaaaaaaaaa.",
        "aVVaaaaGGllllllGa.",
        "awVVVVGGGGGGGGGGGa",
        "aawwwaaGaVVVVVaaa.",
        "....awVaaawwwa....",
        "....aaaa..aaa.....",
    ]),
    "m4": dict(grip=(4, 4), muzzle=(15, 3), rows=[
        "......aaa.......",
        "aaa..aGlGa......",
        "aGGaaaGGGaaaaaa.",
        "aDGGGGGGGGGGGGGa",
        "aaaDGaDDGaaaaaa.",
        "..aDGaaDa.......",
        "..aaa..aa.......",
    ]),
    "thompson": dict(grip=(5, 4), muzzle=(16, 2), rows=[
        "aaa...aaaaa......",
        "aVVaaaGlllGaaaaaa",
        "awVVVVGGGGGGVVVGa",
        "aawwaVVaDGaawwaa.",
        "....awVaDGa.aa...",
        "....aaaaDGa......",
        "........aaa......",
    ]),
    "sniper": dict(grip=(6, 4), muzzle=(20, 3), rows=[
        ".......aaaaaaa.......",
        "......aDlllBDa.......",
        "aaa....aaGaaaa.......",
        "aVVaaaaaGGGaaaaaaaaaa",
        "awVVVVVVVVVWWWWGGGGGa",
        "aawwaawVaaaaaaaaaaaa.",
        "....aaaa.............",
    ]),
    "rpg": dict(grip=(6, 4), muzzle=(20, 2), rows=[
        "..............aaa....",
        "aaaaaaaaaaaaaaEEEaa..",
        "aGGGlllGGGGGGaEEEEEa.",
        "aGGGGGGVVGGGGaeEEEEEa",
        "aaaaaawVaaaVaaeeeea..",
        ".....awVa.aVa.aaaa...",
        ".....aaaa.aaa........",
    ]),
    "minigun": dict(grip=(4, 5), muzzle=(16, 3), rows=[
        "aaaaaaaa.........",
        "aGGlGGGGaaaaaaaaa",
        "aGGGGGDGlllllllla",
        "aGGGGGGGGGGGGGGGa",
        "aGGGGGDGlllllllla",
        "aaaYyaaaaaaaaaaaa",
        "..aYya...........",
        "..aaaa...........",
    ]),
    "crossbow": dict(grip=(4, 4), muzzle=(15, 3), rows=[
        "...........aa...",
        "..........aRa...",
        "aaa......aaRaaa.",
        "aVVaaaaaaaWWWlla",
        "awVVVVVVWWWWaaa.",
        "aawwaVVaaaaRa...",
        "....awVa..aRa...",
        "....aaaa...aa...",
    ]),
    "flamethrower": dict(grip=(4, 4), muzzle=(16, 2), rows=[
        "aaaaaaaaaaaa.aaa.",
        "aDGGllllGGGaaaBaa",
        "aGGGGGGGGGGGGGbBa",
        "aaaRRRRRRaaaaaaaa",
        "..aRrrrrRa.......",
        "...aaDGaaa.......",
        "...aDGa..........",
        "...aaaa..........",
    ]),
    "plasma_rifle": dict(grip=(4, 4), muzzle=(16, 3), rows=[
        ".......aaaaa......",
        "aaa...aNnNNaaaa...",
        "asSaaaSSSSSSSSSaaa",
        "asSSSSSSSSSNNNNNNa",
        "aaasSaaaaaaaaaaaa.",
        "..asSa............",
        "..aaaa............",
    ]),
    "tesla_gun": dict(grip=(4, 4), muzzle=(15, 2), rows=[
        "aaaaaaaaaaaa.aaa.",
        "aGGllGGOOOOOaabBa",
        "aGGGGGGOoOoOGabBa",
        "aaaGGaaOOOOOaaaa.",
        "..aWVa.aaaaa.....",
        "..awVa...........",
        "..aaaa...........",
    ]),
    "arc_caster": dict(grip=(4, 4), muzzle=(16, 2), rows=[
        "aaaaaaaa.....aaa.",
        "aGGllGGOOOOaaPaaa",
        "aGGGGGGOoOoaPPPaa",
        "aaaGGaaOOOOaapaDa",
        "..aWVa.aaaa.aa.aa",
        "..awVa...........",
        "..aaaa...........",
    ]),
    "railgun": dict(grip=(5, 4), muzzle=(21, 3), rows=[
        "........aaa............",
        "aaaaaaaasCsaaaaaaaaaaa.",
        "aDDSSSSSSSSSSSSSSSSSSSa",
        "aDDSSSCCCCsCCCCsCCCCbba",
        "aaaasSaaaaaaaaaaaaaaaa.",
        "...asSa................",
        "...aaaa................",
    ]),
}

if __name__ == "__main__":
    for name, gun in GUNS.items():
        widths = {len(r) for r in gun["rows"]}
        assert len(widths) == 1, (name, widths)
    json.dump({"map": PAL, "guns": GUNS}, open(sys.argv[1], "w"), indent=1)
    print(len(GUNS), "guns")
