package cz.empatix.settings.service;

import com.fasterxml.jackson.databind.ObjectMapper;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.common.initializators.GLFWInit;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.ui.SlidingList;
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
    private final GameSettings settings = loadSettings();
    private final SettingsApplier settingsApplier = new SettingsApplier();
    private static final SettingsService instance = new SettingsService();

    public static SettingsService getInstance() {
        return instance;
    }

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
                false, true, 0.5f, 0.5f, 0.5f);
    }

    public GameSettings getSettings() {
        return settings;
    }

    public void saveSettings() {
        LOGGER.info("Saving settings to {}", SETTINGS_FILE);
        try {
            MAPPER.writeValue(SETTINGS_FILE.toFile(), settings);
        } catch (Exception e) {
            LOGGER.error("Failed to save settings", e);
        }
    }
    public void applySettings() {
        settingsApplier.applySettings(settings);
    }

    public List<Vector2i> getResolutions() {
        return WindowHandler.getMonitorResolutions();
    }
}
