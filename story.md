# Bane — Příběh

> Směr **„Pokusný subjekt“** (schváleno). Dokument je příběhová bible: lore, postavy,
> kapitoly po patrech, konkrétní herní texty a mapování na kód.
> Texty ve hře jsou anglicky (jako dnes), poznámky česky.
> Sekce 8 shrnuje stav hry, ze kterého příběh vychází.

---

## 1. Premisa jednou větou

Jsi pokusný subjekt. Vědec tě posílá portálem do **Bane** — rány ve světě, která se po
každém sestupu zahojí a přeskládá — a platí ti talenty za každý kus pravdy, který z ní
vyneseš. Nejsi první. A nejsi tak docela člověk.

---

## 2. Svět

### Bane
Ne jeskyně, ale **rána**. Před dávnými lety se pod chrámem na kopci něco protrhlo. Co do
rány spadne, to překroutí: krysy, hadi, netopýři, pavouci rostou a mění se. Co v ní
zemře, to vstřebá. A co z ní vyleze, to si nepamatuje, odkud přišlo.

Rána se **hojí**: po každém průchodu se patra přeskládají (= procedurální generování).
Žebříky dolů jsou starší než Vědec — vytesali je lidé z chrámu, kteří se pokoušeli ránu
zavřít. Patro 1 je vždy pavoučí: pavouci jsou „stehy“, které ránu drží pohromadě u
povrchu.

### Laboratoř (lobby)
Stojí přímo nad ranou, postavená z trosek chrámu. Portál je Vědcův vynález: jediná
stabilní cesta dovnitř a zpět. Nástěnka s úkoly jsou jeho pracovní poznámky.
Stůl se zbraněmi je sbírka toho, co předchozí subjekty vynesly nahoru.

### Kletba
Bane = jméno rány i kletby v ní. Kletba chce jediné: **růst**. Obětní oltář v podzemí je
místo, kde si vezme kus tebe přímo (platíš zdravím). Talenty, které hráč sbírá, jsou
ve skutečnosti kousky kletby, které Vědec „čistí“ a vrací subjektu jako vylepšení.

---

## 3. Postavy

### Hráč — Subjekt
- Bez paměti před laboratoří. Probudil se u portálu, Vědec mu dal pistoli a brokovnici.
- Má číslo. Hráč ho zjistí z death screenu (viz 6). Navrhované číslo: **41**.
- Pravda (odhalená až v pravém konci): subjekty nejsou lidé z povrchu. Vědec je
  **vytahuje z rány** — jsou to to, co kletba vyplivne, když ji dost nakrmíš. Subjekt 41
  je zatím nejstabilnější. Proto mu Vědec říká „my finest work“.

### Vědec
- Upgrade NPC v lobby. Nadšený, laskavý, zdrobněliny. Nikdy nelže přímo, jen zamlčuje.
- Motivace: nechce ránu zavřít, chce ji **ovládnout**. Kletba je pro něj nekonečný zdroj.
- Jeho repliky se mění s *nejhlubším dosaženým patrem* a *počtem smrtí* (viz 5).

### Obchodník (shop room)
- Subjekt **č. 12**. Jediný, který se rozhodl nevrátit nahoru. Žije v ráně, prodává to,
  co najde. Unavený, suchý humor. Ví, co subjekty jsou, ale řekne to až po několika
  návštěvách.

### Oltář (sacrifice room)
- Hlas kletby. Mluví velkými písmeny, bez interpunkce, ve druhé osobě.
- Jediná postava, která hráči říká pravdu od začátku — jen jí nikdo nerozumí.

### Vlk (pet)
- Jediné, co Vědec nestvořil. Vylezl z rány sám a sedí u portálu. Odemyká se talenty,
  ale Vědec o něm mluví nerad („It followed you. I did not plan that.“).

### Bossové — strážci
| Boss | Co je zač |
|---|---|
| **Spider king** | Steh. Drží ránu u povrchu zavřenou. Zabít ho = otevřít cestu dolů. |
| **King slime** | To, co zbylo ze subjektů, které se rozpustily. |
| **Sewer ogre** | Chrámový dělník, který kopal odvodnění. Prvni, koho rána vzala. |
| **Stone golem** | Socha strážce z chrámu. Oživená, bez rozkazu, hlídá ze zvyku. |
| **Argus** | Vědcovo **oko**. Postavil ho, aby sledoval subjekty. Vymklo se. |
| **Hellhound** | Strážce dna. Byl tu před chrámem i před Vědcem. Nehlídá ránu před tebou — hlídá tebe před tím, co je pod ní. |

