package cz.empatix.audio.service;

public class AudioService {
    private final AudioSourcePool audioSourcePool;
    private final SoundtrackService soundtrackService;
    private final SoundEffectService soundEffectService;
    public AudioService() {
        this.audioSourcePool = new AudioSourcePool();
        this.soundtrackService = new SoundtrackService();
        this.soundEffectService = new SoundEffectService();
    }
}
