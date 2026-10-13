package cz.empatix.upgrades.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicBoolean;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;

/**
 * Construction + state-update coverage of {@link TalentPath}. The constructor fetches the
 * talent-path texture through {@code AssetsService}; {@code update}/{@code setImage} only re-pick
 * the animation frame from the {@code isLocked} supplier and tree type (no GL output), so we drive
 * every locked/unlocked branch and assert the calls complete without error.
 */
@ExtendWith(HeadlessAssets.class)
class TalentPathTest {

    @Test
    void constructsForEveryTreeTypeWhenUnlocked() {
        for (final TreeType type : TreeType.values()) {
            assertDoesNotThrow(() -> new TalentPath(type, () -> false));
        }
    }

    @Test
    void constructsLockedRegardlessOfType() {
        for (final TreeType type : TreeType.values()) {
            assertDoesNotThrow(() -> new TalentPath(type, () -> true));
        }
    }

    @Test
    void updateReevaluatesLockStateFromSupplier() {
        final AtomicBoolean locked = new AtomicBoolean(true);
        final TalentPath path = new TalentPath(TreeType.OFFENSIVE, locked::get);

        // Flip from locked -> unlocked and back; update() must keep working through both branches.
        assertDoesNotThrow(() -> path.update(0.016));
        locked.set(false);
        assertDoesNotThrow(() -> path.update(0.016));
        locked.set(true);
        assertDoesNotThrow(() -> path.update(0.016));
    }

    @Test
    void setPositionDoesNotThrow() {
        final TalentPath path = new TalentPath(TreeType.DEFENSIVE, () -> false);
        assertDoesNotThrow(() -> path.setPosition(10, 20));
    }
}
