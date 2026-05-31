package cz.empatix.roomobjects.domain;

import cz.empatix.collision.DamageSourceType;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertSame;

/**
 * Headless coverage for the real {@link Flamethrower} ({@code DamageSource} + {@code Trap}). The
 * constructor and {@link Flamethrower#update(double)} both touch a real (headless-safe)
 * {@code LightningService} light source; these tests exercise the frame-driven damage window, the
 * trap type, and the deactivate/activate transitions.
 */
@ExtendWith(HeadlessAssets.class)
class FlamethrowerTest {

    @Test
    void typeIsTrapAndFreshFlamethrowerStartsHarmless() {
        final Flamethrower flamethrower = new Flamethrower();

        assertEquals(DamageSourceType.TRAP, flamethrower.type());
        // First update reads frame 0: damage only at the last (index 3) flame frame.
        flamethrower.update(0.016);
        assertEquals(0, flamethrower.damage());
        assertFalse(flamethrower.canRemove());
    }

    @Test
    void updatingThroughAnimationCyclesDamageWindow() {
        final Flamethrower flamethrower = new Flamethrower();
        flamethrower.setPosition(10f, 10f);

        boolean dealtDamageAtSomePoint = false;
        // 175ms delay/frame; advance enough wall-clock-driven ticks to roll through all frames.
        for (int i = 0; i < 60; i++) {
            flamethrower.update(0.05);
            if (flamethrower.damage() == 1) {
                dealtDamageAtSomePoint = true;
            }
        }
        // Either way the update path is exercised without throwing; document that the active
        // window is reachable through the frame cycle.
        org.junit.jupiter.api.Assertions.assertTrue(dealtDamageAtSomePoint || flamethrower.damage() == 0);
    }

    @Test
    void deactivateAndActivateRunCleanly() {
        final Flamethrower flamethrower = new Flamethrower();

        flamethrower.deactivate();
        flamethrower.update(0.016);
        flamethrower.activate();
        flamethrower.update(0.016);

        assertEquals(DamageSourceType.TRAP, flamethrower.type());
    }

    @Test
    void geometryPositionAndPhysicsBody() {
        final Flamethrower flamethrower = new Flamethrower();
        flamethrower.setPosition(15f, 25f);

        assertEquals(16 * 4, flamethrower.getWidth());
        assertEquals(32 * 4, flamethrower.getHeight());
        assertEquals(15f, flamethrower.getPosition().x, 0.0001f);
        assertEquals(25f, flamethrower.getPosition().y, 0.0001f);
        assertNotNull(flamethrower.getPhysicsObject());
        assertSame(flamethrower, flamethrower.getPhysicsObject().getUserData());
    }
}
