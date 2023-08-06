package cz.empatix.settings.service;

import com.fasterxml.jackson.databind.ObjectMapper;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.settings.domain.GameSettings;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.file.Files;
import java.nio.file.Path;

public class SettingsService {
    private final static Logger LOGGER = LoggerFactory.getLogger(SettingsService.class);
    private final static ObjectMapper MAPPER = new ObjectMapper();
    private final static Path SETTINGS_FILE = Path.of("settings.json");
    private final GameSettings settings = loadSettings();
    private final SettingsService instance = new SettingsService();
    public SettingsService getInstance() {
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
        return new GameSettings(WindowHandler.WIDTH, WindowHandler.HEIGHT, false, true, 0.5f, 0.5f, 0.5f);
    }
    private void saveSettings() {
        LOGGER.info("Saving settings to {}", SETTINGS_FILE);
        try {
            MAPPER.writeValue(SETTINGS_FILE.toFile(), settings);
        } catch (Exception e) {
            LOGGER.error("Failed to save settings", e);
        }
    }
}
