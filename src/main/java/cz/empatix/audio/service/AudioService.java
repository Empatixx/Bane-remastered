package cz.empatix.audio.service;

import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.domain.Soundtrack;

public class AudioService {
    private final SoundtrackService soundtrackService;
    private final SoundEffectService soundEffectService;
    public AudioService() {
        final AudioSourcePool audioSourcePool = new AudioSourcePool();
        this.soundtrackService = new SoundtrackService(audioSourcePool);
        this.soundEffectService = new SoundEffectService(audioSourcePool);
    }
    public void update(final float dt){
        soundtrackService.update(dt);
    }
    public void play(final Soundtrack soundtrack) {
        soundtrackService.play(soundtrack);
    }
    public void play(final SoundEffect soundEffect) {
        soundEffectService.play(soundEffect);
    }
}
