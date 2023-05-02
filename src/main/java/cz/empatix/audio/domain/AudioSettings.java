package cz.empatix.audio.domain;

public class AudioSettings {
    private static final float DEFAULT_OVERALL_VOLUME = 1f;
    private static final float DEFAULT_SOUNDTRACK_VOLUME = 1f;
    private static final float DEFAULT_SOUND_EFFECT_VOLUME = 1f;
    public float overallVolume = DEFAULT_OVERALL_VOLUME;
    public float soundtrackVolume = DEFAULT_SOUNDTRACK_VOLUME;
    public float soundEffectVolume = DEFAULT_SOUND_EFFECT_VOLUME;

    private static AudioSettings instance;
    public static AudioSettings getInstance(){
        if(instance == null){
            instance = new AudioSettings();
        }
        return instance;
    }

    public void setOverallVolume(final float overallVolume) {
        this.overallVolume = overallVolume;
    }
    public void setSoundtrackVolume(final float soundtrackVolume) {
        this.soundtrackVolume = soundtrackVolume;
    }
    public void setSoundEffectVolume(final float soundEffectVolume) {
        this.soundEffectVolume = soundEffectVolume;
    }
    public float getOverallVolume() {
        return overallVolume;
    }
    public float getSoundtrackVolume() {
        return soundtrackVolume;
    }
    public float getSoundEffectVolume() {
        return soundEffectVolume;
    }

}
