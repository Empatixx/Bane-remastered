package cz.empatix.collision;

import org.junit.jupiter.api.Test;

import java.util.EnumSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Guards the hand-maintained collision matrix in {@link CollisionType}. The matrix is
 * easy to break silently when adding a category, so these tests pin the relationships.
 */
class CollisionTypeTest {

    /** Categories whose collision relation is expected to be mutual (combat bodies). */
    private static final Set<CollisionType> COMBAT = EnumSet.of(
            CollisionType.PLAYER,
            CollisionType.ENEMY,
            CollisionType.PLAYER_BULLET,
            CollisionType.ENEMY_BULLET,
            CollisionType.ROOM_OBJECT);

    @Test
    void combatCategoriesCollideSymmetrically() {
        for (final CollisionType a : COMBAT) {
            for (final CollisionType b : COMBAT) {
                assertEquals(a.collidesWith(b), b.collidesWith(a),
                        () -> "asymmetric combat collision between " + a + " and " + b);
            }
        }
    }

    @Test
    void playerCollidesWithThreatsButNotEnemyBody() {
        assertTrue(CollisionType.PLAYER.collidesWith(CollisionType.ENEMY_BULLET));
        assertTrue(CollisionType.PLAYER.collidesWith(CollisionType.TILE));
        assertTrue(CollisionType.PLAYER.collidesWith(CollisionType.ROOM_OBJECT));
        // players pass through enemy bodies and friendly bullets
        assertFalse(CollisionType.PLAYER.collidesWith(CollisionType.ENEMY));
        assertFalse(CollisionType.PLAYER.collidesWith(CollisionType.PLAYER_BULLET));
    }

    @Test
    void noneCollidesWithNothing() {
        for (final CollisionType other : CollisionType.values()) {
            assertFalse(CollisionType.NONE.collidesWith(other),
                    () -> "NONE must not collide with " + other);
        }
    }

    /**
     * Characterization: ITEM/TRAP only stop on tiles, and that relation is intentionally
     * one-directional — TILE does not list ITEM/TRAP back. This documents current behavior;
     * if the matrix is ever made symmetric here, update this test deliberately.
     */
    @Test
    void itemAndTrapToTileIsOneDirectional() {
        assertTrue(CollisionType.ITEM.collidesWith(CollisionType.TILE));
        assertFalse(CollisionType.TILE.collidesWith(CollisionType.ITEM));

        assertTrue(CollisionType.TRAP.collidesWith(CollisionType.TILE));
        assertFalse(CollisionType.TILE.collidesWith(CollisionType.TRAP));
    }
}
