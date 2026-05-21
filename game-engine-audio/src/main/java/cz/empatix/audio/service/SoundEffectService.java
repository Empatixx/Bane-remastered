package cz.empatix.audio.service;

import cz.empatix.audio.assets.AudioAssets;
import cz.empatix.audio.domain.AudioSource;
import cz.empatix.audio.domain.SoundEffectClip;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class SoundEffectService {
    private final static Logger LOGGER = LoggerFactory.getLogger(SoundEffectService.class);
    private final AudioSourcePool audioSourcePool;
    private float currentVolume = 1f;
    public SoundEffectService(final AudioSourcePool audioSourcePool) {
        this.audioSourcePool = audioSourcePool;
    }
    public void play(final SoundEffectClip soundEffect) {
        LOGGER.info("Playing sound effect {}.", soundEffect.name());
        final AudioSource audioSource = audioSourcePool.getAudioSource();
        if(audioSource == null){
            LOGGER.warn("No audio source available for sound effect {}.", soundEffect.name());
            return;
        }
        final int buffer = AudioAssets.getAudio(soundEffect.getPath());
        audioSource.play(buffer);
    }

    public void setVolume(final float overallVolume, final float effectsVolume) {
        audioSourcePool.setVolume(overallVolume * effectsVolume);
    }
}
