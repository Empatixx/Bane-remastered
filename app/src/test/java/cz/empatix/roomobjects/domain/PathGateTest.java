package cz.empatix.roomobjects.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Headless coverage for the real {@link PathGate} state machine. The gate's collision type and
 * animation state are private, so these tests drive every public transition
 * ({@code setDirection}, {@code setState(open/closed)} for all three {@link PathGateDirection}s and
 * the open/close/idle update branches) and assert they run without throwing plus the stable getters.
 */
@ExtendWith(HeadlessAssets.class)
class PathGateTest {

    @Test
    void closeAndOpenTransitionsDriveAnimationBranchesForEveryDirection() {
        for (final PathGateDirection direction : PathGateDirection.values()) {
            final PathGate gate = new PathGate();
            gate.setDirection(direction);

            gate.setState(true); // close -> drives CLOSE animation branch towards IDLE
            for (int i = 0; i < 12; i++) {
                gate.update(0.05);
            }
            gate.setState(false); // reopen
            for (int i = 0; i < 12; i++) {
                gate.update(0.05);
            }

            assertFalse(gate.canRemove());
        }
    }

    @Test
    void freshGateUpdatesOnOpenStateWithoutThrowing() {
        final PathGate gate = new PathGate();

        for (int i = 0; i < 10; i++) {
            gate.update(0.05);
        }

        assertFalse(gate.canRemove());
    }

    @Test
    void geometryAndPosition() {
        final PathGate gate = new PathGate();
        gate.setPosition(12f, 34f);

        assertEquals(32 * 2, gate.getWidth());
        assertEquals(32 * 2, gate.getHeight());
        assertEquals(12f, gate.getPosition().x, 0.0001f);
        assertEquals(34f, gate.getPosition().y, 0.0001f);
    }
}
