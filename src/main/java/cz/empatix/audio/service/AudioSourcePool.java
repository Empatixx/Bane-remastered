package cz.empatix.audio.service;

import cz.empatix.audio.domain.AudioSource;
import cz.empatix.common.initializators.ALInit;

import java.util.ArrayList;
import java.util.List;

public class AudioSourcePool {
    private final static int MAX_AUDIO_SOURCES = 32;
    private final List<AudioSource> audioSources = new ArrayList<>(MAX_AUDIO_SOURCES);
    public AudioSourcePool(){
        if(!ALInit.canUseAL){
            return;
        }
        for(int i = 0; i < MAX_AUDIO_SOURCES; i++){
            audioSources.add(new AudioSource());
        }
    }
    public AudioSource getAudioSource(){
        return audioSources
                .stream()
                .filter(audioSource -> !audioSource.isPlaying())
                .findAny()
                .orElse(null);
    }
    public AudioSource reserveAudioSource(){
        final AudioSource audioSource = getAudioSource();
        if(audioSource == null){
            return null;
        }
        audioSources.remove(audioSource);
        return audioSource;
    }
}
