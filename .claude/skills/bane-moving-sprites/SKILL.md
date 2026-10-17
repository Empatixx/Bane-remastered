---
name: bane-moving-sprites
description: Use when adding or changing anything in Bane (rust/) that is drawn and moves — an NPC, a figure in a cinematic, a creature, a prop that travels or changes state — or when picking which sprite sheet, row and clip an entity plays.
---

# Moving sprites in Bane

## Core rule

What moves on screen plays the animation for that movement. A walking figure plays a
walk/run row, faces the way it goes, and its feet keep pace with how fast it goes. Changing
`Position` under an idle or still sprite makes it slide like a cut-out, and that is a bug.

## Recipe for an entity that moves

1. **Find its sheet and rows first.** Read the `PropSpec` (`rust/app/src/props.rs`), the
   enemy spec, or the `tools/make_*.py` that drew it, and write down which row is which
   (stand, walk/run, attack, special).
2. **No walk row means no walking.** Pick another sheet that has one (for example the priest,
   tinted by `Color`), or draw the row first with the sheet's own `tools/make_*.py`, at the
   sheet's own pixel scale and palette (see the libresprite skill for pixel work). Only then
   move it.
3. **Spawn it with the clip it will play.** Give it
   `Animation::new(columns, rows, Clip { row, frames, frame_time, looping, flipped })`
   together with `Sprite::cell(texture, 0, row, columns, rows)`.
4. **Switch clips when its state changes.** Use the walk row while `Position` changes, the
   stand row when it stops, and a one-shot clip (`looping: false`, then `ThenPlay`) for
   actions. Call `animation.play(...)` only when the row really changes, not every frame.
5. **Face the direction of travel.** Set `flipped` from the sign of x velocity (sheets face
   right).
6. **Match the step to the speed.** Choose `frame_time` so a full cycle covers about the
   distance one stride travels. Feet should not skate.
7. **Ground it.** Use `YSort::bottom(size)` for depth and `Lit` to take the scene's light.
   Never bob the head separately from the body (at 10x19 pixels it reads as the head coming
   off).
8. **Look at it moving before calling it done.** Record a few frames headless
   (`bane --shot f.png --seconds S --shots N --interval 0.05`) or a short clip, and check
   them for sliding, facing and pacing.

## Quick reference: character sheets

| Sheet | Grid | Rows |
|---|---|---|
| `PRIEST` / `PILGRIM` (`tools/make_priest.py`) | 4x3, 10x19 shown at 4x | 0 stand, 1 run (faces right), 2 pray |
| `GUIDE` prop (Elian) | uses `PRIEST` | same |
| `SCIENTIST` | 6x1, 16x40 | idle only, so he must stand still |
| `SHOPKEEPER` | 24x1, 80x90 | idle only, so he must stand still |
| `player.tga` | see `systems/player_animation.rs` | walk rows per facing |

## Common mistakes

| Mistake | Fix |
|---|---|
| Sliding an idle-only NPC (Scientist, Shopkeeper) across the screen | Use a sheet with a walk row, or draw one first |
| Walk clip left running after the figure stops | Switch back to the stand row on arrival |
| Figure walks left but faces right | Set `flipped` from the x direction |
| Run cycle on a slow amble, or the reverse | Fit `frame_time` to the speed |
| A prop that changes state just swaps texture | Give it a row per state, or a one-shot transition clip |
