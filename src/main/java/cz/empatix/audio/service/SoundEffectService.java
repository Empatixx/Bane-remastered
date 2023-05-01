package cz.empatix.audio.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.AudioSource;
import cz.empatix.audio.domain.SoundEffect;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class SoundEffectService {
    private final static Logger LOGGER = LoggerFactory.getLogger(SoundEffectService.class);
    private final AudioSourcePool audioSourcePool;
    public SoundEffectService(final AudioSourcePool audioSourcePool) {
        this.audioSourcePool = audioSourcePool;
    }
    public void play(final SoundEffect soundEffect) {
        final AudioSource audioSource = audioSourcePool.getAudioSource();
        if(audioSource == null){
            LOGGER.warn("No audio source available for sound effect {}.", soundEffect.name());
            return;
        }
        final int buffer = AssetsService.getAudio(soundEffect.getPath());
        audioSource.play(buffer);
    }
}
