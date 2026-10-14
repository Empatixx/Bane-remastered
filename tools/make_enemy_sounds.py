"""Synthesizes the enemy and boss ability sounds into assets/audio/enemies.

Run with: uv run --with numpy --with soundfile python tools/make_enemy_sounds.py [out_dir]

Every sound is a tell or an attack: wind-ups, shots, leaps, slams, summons and rage roars.
They are kept short and well below the gun sounds; the game fades them with distance from
the player and nudges their pitch on every play.
"""

import sys
from pathlib import Path

import numpy as np
import soundfile as sf

from make_gun_sounds import (
    RATE,
    bandpass,
    click,
    decay,
    fade_out,
    highpass,
    lowpass,
    noise,
    normalize,
    place,
    seconds,
    sweep,
)

ENEMIES = Path(__file__).resolve().parent.parent / "assets/audio/enemies"
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else ENEMIES


def saw(phase):
    return 2.0 * ((phase / (2 * np.pi)) % 1.0) - 1.0


def glide(length, start, end, curve=1.0):
    """Phase of a tone gliding from start to end Hz, eased by the curve."""
    t = seconds(length)
    frequency = start + (end - start) * (t / length) ** curve
    return 2 * np.pi * np.cumsum(frequency) / RATE


def swell(length, attack, release):
    t = seconds(length)
    rise = np.minimum(t / max(attack, 1e-4), 1.0)
    fall = np.minimum((length - t) / max(release, 1e-4), 1.0)
    return rise * np.clip(fall, 0.0, 1.0)


def hump(length, power=1.0):
    return np.sin(np.pi * seconds(length) / length) ** power


def jitter(length, seed, rate=30.0, depth=0.5):
    """Slow random wobble around one, for rough voices and grinding stone."""
    steps = max(int(length * rate) + 2, 2)
    points = np.random.default_rng(seed).uniform(-1.0, 1.0, steps)
    positions = np.linspace(0, steps - 1, int(length * RATE))
    return 1.0 + depth * np.interp(positions, np.arange(steps), points)


def growl(length, start, end, seed, rough=0.5, low=120.0, high=1400.0):
    """A throaty voice: a gliding saw shaken by a random wobble, shaped by a band."""
    phase = glide(length, start, end)
    wobble = jitter(length, seed, 28.0, rough)
    voice = saw(phase * 1.0) * wobble + 0.5 * saw(phase * 2.01) * jitter(length, seed + 1, 40.0, rough)
    breath = bandpass(noise(length, seed + 2), low, high) * rough * 0.6
    return bandpass(voice, low, high) + breath


def whoosh(length, seed, low=500.0, high=3000.0, power=1.5):
    return bandpass(noise(length, seed), low, high) * hump(length, power)


def blob(length, start, end, rate):
    return np.sin(glide(length, start, end, 0.5)) * decay(length, rate, 0.003)


def squelch(seed, length=0.3, start=280.0, end=70.0):
    body = blob(length, start, end, 12)
    wet = lowpass(noise(length, seed), 1400) * decay(length, 22, 0.002)
    return body * 0.8 + wet * 1.2


def bubbles(length, seed, count, low=300.0, high=900.0):
    rng = np.random.default_rng(seed)
    track = np.zeros(int(length * RATE))
    for _ in range(count):
        size = rng.uniform(0.03, 0.07)
        pitch = rng.uniform(low, high)
        pop = np.sin(glide(size, pitch, pitch * 1.8, 0.7)) * decay(size, 45, 0.002)
        place(track, pop * rng.uniform(0.4, 1.0), rng.uniform(0.0, length - size))
    return track


def rumble(length, seed, cutoff=180.0):
    return lowpass(noise(length, seed), cutoff) * 3.0


def bell(length, pitch, rate=7.0):
    t = seconds(length)
    tone = np.sin(2 * np.pi * pitch * t) + 0.4 * np.sin(2 * np.pi * pitch * 2.76 * t)
    return tone * decay(length, rate, 0.002)


