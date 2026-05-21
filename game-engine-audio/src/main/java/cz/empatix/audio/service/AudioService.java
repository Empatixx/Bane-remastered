package cz.empatix.audio.service;

import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.init.ALInit;

public class AudioService {
    private static AudioService instance;
    public static AudioService getInstance(){
        if(instance == null){
            instance = new AudioService();
        }
        return instance;
    }
    private final SoundtrackService soundtrackService;
    private final SoundEffectService soundEffectService;
    private AudioService() {
        final AudioSourcePool audioSourcePool = new AudioSourcePool();
        this.soundtrackService = new SoundtrackService(audioSourcePool);
        this.soundEffectService = new SoundEffectService(audioSourcePool);
        // Initial volume is applied by the application via setVolume() once settings are
        // available, keeping the audio module independent of the game settings layer.
    }
    public void update(final float dt){
        if(!ALInit.canUseAL){
            return;
        }
        soundtrackService.update(dt);
    }
    public void play(final Soundtrack soundtrack) {
        if(!ALInit.canUseAL){
            return;
        }
        soundtrackService.play(soundtrack);
    }
    public void play(final SoundEffect soundEffect) {
        if(!ALInit.canUseAL){
            return;
        }
        soundEffectService.play(soundEffect);
    }

    public void setVolume(final float overallVolume, final float musicVolume, final float effectsVolume) {
        if(!ALInit.canUseAL){
            return;
        }
        soundtrackService.setVolume(overallVolume, musicVolume);
        soundEffectService.setVolume(overallVolume, effectsVolume);
    }
}