---

## 4. Kapitoly: patra

Biomy se losují (kromě patra 1), proto je příběh vázaný na **bosse a hloubku**, ne na
biom. Jediná pevná změna: **patro 5 = Temple + finální boss**.

| Patro | Boss | Co hráč pochopí | Zdroj informace |
|---|---|---|---|
| 1 | Spider king | Rána se hojí, pavouci jsou stehy. | Vědec před runem, obchodník. |
| 2 | King slime / Sewer ogre | Tady skončila většina subjektů. | Obchodník, nástěnka. |
| 3 | Stone golem / Argus | Vědec tě sleduje. Argus je jeho. | Vědec po smrti Arguse. |
| 4 | Hellhound | Hellhound tě nechce zabít, chce tě zastavit. | Oltář, obchodník. |
| 5 | **The Heart** (finální) | Dno rány. Vědec ji chce použít, ne zavřít. | Vědec po výhře. |

### Finální boss: The Heart
Rozhodnuto: **nový boss**, ne druhá fáze. Pracovní návrh:
- Nehybné jádro uprostřed arény (jako EyeWatcher v měřítku bosse) + fáze, ve kterých
  přivolává „stehy“ (pavoučata) a „zbytky“ (slizy) — tedy recykluje existující nepřátele.
- Fáze 3: z jádra vylezou **siluety předchozích subjektů** (hráčův sprite s jinou
  paletou), které střílí pistolí. Tím boss řekne bez slov, co hráč je.
- Po smrti: artefaktová truhla (jako dnes) + victory.

---

## 5. Herní texty

### 5.1 Vědec — lobby (`systems/scientist.rs`)
Dnešní `GREETINGS` a `HINTS` zůstávají. Přidávají se řádky podle profilu:

**Podle nejhlubšího patra (`deepest`):**
```
0  "The portal is ready. Just step in."
1  "The spiders? They hold the wound shut. Cut through."
2  "Floor two! Most of them never got past two."
3  "You saw the Eye? Ignore it. It is... mine."
4  "The hound. Do not listen to it. Dogs lie."
5  "You reached the bottom. Now we can begin."
```

**Podle počtu smrtí (`deaths`):**
```
1   "Back already! Good. Good. Nothing is lost."
5   "Five times. You are holding together nicely."
10  "Ten! Twelve took forty. You are my finest work."
20  "Still you. Still whole. Remarkable."
```

**Po nákupu upgradu** (`PURCHASE`, dnes „Excellent choice!“ — rozšířit o rotaci):
```
"Excellent choice!"
"Cleaned it myself. Mostly."
"That one came up with subject thirty."
```

**Po výhře:**
```
"It is open. It is finally open!"
"Rest. Tomorrow we go down together."
```

**Po splnění challenge *Bane*** (hráč se vrátil bez „pomůcek“):
```
"You went without my work. And came back."
"...What did it show you?"
```

### 5.2 Obchodník (`systems/shop.rs`)
Cyklus podle počtu návštěv v profilu (`shop_visits`), pak náhodně:
```
1  "New one? Coins on the table. No credit."
2  "He calls you subject? He called me twelve."
3  "I stopped going up. The air up there costs more."
4  "The slimes. You know what they are? Don't eat them."
5  "The hound let me pass once. I think it felt sorry."
6  "You don't remember the surface. None of us did."
7  "He doesn't pull us from up there, friend. He pulls us from down here."
8  "Enjoy your purchase. Enjoy anything."
```
Dnešní „Enjoy your purchase!“ zůstává jako reakce na nákup.

### 5.3 Oltář (sacrifice room)
Jedna věta při vstupu do místnosti, náhodně:
```
"YOU ARE MINE ALREADY"
"HE ONLY BORROWS YOU"
"GIVE A PIECE  I GIVE IT BACK  LATER"
"FORTY BEFORE YOU  ALL CAME HOME"
"THE DOG KNOWS"
```

### 5.4 Nástěnka — missions (`progress/missions.rs`)
Názvy a popisy zůstávají mechanické (Crypt Cleaner, Deep Delver…). Přidává se **jeden
řádek Vědcovy poznámky** per goal kind, zobrazený malým písmem pod popisem:
```
Kills       "Fewer of them, more of you."
KillsOf     "I need to know what it does to this kind."
Boss        "The seals. Break the seals."
Floor       "Go deeper. Bring back what you see."
GoldInRun   "Coins hold the curse nicely. Carry them."
Gold        "Every coin is a piece of it. Keep collecting."
Items       "Things come up changed. I want to see how."
Untouched   "Let nothing touch you. I need you clean."
```