def ticks(length, seed, count, low, high, accelerate=True):
    track = np.zeros(int(length * RATE))
    rng = np.random.default_rng(seed)
    for i in range(count):
        at = length * (1 - (1 - i / count) ** 2 if accelerate else i / count) * 0.92
        place(track, click(seed + i, rng.uniform(low, high), 0.03, 120) * rng.uniform(0.5, 1.0), at)
    return track


def ghost_dash(seed):
    def make():
        length = 0.45
        t = seconds(length)
        air = whoosh(length, 200 + seed, 700, 3200, 1.2)
        wail = np.sin(glide(length, 900 + 80 * seed, 520) + 4 * np.sin(2 * np.pi * 7 * t)) * hump(length, 1.5)
        return air + wail * 0.25

    return make


def slime_smash(seed):
    def make():
        track = np.zeros(int(0.4 * RATE))
        place(track, squelch(210 + seed, 0.3, 300 + 30 * seed, 75), 0.0)
        place(track, bubbles(0.3, 215 + seed, 3, 400, 900) * 0.35, 0.06)
        return track

    return make


def slime_burst():
    track = np.zeros(int(0.5 * RATE))
    pop = lowpass(noise(0.05, 220), 900) * decay(0.05, 60) * 2.0
    place(track, pop, 0.0)
    place(track, squelch(221, 0.35, 340, 60) * 1.2, 0.0)
    splat = bandpass(noise(0.3, 222), 600, 3000) * decay(0.3, 14) * 0.5
    place(track, splat, 0.03)
    place(track, bubbles(0.35, 223, 4) * 0.3, 0.1)
    return track


def toxic_burst():
    length = 0.8
    track = np.zeros(int(length * RATE))
    place(track, squelch(230, 0.25, 240, 70), 0.0)
    hiss = highpass(noise(0.7, 231), 2200) * swell(0.7, 0.02, 0.6) * 0.35
    place(track, hiss, 0.02)
    place(track, bubbles(0.6, 232, 6, 200, 600) * 0.4, 0.1)
    return track


def kin_heal():
    track = np.zeros(int(0.6 * RATE))
    for i, pitch in enumerate([880.0, 1108.0, 1318.0]):
        place(track, bell(0.4, pitch, 9) * 0.6, i * 0.07)
    return track


def leaper_crouch():
    length = 0.45
    track = ticks(length, 240, 9, 2400, 3800)
    hiss = bandpass(noise(length, 249), 3000, 7000) * swell(length, 0.4, 0.05) * 0.12
    return track + hiss


def leaper_land(seed):
    def make():
        length = 0.3
        thump = lowpass(noise(length, 250 + seed), 300) * decay(length, 22) * 3.0
        body = np.sin(glide(length, 120, 55)) * decay(length, 16)
        track = thump + body * 0.8
        place(track, click(255 + seed, 2600 + 300 * seed, 0.03, 140) * 0.4, 0.0)
        return track

    return make


def eye_charge():
    length = 0.45
    t = seconds(length)
    tone = np.sin(glide(length, 300, 950, 1.3)) * (0.7 + 0.3 * np.sin(2 * np.pi * 18 * t))
    fizz = bandpass(noise(length, 260), 3000, 8000) * 0.12
    return (tone * 0.5 + fizz) * swell(length, 0.3, 0.03)


def eye_shoot(seed):
    def make():
        length = 0.18
        phase = glide(length, 950 + 80 * seed, 300, 0.4)
        tone = np.sin(phase) + 0.25 * np.sign(np.sin(phase * 0.5))
        return lowpass(tone, 5000) * decay(length, 20, 0.002)

    return make


