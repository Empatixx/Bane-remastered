package cz.empatix.integration;

import cz.empatix.animation.AnimationState;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.collision.DamageHandler;
import cz.empatix.enemies.domain.EnemyImpl;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless INTEGRATION test of the combat pipeline: a real {@link Player} and a real {@link EnemyImpl}
 * share one real (headless) dyn4j world via {@link PhysicsService}, with the real {@link DamageHandler}
 * registered. Stepping the world makes their overlapping bodies collide, and the collision must route
 * through DamageHandler so the enemy (a DamageSource) actually damages the player (a Damageable).
 *
 * <p>Unlike the unit tests (which call hit() directly), this exercises the whole physics -> collision
 * dispatch -> damage chain wired together with real objects.
 */
@ExtendWith(HeadlessAssets.class)
class CombatIntegrationTest {

    private static final double STEP = 1.0 / 60.0;

    private static Animation enemyAnimation() {
        final Animation animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, 32, 32, 100)
                .texture(AssetsService.getTexture("ingame/test.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        return animation;
    }

    /** EnemyImpl has no spawn-invincibility window, so its contact damage lands immediately. */
    private static EnemyImpl enemyDealing(final int damage) {
        return new EnemyImpl(40, 40, 100, enemyAnimation(), damage, d -> { },
                () -> false, () -> 1_000_000, 5);
    }

    @Test
    void enemyContactDamagesThePlayerThroughThePhysicsCollisionPipeline() {
        final PhysicsService physics = PhysicsService.getInstance(); // the world HeadlessAssets built
        physics.addCollisionHandler(new DamageHandler());

        final Player player = new Player();
        final EnemyImpl enemy = enemyDealing(7);

        // Stack both bodies on the origin so their fixtures overlap.
        player.setPosition(0, 0);
        enemy.setPosition(0, 0);

        final int vitalityBefore = player.getHealth() + player.getArmor();

        for (int i = 0; i < 8; i++) {
            physics.fixedUpdate(STEP);
        }

        final int vitalityAfter = player.getHealth() + player.getArmor();
        assertTrue(vitalityAfter < vitalityBefore,
                "colliding with an enemy must cost the player health/armor (was "
                        + vitalityBefore + ", now " + vitalityAfter + ")");
    }

    @Test
    void repeatedContactInOneInvincibilityWindowOnlyLandsOnce() {
        final PhysicsService physics = PhysicsService.getInstance();
        physics.addCollisionHandler(new DamageHandler());

        final Player player = new Player();
        final EnemyImpl enemy = enemyDealing(7);

        player.setPosition(0, 0);
        enemy.setPosition(0, 0);

        final int vitalityBefore = player.getHealth() + player.getArmor();

        // Many steps land many collisions, but the player's invincibility window must collapse them
        // into a single applied hit (the steps run far quicker than the ~1s window).
        for (int i = 0; i < 20; i++) {
            physics.fixedUpdate(STEP);
        }

        final int lost = vitalityBefore - (player.getHealth() + player.getArmor());
        assertEquals(7, lost, "only one 7-damage hit should land within a single invincibility window");
    }

    @Test
    void aHarmlessEnemyLeavesThePlayerUntouched() {
        final PhysicsService physics = PhysicsService.getInstance();
        physics.addCollisionHandler(new DamageHandler());

        final Player player = new Player();
        final EnemyImpl enemy = enemyDealing(0); // zero damage -> DamageHandler short-circuits

        player.setPosition(0, 0);
        enemy.setPosition(0, 0);

        final int vitalityBefore = player.getHealth() + player.getArmor();

        for (int i = 0; i < 8; i++) {
            physics.fixedUpdate(STEP);
        }

        assertFalse(player.getHealth() + player.getArmor() < vitalityBefore,
                "a zero-damage source must not reduce the player's vitality");
    }
}