### 5.5 Nástěnka — challenges (`progress/challenges.rs`)
Popisy přepsat jako Vědcovy rukopisné poznámky:
| Id | Nový popis |
|---|---|
| glass_cannon | "One hit and you break. But oh, how you hit." |
| pistol_only | "Just the pistol. Just like your first day." |
| horde | "I fed it a little. Let us see what grew." |
| merchants_strike | "Twelve is not to be trusted. Go without him." |
| two_left_feet | "No dodging. I want a clean sample." |
| **bane** | "Go without my work. Show me what you are." |

Challenge *Bane* je příběhově klíčová: je to run bez Vědcových „pomůcek“ a jen po něm se
odemkne pravý konec (viz 6).

### 5.6 Death screen (`prefabs/ui.rs`, `systems/death_screen.rs`)
Pod „Game Over“ jeden řádek:
```
"Subject 41. Attempt {deaths}. Recorded."
```
Pod „Victory!“:
```
"Subject 41. The wound is open. Recorded."
```

---

## 6. Konce

### Základní vítězství
Hráč porazí The Heart. Rána je „otevřená“ = kletba je bez strážce. V lobby Vědec jásá
(„Now we can begin.“). Hráč se vrací do normální smyčky. Nic se nezměnilo, jen Vědec
už nemluví o testech, ale o „our work“. Pocit: vyhrál jsi, ale pro někoho jiného.

### Pravý konec
Podmínka: dokončit challenge **Bane** (patro 5 bez Vědcových pomůcek). Při zabití The
Heart v tomto runu se místo truhly objeví **portál dolů** (druhá barva). Vstup = krátká
textová sekvence (3–4 obrazovky, stejný styl jako death screen):
```
"Below the heart there is no floor."
"Only the others. Forty of them. Waiting to be pulled up again."
"You remember now. You were never brought here. You were brought UP."
```
Volba (dvě tlačítka):
- **Close the wound** — profil dostane příznak `closed`. Lobby se změní: portál je
  vyhaslý, Vědec sedí a mlčí, vlk u portálu. Hráč může start „new game +“: nový profil s
  jedním bonusem (např. zachovaný vlk). *Konec hry.*
- **Leave it open** — nic se nemění, hráč sbírá talenty dál. Vědec: „Good. Good. We
  understand each other.“ *Pokračování.*

---

## 7. Mapování na kód (co je potřeba implementovat)

| Příběhový prvek | Kde | Rozsah |
|---|---|---|
| Vědcovy repliky podle `deepest` / `deaths` | `systems/scientist.rs`, `profile.rs` (čítače už existují v `record_run`) | S |
| Obchodníkův cyklus podle návštěv | `systems/shop.rs`, `profile.rs` (`shop_visits`) | S |
| Oltářní věta při vstupu | `systems/rooms.rs` (room enter event) + `say` | S |
| Poznámka pod mission | `progress/missions.rs` (`note()`), `scenes/quest_board.rs` | S |
| Nové popisy challenges | `progress/challenges.rs` | S |
| Řádek na death/victory screenu | `prefabs/ui.rs` | S |
| Patro 5 = Temple | `dungeon/theme.rs::for_floor` | S |
| The Heart boss | `enemies.rs`, `systems/bosses.rs`, nový `systems/heart.rs`, sprite | L |
| Pravý konec: portál dolů + textová sekvence + volba | `scenes/ingame.rs`, nový `scenes/epilogue.rs`, `profile.rs` (`closed`) | M |
| Lobby po zavření rány | `scenes/lobby.rs` (vyhaslý portál, tichý Vědec) | S |

Doporučené pořadí: řádky S nejdřív (texty jsou levné a dají celou atmosféru), pak patro
5 + The Heart, pravý konec nakonec.

---

## 8. Stav hry, ze kterého se vychází

- Single‑player 2D top‑down roguelike shooter, Rust port v `rust/`, větev
  `missions-challenges`.
- Run: lobby → portál → patra 1–5 → boss na patře 5 = `victory` (`FINAL_FLOOR = 5`).
- Patro: start, 5 bojových místností, shop, treasure, boss, volitelně sacrifice.
- Biomy: patro 1 vždy Spider, dále losované Stone/Sewer/Moss/Ice/Lava/Temple.
- Lobby: Vědec (upgrady za talenty), portál, nástěnka missions/challenges, stůl se
  zbraněmi.
