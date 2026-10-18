<div align="center">

# BANE

### Five floors down. Every floor remembers you.

A fast, hand-drawn top-down roguelike shooter. Dodge, shoot, die, get stronger, go deeper.

![Bane trailer](media/trailer.gif)

[Watch the trailer in full quality](media/trailer.mp4)

English | [Cesky](README.cs.md)

</div>

---

## The Bane is open. Go in.

Something tore a hole in the world. Its floors never look the same twice. You are Elian, sent down
through the portal. Below you wait rats, slimes, spiders and things with too many eyes. At the
end of every floor waits a boss who used to be somebody.

You start with a pistol, a shotgun and a dodge roll. Everything else you have to take.

## Why one more run

**16 guns.** Pistol, Uzi, Sniper, Flamethrower, Tesla Gun, RPG, Railgun and more. Every gun has
its own upgrade tree and a mutation at the end of it.

**7 bosses with a name and a story.** A hellhound that waits at the bottom, a scribe who wrote
down every prayer, a living chest, a king of spiders. Each boss has its own attacks and gets angry
when it is losing.

**A new dungeon every time.** Rooms, traps, shops, treasure and sacrifice altars are made fresh
for every floor. There are seven biomes: Stone, Sewer, Moss, Spider, Ice, Lava and Temple.

**Loot that changes the run.** 13 one-use artefacts, a shop, a slot machine, perks and a pet that
picks up your coins. Gamble for a gun, pay for it with blood, or save your coins.

**You get stronger even when you die.** Between runs you walk around the lab. Spend talents with
the Scientist, upgrade guns at the weapon table, take quests and challenges at the quest board.

**Made to look good.** Hand-drawn pixel art, dynamic light, a different mood in every biome, a
dodge roll that feels right, screen shake and hit flashes.

## Take a look

### Seven biomes

| | | |
|:-:|:-:|:-:|
| ![Stone](media/biome-stone.png)<br>**Stone** | ![Sewer](media/biome-sewer.png)<br>**Sewer** | ![Moss](media/biome-moss.png)<br>**Moss** |
| ![Spider](media/biome-spider.png)<br>**Spider** | ![Ice](media/biome-ice.png)<br>**Ice** | ![Lava](media/biome-lava.png)<br>**Lava** |
| ![Temple](media/biome-temple.png)<br>**Temple** | | |

### Rooms

| | | |
|:-:|:-:|:-:|
| ![Boss](media/room-boss.png)<br>**Every floor ends with a boss** | ![Shop](media/room-shop.png)<br>**Spend your coins** | ![Treasure](media/room-treasure.png)<br>**Treasure for the brave** |

## Controls

| | |
|---|---|
| **Move** `W` `A` `S` `D` | **Aim and shoot** mouse |
| **Dodge roll** `Space` | **Reload** `R` |
| **Switch gun** `1`-`9` or mouse wheel | **Interact** `E` |
| **Use artefact** `F` | **Map** `Tab` |

## Play it

```bash
git clone git@github.com:Empatixx/Bane-remastered.git
cd Bane-remastered/rust
cargo run --release
```

You need the [Rust toolchain](https://rustup.rs).

## Wiki

The game wiki is in [`docs/`](docs/) (in Czech). It describes every gun, enemy, artefact and biome.
Its pages are plain Markdown files in [`docs/content/`](docs/content/), so you can read them right
on GitHub.

## License

Bane is free software under the [GNU GPL v3](LICENSE).

---

<details>
<summary>For developers</summary>

Bane is written in Rust: `wgpu` and `winit` for graphics, `hecs` for the ECS, `rapier2d` for
physics, `kira` for audio. The simulation runs at a fixed 60 Hz. The game started as a Java game
and was ported to Rust. The Java version is still in the git history.

- [`rust/`](rust/): the game. Headless screenshots, debug switches (`BANE_THEME`, `BANE_AT`,
  `BANE_SEED`) and ECS notes are in [`rust/README.md`](rust/README.md) and [`rust/docs/`](rust/docs/).
- [`tools/`](tools/): Python scripts that make sprites, sounds and atmosphere.
- [`story.md`](story.md): the story and where it goes.

</details>
