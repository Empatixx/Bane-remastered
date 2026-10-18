<div align="center">

# BANE

### Five floors down. Every one of them remembers who you are.

A fast, hand-drawn top-down roguelike shooter. Dodge, shoot, die, upgrade, go deeper.

![Bane trailer](media/trailer.gif)

[▶ Watch the trailer in full quality](media/trailer.mp4)

</div>

---

## The Bane is open. Go in.

Something tore a hole in the world, and its floors never look the same twice. You are Elian, the one
sent down through the portal. Below you wait rats, slimes, spiders and things with too many eyes,
and at the bottom of every floor, a boss who used to be somebody.

You get a pistol, a shotgun and a dodge roll. Everything else you have to take.

## Why you'll want one more run

**🔫 16 guns that feel different.** Pistol, Uzi, Sniper, Flamethrower, Tesla Gun, RPG, Railgun and
more. Every gun has its own upgrade tree and a mutation waiting at the end of it.

**👹 7 bosses with a name and a story.** A hellhound that waits at the bottom, a scribe who wrote
down every prayer, a living chest, a king of spiders. Each has its own attack patterns and goes into
a rage when it's losing.

**🗺️ A new dungeon every time.** Rooms, traps, shops, treasure and sacrifice altars are generated
fresh for every floor, across seven biomes: Stone, Sewer, Moss, Spider, Ice, Lava and Temple.

**🎲 Loot that changes the run.** 13 one-shot artefacts, a shop, a slot machine, perks and a pet that
fetches your coins. Gamble on a gun, pay for it in blood, or save up.

**⬆️ You get stronger even when you die.** Between runs you walk around the lab: spend talents with
the Scientist, upgrade guns at the weapon table, take on quests and challenges at the quest board.

**🎨 Built to look good.** Hand-drawn pixel art, dynamic lighting, atmosphere in every biome, a
dodge roll that feels right, screen shake and hit flashes where they belong.

## Take a look

### Seven biomes, seven moods

| | | |
|:-:|:-:|:-:|
| ![Stone](media/biome-stone.png)<br>**Stone** | ![Sewer](media/biome-sewer.png)<br>**Sewer** | ![Moss](media/biome-moss.png)<br>**Moss** |
| ![Spider](media/biome-spider.png)<br>**Spider** | ![Ice](media/biome-ice.png)<br>**Ice** | ![Lava](media/biome-lava.png)<br>**Lava** |
| ![Temple](media/biome-temple.png)<br>**Temple** | | |

### Meet the locals

| | | |
|:-:|:-:|:-:|
| ![Boss](media/room-boss.png)<br>**Every floor ends in a boss** | ![Shop](media/room-shop.png)<br>**Spend it while you have it** | ![Treasure](media/room-treasure.png)<br>**Treasure for the brave** |

## Controls

| | |
|---|---|
| **Move** `W` `A` `S` `D` | **Aim / shoot** mouse |
| **Dodge roll** `Space` | **Reload** `R` |
| **Switch gun** `1`–`9` or wheel | **Interact** `E` |
| **Artefact** `F` | **Map** `Tab` |

## Play it

```bash
git clone git@github.com:Empatixx/Bane_refactor.git
cd Bane_refactor/rust
cargo run --release
```

You need a [Rust toolchain](https://rustup.rs). It runs on macOS, Windows and Linux.

---

<details>
<summary><b>For developers</b></summary>

Bane is written in Rust: `wgpu` + `winit` for rendering, `hecs` ECS, `rapier2d` physics, `kira`
audio. The simulation runs at a fixed 60 Hz with a variable render rate. It started as a Java game
and was ported; the Java version lives on in the git history.

- [`rust/`](rust/) — the game. Headless screenshots, debug switches (`BANE_THEME`, `BANE_AT`,
  `BANE_SEED`) and the ECS notes are in [`rust/README.md`](rust/README.md) and [`rust/docs/`](rust/docs/).
- [`docs/`](docs/) — the Bane wiki (Czech): every gun, enemy, artefact and biome.
- [`tools/`](tools/) — Python scripts that generate sprites, sounds and atmosphere assets.
- [`story.md`](story.md) — the story and where it's going.

</details>
