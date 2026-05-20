package cz.empatix.settings.service;

import com.fasterxml.jackson.databind.ObjectMapper;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.settings.domain.GameSettings;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class SettingsService {
    private final static Logger LOGGER = LoggerFactory.getLogger(SettingsService.class);
    private final static ObjectMapper MAPPER = new ObjectMapper();
    private final static Path SETTINGS_FILE = Path.of("settings.json");
    private static final SettingsService INSTANCE = new SettingsService();

    public static SettingsService getInstance() {
        return INSTANCE;
    }

    private final GameSettings appliedSettings = loadSettings();
    private final GameSettings unAppliedSettings = new GameSettings(appliedSettings);
    private final SettingsApplier settingsApplier = new SettingsApplier();


    private GameSettings loadSettings() {
        if (!Files.exists(SETTINGS_FILE)) {
            return createDefaultSettings();
        }
        LOGGER.info("Loading settings from {}", SETTINGS_FILE);
        try {
            return MAPPER.readValue(SETTINGS_FILE.toFile(), GameSettings.class);
        } catch (Exception e) {
            LOGGER.error("Failed to load settings", e);
            return createDefaultSettings();
        }
    }

    private GameSettings createDefaultSettings() {
        LOGGER.info("Creating default settings");
        return new GameSettings(WindowHandler.TARGET_WIDTH * 2, WindowHandler.TARGET_HEIGHT * 2,
                false, true, 1f, 0.5f, 0.5f);
    }

    public GameSettings getSettings() {
        return appliedSettings;
    }

    public void saveSettings() {
        LOGGER.info("Saving settings to {}", SETTINGS_FILE);
        try {
            MAPPER.writeValue(SETTINGS_FILE.toFile(), appliedSettings);
        } catch (Exception e) {
            LOGGER.error("Failed to save settings", e);
        }
    }

    public void apply() {
        settingsApplier.applySettings(unAppliedSettings);
        appliedSettings.deepCopy(unAppliedSettings);
    }

    public void revert() {
        unAppliedSettings.deepCopy(appliedSettings);
    }

    public List<Vector2i> getResolutions() {
        return WindowHandler.getMonitorResolutions();
    }

    public void setResolution(final Vector2i resolution) {
        unAppliedSettings.setResolutionWidth(resolution.x());
        unAppliedSettings.setResolutionHeight(resolution.y());
    }

    public void setVsync(final boolean vsync) {
        unAppliedSettings.setVsync(vsync);
    }

    public void setFullscreen(final boolean fullscreen) {
        unAppliedSettings.setFullscreen(fullscreen);
    }

    public void setOverallVolume(final float overallPercentage) {
        LOGGER.info("Setting overall volume to {}%", (int)(overallPercentage * 100));
        unAppliedSettings.setOverallVolume(overallPercentage);
    }

    public void setMusicVolume(final float musicPercentage) {
        unAppliedSettings.setMusicVolume(musicPercentage);
    }

    public void setEffectsVolume(final float effectsPercentage) {
        unAppliedSettings.setEffectsVolume(effectsPercentage);
    }
}