def spider_hatch():
    length = 0.6
    track = np.zeros(int(length * RATE))
    for i in range(4):
        place(track, click(270 + i, 1400 + 250 * i, 0.04, 90) * 0.8, 0.02 + i * 0.035)
    place(track, squelch(275, 0.3, 260, 90) * 0.9, 0.12)
    place(track, ticks(0.3, 276, 6, 3000, 4200, False) * 0.5, 0.28)
    return track


def web_charge():
    length = 0.5
    hiss = bandpass(noise(length, 280), 1800, 6000) * swell(length, 0.42, 0.04) * 0.6
    chitter = ticks(length, 281, 10, 2600, 4000) * 0.5
    return hiss + chitter


def web_spit():
    length = 0.3
    burst = bandpass(noise(length, 290), 700, 4500) * decay(length, 18, 0.002)
    zip_ = np.sin(glide(length, 1300, 380, 0.4)) * decay(length, 14) * 0.5
    track = burst + zip_
    place(track, squelch(291, 0.2, 320, 120) * 0.5, 0.02)
    return track


def king_spiral(seed):
    def make():
        length = 0.09
        tone = np.sin(glide(length, 420 + 60 * seed, 820 + 60 * seed, 0.6)) * decay(length, 30, 0.003)
        return tone + lowpass(noise(length, 300 + seed), 1500) * decay(length, 60) * 0.3

    return make


def king_smash():
    length = 0.7
    track = squelch(310, 0.6, 180, 40) * 1.3
    place(track, lowpass(noise(0.12, 311), 250) * decay(0.12, 25) * 4.0, 0.0)
    place(track, bubbles(0.5, 312, 6, 200, 600) * 0.4, 0.12)
    return track


def golem_slam():
    length = 0.5
    grind = bandpass(noise(length, 320), 180, 1300) * jitter(length, 321, 35, 0.8)
    track = (grind * 0.8 + rumble(length, 322)) * swell(length, 0.4, 0.05)
    return track


def golem_throw():
    length = 0.45
    track = growl(0.32, 75, 60, 330, 0.7, 80, 900) * swell(0.32, 0.05, 0.1)
    track = np.concatenate([track, np.zeros(int((length - 0.32) * RATE))])
    place(track, whoosh(0.3, 331, 300, 2200, 1.0) * 0.8, 0.15)
    return track


def golem_rain():
    length = 0.95
    track = rumble(length, 340, 150) * swell(length, 0.3, 0.3)
    rng = np.random.default_rng(341)
    for i in range(9):
        size = 0.06
        crumble = lowpass(noise(size, 342 + i), rng.uniform(600, 1500)) * decay(size, 40)
        place(track, crumble * rng.uniform(0.6, 1.2), rng.uniform(0.1, length - size))
    return track


def laser_charge():
    length = 1.0
    t = seconds(length)
    phase = sweep(length, 180, 1700, 0.7)
    hum = np.sin(phase) + 0.3 * np.sin(2 * phase)
    fizz = bandpass(noise(length, 350), 3000, 9000) * 0.15 * (t / length)
    return (hum * 0.5 + fizz) * swell(length, 0.15, 0.03)


def laser_fire():
    length = 1.9
    t = seconds(length)
    buzz = lowpass(saw(2 * np.pi * 110 * t) + 0.6 * np.sign(np.sin(2 * np.pi * 221 * t)), 2400)
    shimmer = np.sin(2 * np.pi * 880 * t + 2 * np.sin(2 * np.pi * 6 * t)) * 0.25
    crackle = bandpass(noise(length, 360), 2500, 8000) * (np.random.default_rng(361).uniform(0, 1, len(t)) > 0.7)
    tremolo = 0.8 + 0.2 * np.sin(2 * np.pi * 12 * t)
    return (buzz * 0.4 + shimmer + crackle * 0.3) * tremolo * swell(length, 0.05, 0.35)


def golem_rage():
    length = 1.3
    voice = growl(length, 62, 44, 370, 0.9, 60, 800) * swell(length, 0.15, 0.5)
    stone = rumble(length, 371, 140) * swell(length, 0.05, 0.8)
    return voice + stone * 0.8


