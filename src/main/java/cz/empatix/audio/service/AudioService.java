package cz.empatix.audio.service;

import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.common.initializators.ALInit;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;

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

        final GameSettings settings = SettingsService.getInstance().getSettings();
        soundtrackService.setVolume(settings.getOverallVolume(), settings.getMusicVolume());
        soundEffectService.setVolume(settings.getOverallVolume(), settings.getEffectsVolume());
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
