## OTA

- Software je stážen bez připoejní k zařízení k PC nebo k paměťové kartě.
- Používá se k distribuci OS nebo softwarových updatů, enkryptovaných klíčů, konfiguraci
- Možné pro mobily i SIM/WIM karty
- Typicky přes broadcast nebo Wi-Fi síť (může být i bluetooth u starých platform)
- OTA a HTMl stránky používá se na dynamické stránky a zdroje, můžou být použity localstorage data.

## PWA (Progressive Web Applications)

- Výhody:
- Aktualizace na pozadí
- Podpora offline zobrazení a cachování dat
- Nevýhody:
- Offline zkušenost nemusí sedět s nativními appkama
- Horší výkon než hybridní appky

## HTML 5 a mobilní aplikace

- Poskytuje high-level GUI (předdefinované známé UI elementy, basic input typy)
- Poskytuje low-level GUI (výstup zobrazen graficky Canvasem, vstup od uživatele dotek, klavesové stisknutí, myš)
- Umožňuje uložit data (konfigurace, trvalé data)
- Umožňuje komunikaci (HTTP, sockety..)

## HTML 5 uložiště

- localStorage - trvalé data, bez expiračního datumu, klíč->hodnota, omezen velikostně, pro větší data se musí použit
  IndexedDB
- sessionStorage - dočasné data, pouze pro momentální session prohlížeče(ztráceno po restartu prohlížeče), klíč->hodnota
- WebSQL - neudržované, SQLite databáze, 5MB limit, každá db má svou verzi(double)
- IndexedDB - objektově orientovaná databáze, 5MB limit, skoro žadná podpora pro mobily
- File API (read only)
- File API: Writer - Apache Cordova, limitovaná podpora

## HTML5 Orientace

- používají se souřadnice osy (x, y, z) a úhly (alfa, beta, gamma)
- listener "deviceorientation" - vrácí úhly
- listener "devicemotion" - vrácí akceleraci (x,y,z), rychlost rotace (alfa,beta,gamma)
- každý OS (android, iOS, a další...) mají různé počáteční rotace a rotační směry.

## HTML5 Geolokace

- Zobrazení pomoci Wi-FI, GSM, GPS
- Pro např. zobrazování lokace na mapě, navigace, zobrazení blízkých obchodů
- Uživatel musí povolit přístup k lokaci

```javascript
navigator.geolocation.getCurrentPosition(function (position) {
    console.log('Latitude:', position.coords.latitude);
    console.log('Longitude:', position.coords.longitude);
});
```

- getCurrentPosition - získá aktuální pozici
    - latitude - zeměpisná šířka, v úhlech
    - longitude - zeměpisná délka, v úhlech
    - accuracy - přesnost
    - altitude - nadmořská výška

## HTML5 Multimedia

### Audio - tag <audio>
- Nejvíce podporováno formáty: MP3, AAC, OGG
  - Atrributy:
  - loadstart - začátek načítání
    - playing - začátek přehrávání
    - waiting - čekání na data
    - play - začátek přehrávání
    - pause - pauza 

### Video - tag <video>
- Nejvíce podporováno formáty: MP4, WebM, OGG
  - Attributy:
      - autoplay - automatické přehrávání
      - duration - délka videa
      - loop - opakování
      - controls - zobrazí ovládací prvky
      - muted - ztlumení zvuku

## Další HTML 5 API (experimentální, podporovano většinou na androidu 5.0+ a Google Chrome)

- Vibration API - největší podpora

```javascript
navigator.vibrate(time_ms)
```

- USB, přístup k zařízením připojené k mobilu (OTG)
- Bluetooth - bluetooth komuniace přes WebBluetooth
- Sensor API - gyroscope
- Battery API

## Apache cordova

- Web orientovaný (HTML5, Javascript, CSS)
- Používá se na vytvoření vestavěných aplikací s instalačními balíčky misto mobilní aplikace
- Používý klasický javascript API, používá věci z HTML

### Nevýhody

- Pomalejší než native appky (hlavně u graficky intezivních her)
- Není plně integrované do všech platform - chybí předpřipravené widgety, přechody, styly.

### Dva způsoby tvorby:

- Nativní platforma - platform-nezávislé věci v HTML, JS, CSS, ale projekt je tvořen na jednu
  specifickou platformu. Jednodušší udělat low-level změny na specifickou platformu.
- Web projekt - Multiplatformní, Skoro nelze dělat low-level změny na specifické platformy.

### Apache Cordova pluginy

- Battery status
- Camera
- Connection - přístup k Wi-fi informacím
- Device - Informace o HW, SW..
- Device Motion - akcelerometr (nahrazen listenerem "devicemotion")
- Device Orientation - kompras (bude pravděpodobně nahrazen)
- Geolocation - GPS
- Vibration

## Uniform Resource Identifier (URI)

### < scheme >:<address+path> [?<query>] [ #<fragment>]

- Scheme - identifikuje jak je spojení vytvořeno (http, socket, file, sms..)
- Address - identifikuje, kde se připojit
- Ostatní (query, fragment..) - identifikuje potřebné informace schématem

