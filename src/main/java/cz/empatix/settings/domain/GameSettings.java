package cz.empatix.settings.domain;

import com.google.common.base.Preconditions;

public class GameSettings {
    public int resolutionWidth;
    public int resolutionHeight;
    public boolean fullscreen;
    public boolean vsync;
    public float overallVolume;
    public float musicVolume;
    public float effectsVolume;
    public GameSettings(final int resolutionWidth,
                        final int resolutionHeight,
                        final boolean fullscreen,
                        final boolean vsync,
                        final float overallVolume,
                        final float musicVolume,
                        final float effectsVolume){
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
}
