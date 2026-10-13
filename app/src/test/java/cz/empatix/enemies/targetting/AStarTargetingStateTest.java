package cz.empatix.enemies.targetting;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

class AStarTargetingStateTest {

    @Test
    void hasAllExpectedStates() {
        assertEquals(4, AStarTargetingState.values().length);
    }

    @Test
    void valueOfRoundTripsForEveryConstant() {
        for (final AStarTargetingState s : AStarTargetingState.values()) {
            assertSame(s, AStarTargetingState.valueOf(s.name()));
        }
    }

    @Test
    void namedConstantsResolve() {
        assertEquals(AStarTargetingState.FOUND, AStarTargetingState.valueOf("FOUND"));
        assertEquals(AStarTargetingState.NOT_FOUND, AStarTargetingState.valueOf("NOT_FOUND"));
        assertEquals(AStarTargetingState.CLOSE, AStarTargetingState.valueOf("CLOSE"));
        assertEquals(AStarTargetingState.CACHED, AStarTargetingState.valueOf("CACHED"));
    }
}
