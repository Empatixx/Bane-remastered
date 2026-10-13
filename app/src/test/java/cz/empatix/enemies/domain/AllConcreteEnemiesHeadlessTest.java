package cz.empatix.enemies.domain;

import cz.empatix.collision.DamageSourceType;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.extension.ExtendWith;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.function.Supplier;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Constructs every concrete common {@link BaseEnemy} as the REAL class headlessly and drives its
 * shared damage/spawn/death logic. {@code spawnTime} is the protected field declared on
 * {@code BaseEnemy} (this test lives in the same package), so the 2s spawn-invincibility window is
 * forced open by assignment instead of sleeping. Each enemy is also positioned, given a target and
 * ticked once so the A* targeting direction can be sampled finite/sane.
 */
@ExtendWith(HeadlessAssets.class)
class AllConcreteEnemiesHeadlessTest {

    static Stream<Arguments> enemies() {
        return Stream.of(
                Arguments.of("Bat", (Supplier<BaseEnemy>) Bat::new),
                Arguments.of("Rat", (Supplier<BaseEnemy>) Rat::new),
                Arguments.of("Snake", (Supplier<BaseEnemy>) Snake::new),
                Arguments.of("Ghost", (Supplier<BaseEnemy>) Ghost::new),
                Arguments.of("DemonEye", (Supplier<BaseEnemy>) DemonEye::new),
                Arguments.of("Slime", (Supplier<BaseEnemy>) Slime::new),
                Arguments.of("RedSlime", (Supplier<BaseEnemy>) RedSlime::new),
                Arguments.of("EyeWatcher", (Supplier<BaseEnemy>) EyeWatcher::new),
                Arguments.of("GreenSpiderling", (Supplier<BaseEnemy>) GreenSpiderling::new),
                Arguments.of("RedSpiderling", (Supplier<BaseEnemy>) RedSpiderling::new),
                Arguments.of("PurpleSpiderling", (Supplier<BaseEnemy>) PurpleSpiderling::new)
        );
    }

    /** Player implements Targetable; a fake at a fixed spot is enough to drive A*. */
    private static cz.empatix.enemies.targetting.Targetable target(final float x, final float y) {
        return new cz.empatix.enemies.targetting.Targetable() {
            @Override
            public Vector2f getPosition() {
                return new Vector2f(x, y);
            }

            @Override
            public int getWidth() {
                return 48;
            }

            @Override
            public int getHeight() {
                return 48;
            }
        };
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void constructsAliveAndNotRemovable(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();

        assertFalse(enemy.isDead(), name + " starts alive");
        assertTrue(enemy.getHealth() > 0, name + " starts with positive health");
        assertTrue(enemy.getWidth() > 0 && enemy.getHeight() > 0, name + " has positive dimensions");
        assertFalse(enemy.canRemove(), name + " is not immediately removable");
        assertEquals(DamageSourceType.ENEMY, enemy.type(), name + " is an ENEMY damage source");
        assertTrue(enemy.isSpawning(), name + " is spawning right after construction");
        assertEquals(0, enemy.damage(), name + " deals no contact damage while spawning");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void takesPlayerDamageAndDiesWhenDepleted(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();
        enemy.spawnTime = 0; // leave spawn-invincibility

        final int startHealth = enemy.getHealth();
        final int received = enemy.hit(DamageSourceType.PLAYER, 1);
        assertEquals(1, received, name + " registers a 1-point player hit");
        assertEquals(startHealth - 1, enemy.getHealth(), name + " loses exactly the damage dealt");
        assertFalse(enemy.isDead(), name + " survives a single point of damage");

        // Drain the rest.
        enemy.hit(DamageSourceType.PLAYER, startHealth);
        assertTrue(enemy.isDead(), name + " dies once health is depleted");
        assertEquals(0, enemy.getHealth(), name + " health clamps to zero");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void enemySourcedDamageIsIgnored(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();
        enemy.spawnTime = 0;
        final int startHealth = enemy.getHealth();

        assertEquals(0, enemy.hit(DamageSourceType.ENEMY, 5), name + " ignores enemy-sourced damage");
        assertEquals(startHealth, enemy.getHealth(), name + " keeps full health");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void targetingProducesFiniteDirectionAfterUpdate(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();
        enemy.spawnTime = 0;
        enemy.setPosition(0f, 0f);
        enemy.aStarTargetting.target(target(300f, 0f));

        // updatePath then one update poll so getDirectionTarget is non-zero (known gotcha).
        enemy.aStarTargetting.updatePath();
        enemy.aStarTargetting.update(0.016f);
        final Vector2f direction = enemy.aStarTargetting.getDirectionTarget();

        assertTrue(Float.isFinite(direction.x) && Float.isFinite(direction.y),
                name + " direction is finite, got " + direction);
        assertTrue(direction.x >= 0, name + " heads toward a target on the right, got " + direction);
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void updateAndFixedUpdateAreNoOpsWhileSpawning(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();
        enemy.setPosition(10f, 10f);
        enemy.aStarTargetting.target(target(200f, 200f));

        // While spawning, update/fixedUpdate early-return without animation/path work.
        enemy.update(0.016);
        enemy.fixedUpdate(0.016);

        assertTrue(enemy.isSpawning(), name + " remains spawning");
        assertFalse(enemy.isDead());
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("enemies")
    void tickingAfterSpawnDoesNotThrowAndCollectsSubRenderables(final String name, final Supplier<BaseEnemy> factory) {
        final BaseEnemy enemy = factory.get();
        enemy.spawnTime = 0;
        enemy.setPosition(0f, 0f);
        enemy.aStarTargetting.target(target(120f, 0f));

        enemy.fixedUpdate(0.016);
        enemy.update(0.016);

        final RenderableState state = new RenderableState(8);
        enemy.collectSubRenderables(state);
        // Smoke check: the call path executed without GL/font access blowing up.
        assertFalse(enemy.isDead(), name + " stays alive after a benign tick");
    }
}
