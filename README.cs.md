<div align="center">

# BANE

### Pet pater dolu. Kazde patro si te pamatuje.

Rychla top-down roguelike strileckou s rucne kreslenou pixel artovou grafikou. Uhybej, strilej, umri, zesil a jdi hloub.

![Bane trailer](media/trailer.gif)

[Cely trailer v plne kvalite](media/trailer.mp4)

[English](README.md) | Cesky

</div>

---

## Bane je otevrena. Vstup.

Neco roztrhlo svet. Jeho patra nikdy nevypadaji stejne. Jsi Elian, poslany dolu portalem. Dole na
tebe cekaji krysy, slizy, pavouci a veci s prilis mnoha ocima. Na konci kazdeho patra stoji boss,
ktery byl kdysi nekym.

Zacinas s pistoli, brokovnici a uhybem. Vsechno ostatni si musis vzit.

## Proc jeste jeden run

**16 zbrani.** Pistol, Uzi, Sniper, Flamethrower, Tesla Gun, RPG, Railgun a dalsi. Kazda zbran ma
vlastni strom vylepseni a na jeho konci mutaci.

**7 bossu se jmenem a pribehem.** Pekelny pes, ktery ceka na dne, pisar, ktery zapsal kazdou
modlitbu, ozivla truhla, kral pavouku. Kazdy boss ma vlastni utoky a zuri, kdyz prohrava.

**Pokazde novy dungeon.** Mistnosti, pasti, obchody, poklady a obetni oltare se generuji znovu pro
kazde patro. Je tu sedm biomu: Stone, Sewer, Moss, Spider, Ice, Lava a Temple.

**Loot, ktery meni run.** 13 artefaktu na jedno pouziti, obchod, hraci automat, perky a mazlicek,
ktery sbira mince. Vsad na zbran, plat za ni krvi, nebo setri.

**Zesilujes, i kdyz umres.** Mezi runy chodis po laboratori. U Vedce utracis talenty, u stolu se
zbranemi vylepsujes zbrane, na nastence berres mise a vyzvy.

**Vypada dobre.** Rucne kreslena pixel art grafika, dynamicke svetlo, jina nalada v kazdem biomu,
uhyb, ktery ma spravny pocit, otresy obrazovky a zablesky pri zasahu.

## Podivej se

### Sedm biomu

| | | |
|:-:|:-:|:-:|
| ![Stone](media/biome-stone.png)<br>**Stone** | ![Sewer](media/biome-sewer.png)<br>**Sewer** | ![Moss](media/biome-moss.png)<br>**Moss** |
| ![Spider](media/biome-spider.png)<br>**Spider** | ![Ice](media/biome-ice.png)<br>**Ice** | ![Lava](media/biome-lava.png)<br>**Lava** |
| ![Temple](media/biome-temple.png)<br>**Temple** | | |

### Mistnosti

| | | |
|:-:|:-:|:-:|
| ![Boss](media/room-boss.png)<br>**Kazde patro konci bossem** | ![Obchod](media/room-shop.png)<br>**Utrat mince** | ![Poklad](media/room-treasure.png)<br>**Poklad pro odvazne** |

## Ovladani

| | |
|---|---|
| **Pohyb** `W` `A` `S` `D` | **Mireni a strelba** mys |
| **Uhyb** `Space` | **Prebiti** `R` |
| **Zmena zbrane** `1`-`9` nebo kolecko | **Interakce** `E` |
| **Artefakt** `F` | **Mapa** `Tab` |

## Jak to spustit

```bash
git clone git@github.com:Empatixx/Bane-remastered.git
cd Bane-remastered/rust
cargo run --release
```

Potrebujes [Rust toolchain](https://rustup.rs).

## Wiki

Wiki ke hre je online: **https://empatixx.github.io/Bane-remastered/docs** (cesky). Zdroj je ve slozce [`docs/`](docs/). Popisuje kazdou zbran, nepritele, artefakt a
biom. Stranky jsou obycejne Markdown soubory v [`docs/content/`](docs/content/), takze si je
prectes rovnou na GitHubu.

## Licence

Bane je svobodny software pod licenci [GNU GPL v3](LICENSE).

---

<details>
<summary>Pro vyvojare</summary>

Bane je napsana v Rustu: `wgpu` a `winit` pro grafiku, `hecs` pro ECS, `rapier2d` pro fyziku,
`kira` pro zvuk. Simulace bezi pevne na 60 Hz. Hra puvodne vznikla v Jave a byla prepsana do Rustu.
Java verze je porad v historii gitu.

- [`rust/`](rust/): hra. Headless screenshoty, ladici prepinace (`BANE_THEME`, `BANE_AT`,
  `BANE_SEED`) a poznamky k ECS jsou v [`rust/README.md`](rust/README.md) a [`rust/docs/`](rust/docs/).
- [`tools/`](tools/): Python skripty, ktere delaji spritey, zvuky a atmosferu.
- [`story.md`](story.md): pribeh a kam smeruje.

</details>