- Používá se TCP - bezpečné, neencryptované, potvrzované
- Posílá se na port HTTP: 80, HTTPS: 443, zdroj většinou bývá random

```http request
HTTP Request has following syntax:
<METHOD> <URL/PATH> HTTP/1.<version>
<Client header
1
>
…
<Client header
n
>
<Empty line>
<Client data, if any>
```

HTTP Response has following syntax:

```http request
HTTP/1.<version> <3-digit error code> <Message>
<Server header
1
>
…
<Server header
m
>
<Empty line>
<Datafromserver,ifany>
```

- HTTP metody: GET(čtení), POST(zápis), PUT(nahrazení), DELETE(smazání), OPTIONS(možné metody)
- HTTP kódy: 1xx (informace), 2xx (úspěch), 3xx (přesměrování), 4xx (chyba klienta), 5xx (chyba serveru)
- Známé kódy: 200 (OK), 302 (Found), 304 (Not Modified), 401 (Unauthorized), 403 (Forbidden), 404 (Not Found), 500 (
  Internal Server Error)

## JSON

- Javascript Object Notation
- Objekt je uložen jako string JavaScript syntax
- Používá se pro výměnu dat mezi serverem a klientem

```javascript
JSON.parse() - převede
JSON
string
na
objekt
JSON.stringify() - převede
objekt
na
JSON
string
```

## AJAX

- Asychronní požadavky bychom měli použivat při:
    - Chceme dělat jiné věci, než čekat na odpověď (např. vyvolání zbytku skriptu)
    - Chceme udělat aplikaci responzivní
    - Jakmile je odpověď dostupná, chceme ji zpracovat
- OnReadyStateChange event - volá se když se změní stav objektu XMLHttpRequest
    - 0 (Uninitialized) - požadavek nebyl inicializován
    - 1 (Open) - server spojení bylo navázáno
    - 2 (Sent) - požadavek byl odeslán
    - 3 (Loading) - požadavek je zpracováván (obsahuje částečná data)
    - 4 (Complete) - požadavek byl zpracován a spojení bylo uzavřeno
  ### CORS
    - Cross-Origin Resource Sharing
    - Bezpečnostní mechanismus, který zabraňuje webovým stránkám v přístupu k zdrojům jiného původu
    - Povoluje se pomocí HTTP hlaviček
    - Když je třeba zavolá se pre-flight request (OPTIONS), která ověří CORS přístup
    - Hlavičky:
        - Access-Control-Allow-Origin - povolené zdroje
        - Access-Control-Allow-Methods - povolené metody
        - Access-Control-Allow-Headers - povolené hlavičky
        - Origin - zdrojová adresa

## Web sockety

- Full-duplex komunikace, TCP, přes HTTP(s)
- Musí být podporováno na obou stranách
- URL schéma: ws://, wss:// (secure)

```javascript
var ws = new WebSocket('ws://www.example.com/socketserver');
ws.onopen = function () {
    ws.send('Hello, Server');
};
ws.onmessage = function (e) {
    console.log('Message:', e.data);
};
ws.onclose = function () {
    console.log('Socket closed');
};
```

## Server-Sent Events (SSE)

- Web stránka dostává automaticky aktualizace od serveru
    - Server musí posílat hlavičku Content-Type: text/event-stream
- Server musí posílat data ve formátu:
  - data: <data>
  - event: <event>
  - id: <id>
  - retry: <ms>
- data - data zprávy
- event - název eventu
- id - ID zprávy
- retry - časový interval, kdy se má zpráva poslat znovu, pokud se ztratí

```javascript
var evtSource = new EventSource('myevents.php');
evtSource.onmessage = function (e) {
    console.log('Message:', e.data);
};
evtSource.onerror = function (e) {
    console.log('Error:', e);
};

evtSource.readyState - 0(CONNECTING), 1(OPEN), 2(CLOSED)
```

## Low-level GUI

- Každá mobilní platforma potřebuje přístup vykreslit grafiku např. nějaké 2D hry
- Před HTML5 se použival Flash
- 3 Způsoby:
    - Canvas - 2D rastrová grafika, nejvíce podporováno
    - WebGL - 3D rastrová grafika, OpenGL ES 2.0, limitovaná podpora
    - SVG - 2D vektorová grafika

### Low level eventy

- Touch events - touchstart, touchmove, touchend, touchcancel
- Mouse events - mousedown, mouseup, mousemove, mouseover, mouseout
- Keyboard events - keydown, keyup, keypress

### Canvas rendering

Vykreslí čtverec:

```javascript
var canvas = document.getElementById('myCanvas');
var ctx = canvas.getContext('2d');
ctx.fillStyle = 'red';
ctx.fillRect(10, 10, 100, 100);
```

Vykreslí čáru:

```javascript
var canvas = document.getElementById('myCanvas');
var ctx = canvas.getContext('2d');
ctx.strokeStyle = 'blue';
ctx.lineWidth = 5;
ctx.beginPath();
ctx.moveTo(10, 10);
ctx.lineTo(100, 100);
ctx.stroke();
```

Vykreslí text:

