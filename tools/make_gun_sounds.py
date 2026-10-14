import sys
from pathlib import Path

import numpy as np
import soundfile as sf

GUNS = Path(__file__).resolve().parent.parent / "assets/audio/guns"
OUT = Path(sys.argv[1]) if len(sys.argv) > 1 else GUNS
RATE = 48000


def seconds(length):
    return np.arange(int(length * RATE)) / RATE


def noise(length, seed):
    return np.random.default_rng(seed).uniform(-1.0, 1.0, int(length * RATE))


def lowpass(signal, cutoff):
    alpha = 1.0 - np.exp(-2.0 * np.pi * cutoff / RATE)
    out = np.empty_like(signal)
    acc = 0.0
    for i, x in enumerate(signal):
        acc += alpha * (x - acc)
        out[i] = acc
    return out


def highpass(signal, cutoff):
    return signal - lowpass(signal, cutoff)


def bandpass(signal, low, high):
    return lowpass(highpass(signal, low), high)


def decay(length, rate, attack=0.002):
    t = seconds(length)
    return np.minimum(t / attack, 1.0) * np.exp(-t * rate)


def place(track, sound, at):
    start = int(at * RATE)
    end = min(len(track), start + len(sound))
    track[start:end] += sound[: end - start]
    return track


def load(name):
    data, rate = sf.read(GUNS / name, always_2d=True)
    mono = data.mean(axis=1)
    if rate != RATE:
        mono = stretch(mono, RATE / rate)
    return mono


def stretch(signal, factor):
    """Resamples by the factor: above one the sound gets longer and lower."""
    positions = np.arange(int(len(signal) * factor)) / factor
    return np.interp(positions, np.arange(len(signal)), signal)


def fade_out(signal, length):
    n = min(len(signal), int(length * RATE))
    signal = signal.copy()
    signal[-n:] *= np.linspace(1.0, 0.0, n)
    return signal


def echo(signal, delays, gains):
    out = np.concatenate([signal, np.zeros(int(max(delays) * RATE))])
    for delay, gain in zip(delays, gains):
        place(out, signal * gain, delay)
    return out


def click(seed, pitch=3200.0, length=0.04, ring=90.0):
    t = seconds(length)
    body = bandpass(noise(length, seed), pitch * 0.5, pitch * 1.5) * decay(length, ring * 2)
    tone = np.sin(2 * np.pi * pitch * t) * decay(length, ring)
    return body * 0.8 + tone * 0.35


def scrape(seed, length, low=900.0, high=4000.0):
    t = seconds(length)
    envelope = np.sin(np.pi * t / length) ** 0.6
    grit = 0.6 + 0.4 * np.sign(np.sin(2 * np.pi * 37 * t))
    return bandpass(noise(length, seed), low, high) * envelope * grit * 0.5


def normalize(signal, peak):
    return signal / max(np.abs(signal).max(), 1e-9) * peak


def sniper_shoot():
    shot = stretch(load("revolver_shoot.ogg")[: int(1.4 * RATE)], 1.18)
    crack = bandpass(noise(0.05, 1), 1500, 9000) * decay(0.05, 70)
    body = place(shot.copy(), crack * 0.8, 0.0)
    return fade_out(echo(body, [0.11, 0.27], [0.32, 0.16])[: int(1.7 * RATE)], 0.6)


def sniper_reload():
    track = np.zeros(int(0.95 * RATE))
    place(track, click(2, 2600), 0.05)
    place(track, scrape(3, 0.16), 0.14)
    place(track, click(4, 2200), 0.30)
    place(track, scrape(5, 0.13, 1200, 5000), 0.55)
    place(track, click(6, 1800, 0.06, 60) * 1.3, 0.70)
    return track


def minigun_shoot(seed):
    def variant():
        shot = stretch(load("uzi_shoot.ogg")[: int(0.16 * RATE)], 1.12 + 0.05 * seed)
        spin = np.sin(2 * np.pi * (140 + 20 * seed) * seconds(len(shot) / RATE)) * 0.12
        return fade_out(shot + spin * decay(len(shot) / RATE, 8, 0.01), 0.06)

    return variant