def argus_charge():
    length = 0.55
    t = seconds(length)
    chord = sum(np.sin(glide(length, f * 0.6, f, 0.8)) for f in (440.0, 523.0, 659.0))
    tremolo = 0.75 + 0.25 * np.sin(2 * np.pi * 14 * t)
    return chord * tremolo * swell(length, 0.4, 0.04) * 0.4


def argus_pulse():
    length = 0.35
    t = seconds(length)
    tone = np.sin(glide(length, 240, 110, 0.5)) * (0.6 + 0.4 * np.sin(2 * np.pi * 55 * t))
    air = bandpass(noise(length, 380), 400, 2500) * decay(length, 18) * 0.4
    return (tone + air) * decay(length, 9, 0.004)


def argus_shot(seed):
    def make():
        length = 0.12
        phase = glide(length, 1600 + 120 * seed, 620, 0.4)
        return (np.sin(phase) + 0.2 * np.sign(np.sin(phase))) * decay(length, 26, 0.002)

    return make


def argus_summon():
    track = np.zeros(int(0.85 * RATE))
    for i, pitch in enumerate([523.0, 659.0, 784.0, 1046.0]):
        place(track, bell(0.45, pitch, 7) * 0.5, i * 0.09)
    shimmer = bandpass(noise(0.8, 390), 4000, 9000) * swell(0.8, 0.3, 0.4) * 0.12
    return place(track, shimmer, 0.0)


def argus_teleport():
    length = 0.45
    t = seconds(length)
    air = bandpass(noise(length, 400), 800, 5000) * (t / length) ** 2
    tone = np.sin(glide(length, 300, 1500, 2.0)) * (t / length) ** 1.5 * 0.5
    return fade_out(air + tone, 0.03)


def argus_rage():
    length = 1.0
    t = seconds(length)
    vibrato = 6 * np.sin(2 * np.pi * 9 * t)
    shriek = saw(glide(length, 620, 760) + vibrato) + saw(glide(length, 655, 800) + vibrato)
    return bandpass(shriek, 400, 3500) * swell(length, 0.12, 0.4) * 0.5


def ogre_jump():
    length = 0.55
    voice = growl(0.4, 125, 92, 410, 0.6, 150, 1500) * swell(0.4, 0.04, 0.15)
    track = np.concatenate([voice, np.zeros(int((length - 0.4) * RATE))])
    place(track, whoosh(0.3, 411, 250, 1800, 1.0) * 0.5, 0.22)
    return track


def ogre_throw():
    length = 0.35
    return growl(length, 115, 150, 420, 0.5, 150, 1600) * swell(length, 0.03, 0.12)


def ogre_swing():
    length = 0.5
    t = seconds(length)
    low = bandpass(noise(length, 430), 180, 900) * (t / length) ** 1.5
    high = bandpass(noise(length, 431), 900, 3000) * (t / length) ** 3
    track = (low + high * 0.6) * swell(length, 0.05, 0.04)
    place(track, growl(0.2, 110, 95, 432, 0.5, 150, 1300) * swell(0.2, 0.02, 0.08) * 0.5, 0.0)
    return track


def ogre_roar():
    length = 0.8
    return growl(length, 112, 84, 440, 0.8, 120, 1800) * swell(length, 0.08, 0.35)


def ogre_rage():
    length = 1.3
    voice = growl(length, 100, 70, 450, 0.9, 100, 1600) * swell(length, 0.1, 0.55)
    return voice + rumble(length, 451, 150) * swell(length, 0.1, 0.6) * 0.5


def hound_growl():
    length = 0.38
    voice = growl(length, 88, 96, 460, 1.0, 90, 1100)
    snarl = bandpass(noise(length, 461), 1200, 3500) * jitter(length, 462, 50, 0.9) * 0.3
    return (voice + snarl) * swell(length, 0.05, 0.08)


