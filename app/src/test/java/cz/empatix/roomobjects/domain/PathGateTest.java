package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link PathGate} state machine. The gate's collision type and
 * animation state are private, so these tests drive every public transition
 * ({@code setDirection}, {@code setState(open/closed)} for all three {@link PathGateDirection}s and
 * the open/close/idle update branches) and assert they run without throwing plus the stable getters.
 */
class PathGateTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void closeAndOpenTransitionsDriveAnimationBranchesForEveryDirection() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
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
    }

    @Test
    void freshGateUpdatesOnOpenStateWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PathGate gate = new PathGate();

            for (int i = 0; i < 10; i++) {
                gate.update(0.05);
            }

            assertFalse(gate.canRemove());
        }
    }

    @Test
    void geometryAndPosition() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PathGate gate = new PathGate();
            gate.setPosition(12f, 34f);

            assertEquals(32 * 2, gate.getWidth());
            assertEquals(32 * 2, gate.getHeight());
            assertEquals(12f, gate.getPosition().x, 0.0001f);
            assertEquals(34f, gate.getPosition().y, 0.0001f);
        }
    }
}