def minigun_reload():
    clatter = stretch(load("M4_reload.ogg"), 1.15)
    spin_length = 0.9
    t = seconds(spin_length)
    frequency = 60 + 260 * (t / spin_length) ** 1.5
    phase = 2 * np.pi * np.cumsum(frequency) / RATE
    whir = (np.sin(phase) + 0.4 * np.sin(2 * phase) + 0.2 * np.sign(np.sin(3 * phase)))
    whir *= np.minimum(t / 0.2, 1.0) * np.linspace(0.6, 1.0, len(t)) * 0.25
    track = np.zeros(len(clatter) + int(0.7 * RATE))
    place(track, clatter, 0.0)
    place(track, whir, len(clatter) / RATE - 0.2)
    return fade_out(track, 0.15)


def crossbow_shoot():
    length = 0.55
    t = seconds(length)
    pitch = 150 * np.exp(-t * 3) + 85
    phase = 2 * np.pi * np.cumsum(pitch) / RATE
    twang = (np.sin(phase) + 0.5 * np.sin(2 * phase) + 0.25 * np.sin(3 * phase)) * decay(length, 9)
    thwack = lowpass(noise(0.08, 7), 1800) * decay(0.08, 45)
    swish = bandpass(noise(0.25, 8), 2000, 7000) * np.sin(np.pi * seconds(0.25) / 0.25) * 0.25
    track = twang * 0.45
    place(track, thwack * 1.2, 0.0)
    place(track, swish, 0.02)
    return track


def crossbow_reload():
    track = np.zeros(int(0.85 * RATE))
    at = 0.04
    for i in range(7):
        place(track, click(20 + i, 1400 + 120 * i, 0.03, 120) * 0.7, at)
        at += 0.085 - i * 0.005
    place(track, scrape(30, 0.12, 400, 2000) * 1.2, at)
    place(track, click(31, 900, 0.08, 40) * 1.6, at + 0.12)
    return track


def flamethrower_shoot(seed):
    def variant():
        length = 0.22
        t = seconds(length)
        envelope = np.minimum(t / 0.03, 1.0) * np.exp(-np.maximum(t - 0.05, 0) * 14)
        roar = lowpass(noise(length, 40 + seed), 900 + 150 * seed)
        hiss = bandpass(noise(length, 50 + seed), 2500, 8000) * 0.25
        flutter = 1.0 + 0.3 * np.sin(2 * np.pi * (23 + 4 * seed) * t)
        return (roar + hiss) * envelope * flutter

    return variant


def flamethrower_reload():
    track = np.zeros(int(1.2 * RATE))
    place(track, click(60, 1500, 0.05, 70) * 1.2, 0.05)
    hiss_length = 0.55
    hiss = highpass(noise(hiss_length, 61), 3000) * np.sin(np.pi * seconds(hiss_length) / hiss_length) * 0.35
    place(track, hiss, 0.2)
    place(track, click(62, 1100, 0.07, 50) * 1.4, 0.8)
    length = 0.3
    t = seconds(length)
    whump = lowpass(noise(length, 63), 400) * np.minimum(t / 0.02, 1.0) * np.exp(-t * 10)
    place(track, whump * 2.0, 0.88)
    return track


def sweep(length, start, end, curve=2.0):
    t = seconds(length)
    frequency = end + (start - end) * (1 - t / length) ** curve
    return 2 * np.pi * np.cumsum(frequency) / RATE


def square(phase):
    return np.sign(np.sin(phase))


def blaster_shoot(seed):
    def variant():
        length = 0.22
        phase = sweep(length, 1900 + 150 * seed, 260, 3.0)
        tone = 0.6 * square(phase) + 0.5 * np.sin(phase * 1.5)
        return lowpass(tone, 6000) * decay(length, 16, 0.001)

    return variant


def charge(length, start, end, seed):
    phase = sweep(length, start, end, 0.6)
    t = seconds(length)
    hum = np.sin(phase) + 0.3 * np.sin(2 * phase)
    fizz = bandpass(noise(length, seed), 3000, 9000) * 0.15
    return (hum * 0.5 + fizz) * np.minimum(t / 0.05, 1.0)


def energy_reload(start, end, seed, length=0.7):
    track = np.zeros(int((length + 0.3) * RATE))
    place(track, click(seed, 1700, 0.05, 70) * 1.2, 0.03)
    place(track, click(seed + 1, 1200, 0.06, 60), 0.15)
    place(track, fade_out(charge(length, start, end, seed + 2), 0.08) * 0.8, 0.25)
    t = seconds(0.12)
    place(track, np.sin(2 * np.pi * end * 1.5 * t) * decay(0.12, 30) * 0.5, 0.25 + length - 0.05)
    return track