def hound_inhale():
    length = 0.45
    t = seconds(length)
    air = bandpass(noise(length, 470), 600, 4000) * (t / length) ** 2
    crackle = highpass(noise(length, 471), 3000) * (np.random.default_rng(472).uniform(0, 1, len(t)) > 0.92) * 0.3
    return fade_out(air + crackle, 0.02)


def hound_breath():
    length = 1.8
    t = seconds(length)
    roar = lowpass(noise(length, 480), 950)
    hiss = bandpass(noise(length, 481), 2500, 8000) * 0.25
    flutter = 1.0 + 0.3 * np.sin(2 * np.pi * 21 * t)
    return (roar + hiss) * flutter * swell(length, 0.08, 0.4)


def hound_howl():
    length = 0.6
    t = seconds(length)
    shape = np.interp(t / length, [0, 0.35, 1], [290, 520, 410])
    phase = 2 * np.pi * np.cumsum(shape * (1 + 0.012 * np.sin(2 * np.pi * 6 * t))) / RATE
    voice = bandpass(saw(phase) + 0.6 * np.sin(phase), 250, 2400)
    under = growl(length, 95, 85, 490, 0.6, 90, 700) * 0.4
    return (voice + under) * swell(length, 0.08, 0.2)


def hound_rage():
    track = np.zeros(int(1.15 * RATE))
    place(track, hound_growl() * 1.1, 0.0)
    place(track, hound_howl() * 0.9, 0.3)
    return track


def creak(length, seed, start=28.0, end=75.0):
    """Wood under strain: a stick-slip train of tiny pulses run through a woody band."""
    t = seconds(length)
    rate = start + (end - start) * (t / length)
    phase = np.cumsum(rate) / RATE
    pulses = (np.diff(np.floor(phase), prepend=0.0) > 0).astype(float)
    pulses *= np.random.default_rng(seed).uniform(0.5, 1.0, len(t))
    return bandpass(lowpass(pulses, 3000), 350, 1900) * 12.0


def clack(seed, pitch=700.0):
    return click(seed, pitch, 0.06, 55) * 1.5 + lowpass(noise(0.06, seed + 1), 600) * decay(0.06, 50)


def mimic_creak():
    length = 0.38
    return creak(length, 500) * swell(length, 0.05, 0.08)


def mimic_chomp():
    track = np.zeros(int(0.42 * RATE))
    place(track, creak(0.22, 510, 40, 90) * swell(0.22, 0.03, 0.05), 0.0)
    place(track, clack(511, 650), 0.24)
    place(track, clack(513, 820) * 0.6, 0.29)
    return track


def mimic_gurgle():
    length = 0.42
    wet = lowpass(noise(length, 520), 700) * jitter(length, 521, 25, 0.9) * 0.6
    return (wet + bubbles(length, 522, 9, 150, 500)) * swell(length, 0.05, 0.08)


def mimic_jingle():
    track = np.zeros(int(0.55 * RATE))
    rng = np.random.default_rng(530)
    for i in range(7):
        pitch = rng.uniform(2200, 4600)
        place(track, bell(0.2, pitch, 22) * rng.uniform(0.4, 0.9), i * 0.045 + rng.uniform(0, 0.02))
    return track


def mimic_shut():
    track = np.zeros(int(0.4 * RATE))
    place(track, creak(0.12, 540, 60, 30) * 0.6, 0.0)
    slam = lowpass(noise(0.15, 541), 400) * decay(0.15, 30) * 3.0
    place(track, slam + np.concatenate([clack(542, 520), np.zeros(int(0.09 * RATE))]), 0.12)
    place(track, click(543, 2600, 0.03, 140) * 0.6, 0.28)
    return track


