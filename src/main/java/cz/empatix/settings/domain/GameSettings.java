package cz.empatix.settings.domain;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.google.common.base.Preconditions;

public class GameSettings {
    private int resolutionWidth;
    private int resolutionHeight;
    private boolean fullscreen;
    private boolean vsync;
    private float overallVolume;
    private float musicVolume;
    private float effectsVolume;
    public GameSettings(final GameSettings settings){
        this(settings.getResolutionWidth(), settings.getResolutionHeight(), settings.isFullscreen(), settings.isVsync(), settings.getOverallVolume(), settings.getMusicVolume(), settings.getEffectsVolume());
    }

    @JsonCreator
    public GameSettings(@JsonProperty("resolutionWidth") final int resolutionWidth,
                        @JsonProperty("resolutionHeight") final int resolutionHeight,
                        @JsonProperty("fullscreen") final boolean fullscreen,
                        @JsonProperty("vsync") final boolean vsync,
                        @JsonProperty("overallVolume") final float overallVolume,
                        @JsonProperty("musicVolume") final float musicVolume,
                        @JsonProperty("effectsVolume") final float effectsVolume) {
        Preconditions.checkArgument(resolutionWidth > 0, "resolutionWidth must be positive");
        Preconditions.checkArgument(resolutionHeight > 0, "resolutionHeight must be positive");
        Preconditions.checkArgument(overallVolume >= 0 && overallVolume <= 1, "overallVolume must be between 0 and 1");
        Preconditions.checkArgument(musicVolume >= 0 && musicVolume <= 1, "musicVolume must be between 0 and 1");
        Preconditions.checkArgument(effectsVolume >= 0 && effectsVolume <= 1, "effectsVolume must be between 0 and 1");

        this.resolutionWidth = resolutionWidth;
        this.resolutionHeight = resolutionHeight;
        this.fullscreen = fullscreen;
        this.vsync = vsync;
        this.overallVolume = overallVolume;
        this.musicVolume = musicVolume;
        this.effectsVolume = effectsVolume;
    }

    public int getResolutionHeight() {
        return resolutionHeight;
    }

    public int getResolutionWidth() {
        return resolutionWidth;
    }

    public boolean isFullscreen() {
        return fullscreen;
    }

    public boolean isVsync() {
        return vsync;
    }

    public float getEffectsVolume() {
        return effectsVolume;
    }

    public float getOverallVolume() {
        return overallVolume;
    }

    public float getMusicVolume() {
        return musicVolume;
    }

    public void setEffectsVolume(float effectsVolume) {
        this.effectsVolume = effectsVolume;
    }

    public void setFullscreen(boolean fullscreen) {
        this.fullscreen = fullscreen;
    }

    public void setMusicVolume(float musicVolume) {
        this.musicVolume = musicVolume;
    }

    public void setOverallVolume(float overallVolume) {
        this.overallVolume = overallVolume;
    }

    public void setResolutionHeight(int resolutionHeight) {
        this.resolutionHeight = resolutionHeight;
    }

    public void setResolutionWidth(int resolutionWidth) {
        this.resolutionWidth = resolutionWidth;
    }

    public void setVsync(boolean vsync) {
        this.vsync = vsync;
    }

    public void deepCopy(final GameSettings appliedSettings) {
        this.resolutionWidth = appliedSettings.resolutionWidth;
        this.resolutionHeight = appliedSettings.resolutionHeight;
        this.fullscreen = appliedSettings.fullscreen;
        this.vsync = appliedSettings.vsync;
        this.overallVolume = appliedSettings.overallVolume;
        this.musicVolume = appliedSettings.musicVolume;
        this.effectsVolume = appliedSettings.effectsVolume;
    }
}