- Bossové: Spider king, King slime, Stone golem, Argus, Hellhound, Sewer ogre.
- Existující texty: Vědec (`GREETINGS`, `HINTS`, `PURCHASE`), shop („Enjoy your
  purchase!“), missions (názvy + popisy), death screen („Game Over“ / „Victory!“).

---

## 9. Úvod: chrám, rána a řád Pečeti

Úvodní cinematic (`scenes/cinematic.rs`: den v chrámu, pečeť, noc, trhlina, tvorové, krypta)
a výcviková krypta vyprávějí, co se stalo předtím, než
hráč poprvé vkročí do laboratoře.

### Říše Velmor
Chrám stojí na kopci nad říší **Velmor**. Čtyři sta let k němu lidé z Velmoru každý den chodili
se modlit. Nevěděli proč. Věděli jen, že se to tak dělá a že dokud se modlí, je klid. Modlitby
lidu a kněží ve skutečnosti držely spící zlo pod kopcem: pod oltářem je **pečeť**, která
slabě dýchá fialovým světlem, dokud ji modlitby živí.

Jednou v noci modlitby utichly (proč, zůstává otázkou pro pozdější příběh, například Teodorova
kronika). Pečeť povolila a pod chrámem se otevřela trhlina, Bane.

### Řád Pečeti
Chrám na kopci patřil **řádu Pečeti** (*Order of the Seal*). Řád věděl, že pod kopcem něco spí,
a staletí to držel modlitbou a ohněm. V kryptě pod chrámem uložil **posledního strážce**: tělo
v kamenném sarkofágu, které se má probudit, až pečeť povolí. Kdo strážce je, řád nevěděl. Jen
že „přišel zdola a čeká“.

Kněží řádu nosí bílá roucha s kapucí a zlatým lemem. Jsou kreslení v nízkém rozlišení
(10×19 px, 4× zvětšené, `tools/make_priest.py`), stejně hrubě jako rekvizity v dungeonu.

### Akolyté (v cinematicu)
| Jméno | Úloha v řádu | Osud |
|---|---|---|
| **Bratr Aldric** | Nejstarší, strážce oltářního ohně. Klečí nejblíž oltáři. | Utíká poslední, chytí ho velký pavouk. |
| **Oswin** | Zvoník, nejmladší z řádu. | Chytí ho krysa u dolní zdi. |
| **Teodor** | Písař, vedl kroniku krypty. Jeho kronika se později najde v dungeonu (nápad: sběratelné stránky). | Chytí ho pavouček u horní zdi. |
| **Cassian** | Strážce brány. | Chytí ho pavouček, když se snaží dostat ke dveřím. |
| **Elian** | Novic a nosič lampy. Měl na starost svíce v kryptě, proto jako jediný zná cestu dolů. | **Přežije.** Uteče do krypty a probudí strážce. |

### Elian, průvodce
- Je to on, kdo v cinematicu doběhne do krypty, odsune víko a probudí hráče („Rise. Close the
  wound.“).
- Ve výcvikové kryptě hráče provází (dosavadní průvodce v kápi se nahrazuje Elianem). Mluví
  rychle a nervózně, je mladý a vyděšený, ale věří, že hráč je ten slíbený strážce.
- Výcvik končí portálem: Elian hráče posílá nahoru do jediného bezpečného místa, které zbylo,
  do laboratoře, kterou si mezitím z trosek chrámu postavil **Vědec**.
- Repliky (anglicky, ve hře):
  ```
  "You're awake! It's me, Elian. I'm... all that's left of the order."
  "They said you'd come back when the seal broke. It broke."
  "I'll show you what Brother Aldric showed me. Follow me."
  (zkouška) "That's how they took Oswin. Don't let them take you."
  (portál) "Go up. The Scientist says he can close it. I hope he's right."
  ```

### Napojení na zbytek příběhu
- Elian věří, že hráč je strážce řádu. Vědec mu říká **Subjekt 41**. Pravda z pravého konce
  („You were never brought here. You were brought UP.“) platí dál: tělo v sarkofágu nebylo
  strážcem řádu, ale čtyřicátou první věcí, kterou rána vrátila nahoru. Řád ho jen uložil
  a čekal.
- Teodorova kronika (nápad) je přirozený nosič téhle pravdy: stránky v dungeonu postupně
  odhalí, odkud se strážce v kryptě vzal.
- Elian může později stát v laboratoři u vstupu do výcviku (kniha „Training“), jako postava,
  se kterou se dá mluvit.
