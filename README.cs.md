<div align="center">

<img src="media/banner.png" alt="Bane Remastered - top-down roguelike střílečka přestavěná v Rustu" width="100%">

**Rychlá ručně kreslená top-down roguelike střílečka. Pět pater dolů. Každé patro si tě pamatuje.**

[![Rust](https://img.shields.io/badge/Rust-2021-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![wgpu](https://img.shields.io/badge/graphics-wgpu-5586A4)](https://wgpu.rs/)
[![ECS](https://img.shields.io/badge/ECS-hecs-1B1B1B)](https://github.com/Ralith/hecs)
[![Tested on](https://img.shields.io/badge/tested%20on-macOS-000000?logo=apple&logoColor=white)](#play-it)
[![Wiki](https://img.shields.io/badge/wiki-online-E27226)](https://empatixx.github.io/Bane-remastered/docs)
[![License: GPL v3](https://img.shields.io/badge/license-GPL--3.0-black)](LICENSE)

![Bane trailer](media/trailer.gif)

[Celý trailer v plné kvalitě](media/trailer.mp4) | [English](README.md) | Česky

</div>

---

> **Bane Remastered je nová verze hry [Bane](https://github.com/Empatixx/Bane).**
> První Bane byla napsaná v Javě a je archivovaná. Tohle je stejná hra postavená znovu od nuly
> v Rustu, s mnohem větším obsahem.

## Bane je otevřená. Vstup.

Něco roztrhlo svět. Jeho patra nikdy nevypadají stejně. Jsi Elian, poslaný dolů portálem. Dole na
tebe čekají krysy, slizy, pavouci a věci s příliš mnoha očima. Na konci každého patra stojí boss,
který byl kdysi někým.

Začínáš s pistolí, brokovnicí a úhybem. Všechno ostatní si musíš vzít.

## Proč ještě jeden run

**17 zbraní.** Pistol, Uzi, Sniper, Flamethrower, Tesla Gun, RPG, Railgun a další. Každá zbraň má
vlastní strom vylepšení a na jeho konci mutaci.

**7 bossů se jménem a příběhem.** Pekelný pes, který čeká na dně, písař, který zapsal každou
modlitbu, oživlá truhla, král pavouků. Každý boss má vlastní útoky a zuří, když prohrává.

**Pokaždé nový dungeon.** Místnosti, pasti, obchody, poklady a obětní oltáře se generují znovu pro
každé patro. Je tu sedm biomů: Stone, Sewer, Moss, Spider, Ice, Lava a Temple.

**Loot, který mění run.** 13 artefaktů na jedno použití, obchod, hrací automat, perky a mazlíček,
který sbírá mince. Vsaď na zbraň, plať za ni krví, nebo šetři.

**Zesiluješ, i když umřeš.** Mezi runy chodíš po laboratoři. U Vědce utrácíš talenty, u stolu se
zbraněmi vylepšuješ zbraně, na nástěnce bereš mise a výzvy.

**Vypadá dobře.** Ručně kreslená pixel art grafika, dynamické světlo, jiná nálada v každém biomu,
úhyb, který má správný pocit, otřesy obrazovky a záblesky při zásahu.

## Podívej se

### Sedm biomů

| | | |
|:-:|:-:|:-:|
| ![Stone](media/biome-stone.png)<br>**Stone** | ![Sewer](media/biome-sewer.png)<br>**Sewer** | ![Moss](media/biome-moss.png)<br>**Moss** |
| ![Spider](media/biome-spider.png)<br>**Spider** | ![Ice](media/biome-ice.png)<br>**Ice** | ![Lava](media/biome-lava.png)<br>**Lava** |
| ![Temple](media/biome-temple.png)<br>**Temple** | | |

### Místnosti

| | | |
|:-:|:-:|:-:|
| ![Boss](media/room-boss.png)<br>**Každé patro končí bossem** | ![Obchod](media/room-shop.png)<br>**Utrať mince** | ![Poklad](media/room-treasure.png)<br>**Poklad pro odvážné** |

## Od první Bane k Remastered

| | [Bane](https://github.com/Empatixx/Bane) (Java, archivovaná) | Bane Remastered (Rust) |
|---|---|---|
| Zbraně | 9 | 17 |
| Nepřátelé | 10 druhů včetně bossů | 31 druhů, 7 z nich jsou bossové |
| Artefakty | 9 | 13, vytvořené znovu |
| Mezi runy | Progress room a vylepšení | Laboratoř, talenty, stůl se zbraněmi, mise a výzvy |
| Platforma | Windows | Stavěno a testováno na macOS |

Některé věci z první Bane tu zatím nejsou: affixy pater, co-op multiplayer a Discord Rich
Presence. Chybí i Modern shotgun a Grenade launcher.

## Ovládání

| | |
|---|---|
| **Pohyb** `W` `A` `S` `D` | **Míření a střelba** myš |
| **Úhyb** `Space` | **Přebití** `R` |
| **Změna zbraně** `1`-`9` nebo kolečko | **Interakce** `E` |
| **Artefakt** `F` | **Mapa** `Tab` |

Všechny klávesy jde změnit v nastavení.

## Jak to spustit

```bash
git clone git@github.com:Empatixx/Bane-remastered.git
cd Bane-remastered/rust
cargo run --release
```

Potřebuješ [Rust toolchain](https://rustup.rs). Hru jsme testovali jen na macOS.

## Wiki

Wiki ke hře je online: **https://empatixx.github.io/Bane-remastered/docs** (česky). Zdroj je ve
složce [`docs/`](docs/). Popisuje každou zbraň, nepřítele, artefakt a biom.

## Licence

Bane je svobodný software pod licencí [GNU GPL v3](LICENSE).

---

<details>
<summary>Pro vývojáře</summary>

Bane je napsaná v Rustu: `wgpu` a `winit` pro grafiku, `hecs` pro ECS, `rapier2d` pro fyziku,
`kira` pro zvuk. Simulace běží pevně na 60 Hz.

- [`rust/`](rust/): hra. Headless screenshoty, ladicí přepínače (`BANE_THEME`, `BANE_AT`,
  `BANE_SEED`) a poznámky k ECS jsou v [`rust/README.md`](rust/README.md) a [`rust/docs/`](rust/docs/).
- [`tools/`](tools/): Python skripty, které dělají spritey, zvuky a atmosféru.
- [`story.md`](story.md): příběh a kam směřuje.

</details>
