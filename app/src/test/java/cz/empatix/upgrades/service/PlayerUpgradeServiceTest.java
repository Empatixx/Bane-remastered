package cz.empatix.upgrades.service;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.AfterAll;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Coverage of {@link PlayerUpgradeService}'s headless-safe behaviour: the active flag toggling
 * (which also flips the {@code GaussianBlur} overlay), the active-gated {@code update}/{@code
 * mousePressed} no-ops, and {@code save} persisting the three trees' bought state. Construction
 * builds the full menu + three real trees (texture/font fetches), so everything runs under a
 * {@link MockedAssets}-stubbed static; the upgrades.bin file is snapshotted and restored.
 */
@ExtendWith(HeadlessAssets.class)
class PlayerUpgradeServiceTest {

    private static final Path PATH = Path.of("upgrades.bin");
    private static byte[] backup;

    @BeforeAll
    static void snapshotSaveFile() throws Exception {
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
    void setActiveTogglesTheActiveFlag() {
        final PlayerUpgradeService service = new PlayerUpgradeService();
        assertFalse(service.isActive());

        service.setActive(true);
        assertTrue(service.isActive());

        service.setActive(false);
        assertFalse(service.isActive());
    }

    @Test
    void updateAndMousePressedAreNoOpsWhileInactive() {
        final PlayerUpgradeService service = new PlayerUpgradeService();
        // inactive -> both methods short-circuit without touching menu/trees.
        assertDoesNotThrow(() -> service.update(0.016, 100, 100));
        assertDoesNotThrow(() -> service.mousePressed(0, 100, 100));
    }

    @Test
    void updateAndMousePressedRunWhenActive() {
        final PlayerUpgradeService service = new PlayerUpgradeService();
        service.setActive(true);

        // Active path walks menu + every tree; nothing GL-bound here.
        assertDoesNotThrow(() -> service.update(0.016, 400, 200));
        assertDoesNotThrow(() -> service.mousePressed(0, 400, 200));
    }

    @Test
    void saveWritesUpgradeStateToFile() {
        final PlayerUpgradeService service = new PlayerUpgradeService();
        assertDoesNotThrow(service::save);
        assertTrue(Files.exists(PATH));
    }
}