def plasma_shoot(seed):
    def variant():
        length = 0.32
        t = seconds(length)
        carrier = sweep(length, 520 + 40 * seed, 120, 2.5)
        modulator = np.sin(2 * np.pi * (90 + 15 * seed) * t) * 3.0
        tone = np.sin(carrier + modulator)
        body = lowpass(noise(length, 70 + seed), 600) * 0.6
        return (tone * 0.7 + body) * decay(length, 10, 0.004)

    return variant


def zap(seed, length=0.25):
    rng = np.random.default_rng(seed)
    t = seconds(length)
    crackle = noise(length, seed) * (rng.uniform(0, 1, len(t)) > 0.75)
    buzz = square(2 * np.pi * (110 + 10 * seed) * t) * 0.35
    gate = (np.sin(2 * np.pi * 37 * t) > -0.3).astype(float)
    return bandpass(crackle + buzz, 400, 7000) * gate * decay(length, 9, 0.002)


def tesla_shoot():
    length = 0.5
    phase = sweep(length, 900, 180, 1.5)
    whoom = np.sin(phase) * decay(length, 7, 0.005) * 0.6
    track = whoom + np.concatenate([zap(80, 0.4), np.zeros(int(0.1 * RATE))]) * 1.3
    return track


def railgun_shoot():
    whine_length = 0.18
    whine = np.sin(sweep(whine_length, 600, 3800, 0.5)) * np.minimum(seconds(whine_length) / 0.1, 1.0) * 0.35
    length = 1.3
    t = seconds(length)
    boom = lowpass(noise(length, 90), 500) * decay(length, 5, 0.002) * 1.6
    crack = bandpass(noise(0.08, 91), 2000, 10000) * decay(0.08, 50)
    ring = np.sin(sweep(length, 2400, 900, 1.0)) * decay(length, 4) * 0.25
    track = np.zeros(int((whine_length + length) * RATE))
    place(track, whine, 0.0)
    place(track, boom + ring, whine_length)
    place(track, crack, whine_length)
    return fade_out(echo(track, [0.13], [0.25]), 0.4)


SOUNDS = {
    "sniper_shoot.ogg": (sniper_shoot, 0.98),
    "sniper_reload.ogg": (sniper_reload, 0.6),
    "minigun_shoot_1.ogg": (minigun_shoot(0), 0.5),
    "minigun_shoot_2.ogg": (minigun_shoot(1), 0.5),
    "minigun_shoot_3.ogg": (minigun_shoot(2), 0.5),
    "minigun_reload.ogg": (minigun_reload, 0.6),
    "crossbow_shoot.ogg": (crossbow_shoot, 0.7),
    "crossbow_reload.ogg": (crossbow_reload, 0.5),
    "flamethrower_shoot_1.ogg": (flamethrower_shoot(0), 0.35),
    "flamethrower_shoot_2.ogg": (flamethrower_shoot(1), 0.35),
    "flamethrower_shoot_3.ogg": (flamethrower_shoot(2), 0.35),
    "flamethrower_reload.ogg": (flamethrower_reload, 0.35),
    "blaster_shoot_1.ogg": (blaster_shoot(0), 0.35),
    "blaster_shoot_2.ogg": (blaster_shoot(1), 0.35),
    "blaster_shoot_3.ogg": (blaster_shoot(2), 0.35),
    "blaster_reload.ogg": (lambda: energy_reload(300, 1400, 100), 0.13),
    "plasma_shoot_1.ogg": (plasma_shoot(0), 0.4),
    "plasma_shoot_2.ogg": (plasma_shoot(1), 0.4),
    "plasma_reload.ogg": (lambda: energy_reload(150, 700, 110, 0.9), 0.12),
    "tesla_shoot.ogg": (tesla_shoot, 0.7),
    "tesla_reload.ogg": (lambda: place(energy_reload(80, 400, 120, 0.8), zap(121, 0.2) * 0.8, 0.95), 0.18),
    "tesla_zap_1.ogg": (lambda: zap(130), 0.45),
    "tesla_zap_2.ogg": (lambda: zap(131), 0.45),
    "railgun_shoot.ogg": (railgun_shoot, 0.98),
    "railgun_reload.ogg": (lambda: energy_reload(120, 2200, 140, 1.3), 0.11),
}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for name, (make, peak) in SOUNDS.items():
        signal = normalize(make(), peak)
        sf.write(OUT / name, np.column_stack([signal, signal]), RATE, format="OGG", subtype="VORBIS")
        print(name, f"{len(signal) / RATE:.2f}s")


if __name__ == "__main__":
    main()
