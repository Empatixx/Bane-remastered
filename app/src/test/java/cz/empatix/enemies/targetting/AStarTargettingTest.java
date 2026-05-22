package cz.empatix.enemies.targetting;

import cz.empatix.newphysics.service.PhysicsService;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

class AStarTargettingTest {

    /** Stub Targetable at a fixed position/size. */
    private static final class FakeTargetable implements Targetable {
        private final Vector2f position;
        private final int width;
        private final int height;

        FakeTargetable(final float x, final float y, final int width, final int height) {
            this.position = new Vector2f(x, y);
            this.width = width;
            this.height = height;
        }

        @Override
        public Vector2f getPosition() {
            return position;
        }

        @Override
        public int getWidth() {
            return width;
        }

        @Override
        public int getHeight() {
            return height;
        }
    }

    @BeforeEach
    void setUp() {
        // Empty world: no obstacles, so A* cells are never blocked.
        new PhysicsService();
    }

    @Test
    void hasNoTargetBeforeOneIsSet() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);

        assertFalse(targetting.hasTarget());
    }

    @Test
    void hasTargetAfterOneIsSet() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);

        targetting.target(new FakeTargetable(100, 100, 16, 16));

        assertTrue(targetting.hasTarget());
    }

    @Test
    void updatePathReturnsNotFoundWithoutTarget() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);

        assertEquals(AStarTargetingState.NOT_FOUND, targetting.updatePath());
    }

    @Test
    void updatePathReturnsCloseWhenTargetHasNotMovedSinceLastQuery() {
        // First query records the target position (lastTargetX/Y start at MAX_VALUE so the
        // first call is never CLOSE); the second query with an unmoved target is CLOSE.
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(200, 0, 16, 16));

        targetting.updatePath();

        assertEquals(AStarTargetingState.CLOSE, targetting.updatePath());
    }

    @Test
    void updatePathFindsPathToDistantTargetInEmptyWorld() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(64, 0, 16, 16));

        final AStarTargetingState state = targetting.updatePath();

        // In an empty world there are no obstacles, so a path to the target exists.
        assertEquals(AStarTargetingState.FOUND, state);
    }

    @Test
    void directionPointsTowardTargetToTheRight() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(200, 0, 16, 16));
        targetting.updatePath();
        // The path's first cell is the start position itself; advancing polls it off so the
        // next cell (and thus the direction) points toward the target.
        targetting.update(0.016f);

        final Vector2f direction = targetting.getDirectionTarget();

        assertNotNull(direction);
        // Target is to the right -> positive x component dominates.
        assertTrue(direction.x > 0, "expected rightward direction, got " + direction);
    }

    @Test
    void directionPointsTowardTargetUpward() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(0, 200, 16, 16));
        targetting.updatePath();
        // Poll off the start cell so the next path cell points toward the target.
        targetting.update(0.016f);

        final Vector2f direction = targetting.getDirectionTarget();

        assertTrue(direction.y > 0, "expected upward direction, got " + direction);
    }

    @Test
    void directionIsNormalizedWhenNoPath() {
        // Without calling updatePath the path is empty -> direction computed straight to target.
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(300, 400, 16, 16));

        final Vector2f direction = targetting.getDirectionTarget();

        assertEquals(1f, direction.length(), 1e-3, "direction should be unit length");
        assertTrue(direction.x > 0 && direction.y > 0);
    }

    @Test
    void isTargetLeftFromEnemyReflectsRelativePosition() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(100, 0), 16, 16);

        targetting.target(new FakeTargetable(0, 0, 16, 16));
        assertTrue(targetting.isTargetLeftFromEnemy(), "target left of enemy");

        targetting.target(new FakeTargetable(500, 0, 16, 16));
        assertFalse(targetting.isTargetLeftFromEnemy(), "target right of enemy");
    }

    @Test
    void updateDoesNotThrowWithEmptyPath() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(200, 0, 16, 16));

        // No updatePath call yet -> path empty -> update is a no-op.
        targetting.update(0.016f);
    }

    @Test
    void updateAdvancesAlongFoundPath() {
        final AStarTargetting targetting = new AStarTargetting(() -> new Vector2f(0, 0), 16, 16);
        targetting.target(new FakeTargetable(64, 0, 16, 16));
        targetting.updatePath();

        // Advancing should not throw and keeps producing a valid direction.
        targetting.update(0.016f);
        assertNotNull(targetting.getDirectionTarget());
    }
}
