package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.concurrent.atomic.AtomicBoolean;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.mockito.Mockito.mockStatic;

/**
 * Construction + state-update coverage of {@link TalentPath}. The constructor fetches the
 * talent-path texture through {@link AssetsService}; {@code update}/{@code setImage} only re-pick
 * the animation frame from the {@code isLocked} supplier and tree type (no GL output), so we drive
 * every locked/unlocked branch and assert the calls complete without error.
 */
class TalentPathTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    @Test
    void constructsForEveryTreeTypeWhenUnlocked() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            for (final TreeType type : TreeType.values()) {
                assertDoesNotThrow(() -> new TalentPath(type, () -> false));
            }
        }
    }

    @Test
    void constructsLockedRegardlessOfType() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            for (final TreeType type : TreeType.values()) {
                assertDoesNotThrow(() -> new TalentPath(type, () -> true));
            }
        }
    }

    @Test
    void updateReevaluatesLockStateFromSupplier() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final AtomicBoolean locked = new AtomicBoolean(true);
            final TalentPath path = new TalentPath(TreeType.OFFENSIVE, locked::get);

            // Flip from locked -> unlocked and back; update() must keep working through both branches.
            assertDoesNotThrow(() -> path.update(0.016));
            locked.set(false);
            assertDoesNotThrow(() -> path.update(0.016));
            locked.set(true);
            assertDoesNotThrow(() -> path.update(0.016));
        }
    }

    @Test
    void setPositionDoesNotThrow() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final TalentPath path = new TalentPath(TreeType.DEFENSIVE, () -> false);
            assertDoesNotThrow(() -> path.setPosition(10, 20));
        }
    }
}
