package cz.empatix.audio.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.AudioSource;
import cz.empatix.audio.domain.Soundtrack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class SoundtrackService {
    private final static Logger LOGGER = LoggerFactory.getLogger(SoundtrackService.class);
    private final static float TWEEN = 0.5f;
    private final AudioSource[] audioSources;
    private AudioSource currentAudioSource;
    private AudioSource nextAudioSource;
    public SoundtrackService(final AudioSourcePool audioSourcePool) {
        this.audioSources = new AudioSource[2];
        audioSources[0] = audioSourcePool.reserveAudioSource();
        audioSources[1] = audioSourcePool.reserveAudioSource();
    }
    public void update(final float dt){
        if(nextAudioSource != null){
            final float currentGain = currentAudioSource.getGain();
            final float nextGain = nextAudioSource.getGain();

            currentAudioSource.setGain(currentGain - TWEEN * dt);
            nextAudioSource.setGain(nextGain + TWEEN * dt);

            if(currentGain <= 0f){
                currentAudioSource.stop();
                currentAudioSource = nextAudioSource;
                nextAudioSource = null;
            }
        }
    }
    public void play(final Soundtrack soundtrack) {
        LOGGER.info("Playing soundtrack {}.", soundtrack.name());
        if(currentAudioSource == null){
            currentAudioSource = audioSources[0];
            final int buffer = AssetsService.getAudio(soundtrack.getPath());
            currentAudioSource.play(buffer);
        } else {
            nextAudioSource = currentAudioSource == audioSources[0] ? audioSources[1] : audioSources[0];
            final int buffer = AssetsService.getAudio(soundtrack.getPath());
            nextAudioSource.play(buffer);
        }
    }
}
