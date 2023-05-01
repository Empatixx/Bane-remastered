package cz.empatix.audio.domain;

import org.lwjgl.openal.AL10;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public record AudioSource (
        int idSource
){
    private final static Logger LOGGER = LoggerFactory.getLogger(AudioSource.class);
    public AudioSource(){
        this(AL10.alGenSources());
    }
    public void delete(){
        AL10.alDeleteSources(idSource);
    }
    public void play(){
        AL10.alSourcePlay(idSource);
    }
    public void pause(){
        AL10.alSourcePause(idSource);
    }
    public void stop(){
        AL10.alSourceStop(idSource);
    }
    public void rewind(){
        AL10.alSourceRewind(idSource);
    }
    public boolean isPlaying(){
        return AL10.alGetSourcei(idSource, AL10.AL_SOURCE_STATE) == AL10.AL_PLAYING;
    }
    public boolean isPaused(){
        return AL10.alGetSourcei(idSource, AL10.AL_SOURCE_STATE) == AL10.AL_PAUSED;
    }
    public boolean isStopped(){
        return AL10.alGetSourcei(idSource, AL10.AL_SOURCE_STATE) == AL10.AL_STOPPED;
    }
    public void setPitch(float pitch){
        AL10.alSourcef(idSource, AL10.AL_PITCH, pitch);
    }
    public void setGain(float gain){
        AL10.alSourcef(idSource, AL10.AL_GAIN, gain);
    }
    public void setLooping(boolean looping){
        AL10.alSourcei(idSource, AL10.AL_LOOPING, looping ? AL10.AL_TRUE : AL10.AL_FALSE);
    }
    public void setPosition(float x, float y, float z){
        AL10.alSource3f(idSource, AL10.AL_POSITION, x, y, z);
    }
    public void setVelocity(float x, float y, float z){
        AL10.alSource3f(idSource, AL10.AL_VELOCITY, x, y, z);
    }
    public void setDirection(float x, float y, float z){
        AL10.alSource3f(idSource, AL10.AL_DIRECTION, x, y, z);
    }
    public void setRelative(boolean relative){
        AL10.alSourcei(idSource, AL10.AL_SOURCE_RELATIVE, relative ? AL10.AL_TRUE : AL10.AL_FALSE);
    }
    public void setMinDistance(float distance){
        AL10.alSourcef(idSource, AL10.AL_REFERENCE_DISTANCE, distance);
    }
    public void setMaxDistance(float distance){
        AL10.alSourcef(idSource, AL10.AL_MAX_DISTANCE, distance);
    }
    public void setRolloffFactor(float factor){
        AL10.alSourcef(idSource, AL10.AL_ROLLOFF_FACTOR, factor);
    }
    public float getGain(){
        return AL10.alGetSourcef(idSource, AL10.AL_GAIN);
    }
    public void play(int buffer){
        stop();
        AL10.alSourcei(idSource, AL10.AL_BUFFER, buffer);
        play();
    }
}
