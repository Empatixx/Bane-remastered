package cz.empatix.audio.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.AudioSource;
import cz.empatix.audio.domain.Soundtrack;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;


public class SoundtrackService {
    private static final int GAIN = 1;
    private final static Logger LOGGER = LoggerFactory.getLogger(SoundtrackService.class);
    private final static float TWEEN = 0.5f;
    private final AudioSource[] audioSources;
    private AudioSource currentAudioSource;
    private AudioSource nextAudioSource;
    private float maxGain = GAIN;
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

            if (nextAudioSource.getGain() >= maxGain){
                nextAudioSource.setGain(maxGain);
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
            currentAudioSource.setLooping(true);
            currentAudioSource.setGain(maxGain);
        } else {
            nextAudioSource = currentAudioSource == audioSources[0] ? audioSources[1] : audioSources[0];
            final int buffer = AssetsService.getAudio(soundtrack.getPath());
            nextAudioSource.play(buffer);
            nextAudioSource.setLooping(true);
            nextAudioSource.setGain(0);
        }
    }

    public void setVolume(final float overallVolume, final float musicVolume) {
        maxGain = overallVolume * musicVolume * GAIN;
        if (currentAudioSource != null){
            currentAudioSource.setGain(maxGain);
        }
    }
}
