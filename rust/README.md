# Bane in Rust

Step-by-step port of the Java game to Rust. Lives next to the Java code in `rust/`.

```bash
cd rust
cargo run
```

Planned stack: `winit` + `wgpu` (window, graphics), `glam` (math), `rapier2d` (physics),
`kira` (audio), `serde` (saves), `tracing` (logging).

## Screenshots without a window (headless)

`--shot` runs the game offscreen at a fixed 60 updates per second, silent, and saves PNGs.
Times count from the moment the assets are loaded.

```bash
# one picture of a moss floor after 4 s, in the biggest normal room
BANE_START=dungeon BANE_THEME=moss BANE_AT=normal cargo run -- --shot moss.png --seconds 4

# 20 frames 0.05 s apart (moss_01.png …), walking right and shooting at a point
BANE_START=dungeon cargo run -- --shot moss.png --seconds 6 --shots 20 --interval 0.05 \
  --hold D:2-6 --fire 3-5 --mouse 700,270 --size 960x540
```

Flags: `--seconds`, `--shots`/`--interval`, `--size WxH` (default 1920x1080), `--mouse X,Y`
(in 960×540 game pixels), `--hold KEY:FROM-TO` (letters, digits, Space, Esc, Tab, Enter, Shift)
and `--fire FROM-TO`. The usual `BANE_SEED`, `BANE_THEME`, `BANE_START`, `BANE_AT`
(`boss`, `normal`, `treasure`, `shop`, `sacrifice`) and `BANE_DATA` still apply.