def mimic_rage():
    length = 1.0
    track = growl(length, 95, 75, 550, 0.8, 100, 1200) * swell(length, 0.08, 0.4) * 0.7
    rng = np.random.default_rng(551)
    at = 0.0
    while at < length - 0.08:
        place(track, clack(552 + int(at * 100), rng.uniform(500, 900)) * rng.uniform(0.4, 0.8), at)
        at += rng.uniform(0.05, 0.09)
    return track


SOUNDS = {
    "ghost_dash_1.ogg": (ghost_dash(0), 0.3),
    "ghost_dash_2.ogg": (ghost_dash(1), 0.3),
    "slime_smash_1.ogg": (slime_smash(0), 0.35),
    "slime_smash_2.ogg": (slime_smash(1), 0.35),
    "slime_burst.ogg": (slime_burst, 0.4),
    "toxic_burst.ogg": (toxic_burst, 0.32),
    "kin_heal.ogg": (kin_heal, 0.2),
    "leaper_crouch.ogg": (leaper_crouch, 0.3),
    "leaper_land_1.ogg": (leaper_land(0), 0.35),
    "leaper_land_2.ogg": (leaper_land(1), 0.35),
    "eye_charge.ogg": (eye_charge, 0.25),
    "eye_shoot_1.ogg": (eye_shoot(0), 0.28),
    "eye_shoot_2.ogg": (eye_shoot(1), 0.28),
    "spider_hatch.ogg": (spider_hatch, 0.38),
    "web_charge.ogg": (web_charge, 0.3),
    "web_spit.ogg": (web_spit, 0.38),
    "king_spiral_1.ogg": (king_spiral(0), 0.16),
    "king_spiral_2.ogg": (king_spiral(1), 0.16),
    "king_smash.ogg": (king_smash, 0.55),
    "golem_slam.ogg": (golem_slam, 0.5),
    "golem_throw.ogg": (golem_throw, 0.5),
    "golem_rain.ogg": (golem_rain, 0.5),
    "laser_charge.ogg": (laser_charge, 0.22),
    "laser_fire.ogg": (laser_fire, 0.32),
    "golem_rage.ogg": (golem_rage, 0.6),
    "argus_charge.ogg": (argus_charge, 0.32),
    "argus_pulse.ogg": (argus_pulse, 0.32),
    "argus_shot_1.ogg": (argus_shot(0), 0.22),
    "argus_shot_2.ogg": (argus_shot(1), 0.22),
    "argus_summon.ogg": (argus_summon, 0.35),
    "argus_teleport.ogg": (argus_teleport, 0.35),
    "argus_rage.ogg": (argus_rage, 0.45),
    "ogre_jump.ogg": (ogre_jump, 0.55),
    "ogre_throw.ogg": (ogre_throw, 0.5),
    "ogre_swing.ogg": (ogre_swing, 0.5),
    "ogre_roar.ogg": (ogre_roar, 0.6),
    "ogre_rage.ogg": (ogre_rage, 0.65),
    "hound_growl.ogg": (hound_growl, 0.5),
    "hound_inhale.ogg": (hound_inhale, 0.4),
    "hound_breath.ogg": (hound_breath, 0.42),
    "hound_howl.ogg": (hound_howl, 0.5),
    "hound_rage.ogg": (hound_rage, 0.6),
    "mimic_creak.ogg": (mimic_creak, 0.6),
    "mimic_chomp.ogg": (mimic_chomp, 0.5),
    "mimic_gurgle.ogg": (mimic_gurgle, 0.42),
    "mimic_jingle.ogg": (mimic_jingle, 0.3),
    "mimic_shut.ogg": (mimic_shut, 0.5),
    "mimic_rage.ogg": (mimic_rage, 0.55),
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, (make, peak) in SOUNDS.items():
        signal = normalize(fade_out(make(), 0.01), peak)
        sf.write(OUT / name, np.column_stack([signal, signal]), RATE, format="OGG", subtype="VORBIS")
        print(name, f"{len(signal) / RATE:.2f}s")


if __name__ == "__main__":
    main()
