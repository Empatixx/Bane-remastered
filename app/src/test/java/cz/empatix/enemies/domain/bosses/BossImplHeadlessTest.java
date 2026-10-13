package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.collision.DamageSourceType;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless coverage for the composition-based {@link BossImpl}: direct health/heal logic, the
 * source-filtered hit callback, and both constructors (basic + offset collider). The boss health
 * bar built in the constructor only reads the stubbed texture, so it is GL-free.
 */
@ExtendWith(HeadlessAssets.class)
class BossImplHeadlessTest {

    private static Animation animation() {
        final Animation animation = Animation.builder()
                .setTotal(1, 4)
                .addState(AnimationState.IDLE, 0, 4, 64, 64, 100)
                .texture(AssetsService.getTexture("ingame/test.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        return animation;
    }

    @Test
    void directHitReducesHealthAndReportsReceivedAmount() {
        final BossImpl boss = new BossImpl(64, 64, 50, animation(), 3, d -> d, () -> 1, 5);

        assertEquals(20, boss.hit(20), "hit returns the damage actually taken");
        assertEquals(30, boss.getHealth());
        assertFalse(boss.isDead());

        assertEquals(1000, boss.hit(1000), "hit returns the requested damage even on overkill");
        assertEquals(0, boss.getHealth(), "health floors at zero");
        assertTrue(boss.isDead());

        assertEquals(0, boss.hit(5), "hitting a dead boss returns zero");
    }

    @Test
    void hitSourceCallbackOnlyFiresForNonEnemyDamage() {
        final AtomicInteger total = new AtomicInteger();
        final BossImpl boss = new BossImpl(64, 64, 50, animation(), 3,
                d -> { total.addAndGet(d); return d; }, () -> 1, 5);

        boss.hit(DamageSourceType.ENEMY, 8);
        assertEquals(0, total.get(), "enemy-sourced damage does not route through the callback");

        boss.hit(DamageSourceType.PLAYER, 8);
        assertEquals(8, total.get(), "player-sourced damage routes through the callback");
    }

    @Test
    void healIsCappedAtMaxAndIgnoredWhenDead() {
        final BossImpl boss = new BossImpl(64, 64, 50, animation(), 3, d -> d, () -> 1, 5);
        boss.hit(30); // -> 20

        boss.heal(1000);
        assertEquals(50, boss.getHealth(), "heal capped at max");

        boss.hit(50); // kill
        boss.heal(10);
        assertEquals(0, boss.getHealth(), "dead boss cannot be healed");
    }

    @Test
    void gettersAndPhysicsAccessorReflectConstruction() {
        final BossImpl boss = new BossImpl(64, 64, 50, animation(), 7, d -> d, () -> 1, 5);
        boss.setPosition(80f, 120f);

        assertEquals(64, boss.getWidth());
        assertEquals(64, boss.getHeight());
        assertEquals(7, boss.damage());
        assertEquals(DamageSourceType.ENEMY, boss.type());
        assertEquals(80f, boss.getX(), 0.0001f);
        assertEquals(120f, boss.getY(), 0.0001f);
        assertNotNull(boss.getPhysicsObject(), "exposes its physics body");
    }

    @Test
    void offsetColliderConstructorSharesDamageLogic() {
        final BossImpl boss = new BossImpl(64, 64, 12, animation(), 2, d -> d, () -> 1,
                30, 30, new Vector2i(10, 10), 5);

        assertFalse(boss.isDead());
        boss.hit(12);
        assertTrue(boss.isDead(), "offset-collider boss uses the same health logic");
    }

    @Test
    void isRightFacingReflectsTargetSide() {
        final BossImpl boss = new BossImpl(64, 64, 50, animation(), 3, d -> d, () -> 1, 5);
        boss.setPosition(500f, 0f);
        // A mock Player (Targetable) at origin (mocked getX/getWidth default to 0) sits left of
        // the boss; Player's static AudioService init is GL-free because ALInit is disabled.
        boss.setTarget(mock(cz.empatix.player.Player.class));

        boss.update(0.016);
        assertTrue(boss.isRightFacing(), "target to the left is reported as left-of-enemy");
    }
}
