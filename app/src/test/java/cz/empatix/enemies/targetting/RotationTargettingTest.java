package cz.empatix.enemies.targetting;

import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class RotationTargettingTest {

    @Test
    void rotationIsZeroWhenNoTargetSet() {
        final RotationTargetting targetting =
                new RotationTargetting(() -> new Vector2f(10, 20), 16, 16);
        assertEquals(0f, targetting.getRotation(),
                "With no target the rotation must default to 0");
    }

    @Test
    void targetingNullClearsTargetAndReturnsZero() {
        final RotationTargetting targetting =
                new RotationTargetting(() -> new Vector2f(0, 0), 8, 8);
        targetting.target(null);
        assertEquals(0f, targetting.getRotation());
    }
}
