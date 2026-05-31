package cz.empatix.upgrades.service;

import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.EnumMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Round-trip coverage of {@link FileUpgradeService#save}/{@link FileUpgradeService#load}, which
 * AES-encrypt a {@code PlayerUpgradeName -> Boolean} map to the {@code upgrades.bin} file (gitignored).
 * The file lives in the module working directory; we snapshot and restore it so the test is
 * self-contained and never clobbers a developer's real save.
 */
class FileUpgradeServiceTest {

    private static final Path PATH = Path.of("upgrades.bin");
    private static byte[] backup;

    @BeforeAll
    static void headlessAudio() throws Exception {
        cz.empatix.audio.init.ALInit.canUseAL = false;
        backup = Files.exists(PATH) ? Files.readAllBytes(PATH) : null;
    }

    @AfterAll
    static void restore() throws Exception {
        if (backup != null) {
            Files.write(PATH, backup);
        } else {
            Files.deleteIfExists(PATH);
        }
    }

    @Test
    void saveThenLoadRoundTripsTheBoughtMap() {
        final FileUpgradeService service = new FileUpgradeService();

        final Map<PlayerUpgradeName, Boolean> toSave = new EnumMap<>(PlayerUpgradeName.class);
        for (final PlayerUpgradeName name : PlayerUpgradeName.values()) {
            toSave.put(name, false);
        }
        toSave.put(PlayerUpgradeName.IMPROVED_HEALTH, true);
        toSave.put(PlayerUpgradeName.PET, true);

        service.save(toSave);
        final Map<PlayerUpgradeName, Boolean> loaded = service.load();

        assertTrue(loaded.get(PlayerUpgradeName.IMPROVED_HEALTH));
        assertTrue(loaded.get(PlayerUpgradeName.PET));
        assertFalse(loaded.get(PlayerUpgradeName.DISCOUNT));
        assertEquals(toSave, loaded);
    }

    @Test
    void loadCreatesDefaultFileWhenMissing() throws Exception {
        Files.deleteIfExists(PATH);
        final FileUpgradeService service = new FileUpgradeService();

        final Map<PlayerUpgradeName, Boolean> loaded = service.load();

        // A fresh file defaults every known upgrade to not-bought.
        assertTrue(Files.exists(PATH));
        for (final PlayerUpgradeName name : PlayerUpgradeName.values()) {
            assertFalse(loaded.get(name), name + " should default to false");
        }
    }
}