```javascript
var canvas = document.getElementById('myCanvas');
var ctx = canvas.getContext('2d');
ctx.font = '30px Arial';
ctx.fillStyle = 'green';
ctx.fillText('Hello, World!', 10, 50);
```

Vykreslí kolečko:

```javascript
var canvas = document.getElementById('myCanvas');
var ctx = canvas.getContext('2d');
ctx.strokeStyle = 'red';
ctx.lineWidth = 5;
ctx.beginPath();
ctx.arc(50, 50, 40, 0, 2 * Math.PI);
ctx.stroke();
```

## Základní pojmy game frameworku

- Layers - vrstvy, které se vykreslují (mapa, hráč, nepřátelé, itemy)
    - Sprites - animované objekty
- Scene/stage - spojí vrstvy dohromady, aby např. tvořili level

## Game framworky

### Quintus

- Detekce koliz, sprite animace, zvuky, tile vrstvy, input

### Phaser

- 2D Canvas nebo WebGL
- Preloading obrazků, zvuků
- Particle systém
- Detekce koliz, sprite animace, zvuky, fullscreen, input

### Kinetic.js

## HTML5 Časovače

- Jednorázový časovač:

```javascript
var timer = setTimeout(function () {
    console.log('Timeout');
}, 1000);
```

- Opakovaný časovač:

```javascript
var interval = setInterval(function () {
    console.log('Interval');
}, 1000);
```

## Web Workers

- Vytvoří se nový vlákno, které může běžet vedle hlavního vlákna
- Používájí se zprávy pro komunikaci mezi vlákny

## Typy vyměnovacích formátů

- XML

```xml
<?xml version="1.0" encoding="UTF-8"?>
<gpx xmlns="http://www.topografix.com/GPX/1/1"
     xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
     creator="Example-builder" version="1.1"
     xsi:schemaLocation="http://www.topografix.com/GPX/1/1
http://www.topografix.com/GPX/1/1/gpx.xsd">
    <metadata/>
    <wpt lat="50.0000" lon="17.1717">
        <name>Sample point</name>
        <cmt>Just a sample &amp; nothing more</cmt>
        <desc>An example of XML file to use for GPS app</desc>
        <sym>Dot</sym>
        <type>Reference Point</type>
    </wpt>
</gpx>
```

- JSON
- CSV
- HTML
- Contracts (vCard, MECARD)
```
BEGIN:VCARD
VERSION:2.1
N:Moravec;Pavel;;Ing., Ph.D.
FN:Pavel Moravec
ORG:VSB-TU Ostrava
TITLE:Assistant professor
NOTE:Know your lecturer!
TEL;WORK:+420597325896
TEL;CELL:+420608******
ADR;WORK:;;EA 409, FEECS;VSB-TU Ostrava;17. listopadu 15, Ostrava - Poruba
URL;WORK:http://homel.vsb.cz/~mor03/
EMAIL;PREF:pavel.moravec@vsb.cz
REV: 20140421T231132+0200
END:VCARD
```
```
MECARD:
N:Ing. Pavel Moravec Ph.D.;
TEL:+420596995896;
EMAIL:pavel.moravec@vsb.cz;
ADR:EA409, FEECS, VSB-TU Ostrava, 708 00 Ostrava-Poruba;
```
- Calendar (iCalendar, ATOM feed)
```
BEGIN:VCALENDAR
PRODID:-//BUSINESS-CALENDAR//APPGENIX-SOFTWARE//
VERSION:2.0
CALSCALE:GREGORIAN
METHOD:PUBLISH
X-WR-TIMEZONE:Europe/Prague
BEGIN:VEVENT
SUMMARY:TAMZI – en
UID:6:38@google.com
ORGANIZER;CN=pavel.moravec@vsb.cz:mailto:pavel.moravec@vsb.cz
DTSTART;TZID=Europe/Prague:20140422T141500
DTEND;TZID=Europe/Prague:20140422T154500
DTSTAMP:20140422T064437Z
CREATED:20140221T091422Z
LAST-MODIFIED:20140422T064437Z
LOCATION:JA401
RRULE:FREQ=WEEKLY;INTERVAL=1;BYDAY=TU;UNTIL=20140518T121500Z
STATUS:TENTATIVE
BEGIN:VALARM
DESCRIPTION:This is an event reminder
ACTION:DISPLAY
TRIGGER:-P0DT0H15M0S
END:VALARM
END:VEVENT
END:VCALENDAR
```
## Obfuskace
- Shrinking - odstranění nepoužitých funkcí, proměnných, komentářů
- Optimization - zmenší kód, zjednoduší, zrychlí
- Obfuscation - změní kód, aby byl těžší na pochopení(čtení)
- Proguard - nástroj pro obfuskaci, optimalizaci a zmenšení kódu
### Minifikace
- Odstranění bílých znaků, komentářů, zkrácení proměnných
- Zmenšení velikosti kódu


## Co chybí v HTML5 pro mobily
- Žadná možná komunikace s:
  - Seriové porty
  - USB
  - Bluetooth
- Multi-threading
  - Web workeři nejsou všude podporováni
  - Nemůžeme použit všechny CPU jádra
- Není možné spustit nativní kód bez použití Apache Cordova