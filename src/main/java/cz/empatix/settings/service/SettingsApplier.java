package cz.empatix.settings.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.settings.domain.GameSettings;

public class SettingsApplier {
    private GameSettings previousSettings = null;
    public void applySettings(final GameSettings settings) {
        applyChanges(previousSettings, settings);
        previousSettings = new GameSettings(settings);
    }
    private void applyChanges(final GameSettings previousSettings,
                              final GameSettings currentSettings){
        if(previousSettings == null){
            WindowHandler.setFullscreen(currentSettings.isFullscreen());
            WindowHandler.setResolution(currentSettings.getResolutionWidth(), currentSettings.getResolutionHeight());
            WindowHandler.setVsync(currentSettings.isVsync());
        } else {
            if(previousSettings.isFullscreen() != currentSettings.isFullscreen()){
                WindowHandler.setFullscreen(currentSettings.isFullscreen());
            }
            if(previousSettings.getResolutionWidth() != currentSettings.getResolutionWidth() ||
                    previousSettings.getResolutionHeight() != currentSettings.getResolutionHeight()){
                WindowHandler.setResolution(currentSettings.getResolutionWidth(), currentSettings.getResolutionHeight());
            }
            if(previousSettings.isVsync() != currentSettings.isVsync()){
                WindowHandler.setVsync(currentSettings.isVsync());
            }
        }
    }
}
