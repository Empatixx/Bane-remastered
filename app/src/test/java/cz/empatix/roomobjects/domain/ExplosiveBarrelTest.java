package cz.empatix.roomobjects.domain;

import cz.empatix.collision.DamageSourceType;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;

/**
 * Headless domain-logic coverage for the real {@link ExplosiveBarrel}: damage/death semantics
 * (which on death create a real {@link cz.empatix.lightning.service.LightningService} light, run a
 * headless physics area query and play a no-op sound with OpenAL disabled), the explosion
 * sub-renderable collection guard, and the light-decay branch of {@link ExplosiveBarrel#update}.
 */
@ExtendWith(HeadlessAssets.class)
class ExplosiveBarrelTest {

    @Test
    void enemyDamageIgnoredPlayerDamageKills() {
        final ExplosiveBarrel barrel = new ExplosiveBarrel();

        assertEquals(0, barrel.hit(DamageSourceType.ENEMY, 99));
        assertFalse(barrel.isDead());

        barrel.hit(DamageSourceType.PLAYER, 4);
        assertTrue(barrel.isDead());
    }

    @Test
    void deathRunsExplosionAndLightDecayInUpdate() {
        final ExplosiveBarrel barrel = new ExplosiveBarrel();
        barrel.setPosition(50f, 50f);

        barrel.hit(DamageSourceType.PLAYER, 4); // onDeath: creates light, queries physics, plays sound

        // Drive the death + explosion + light-decay branches; must not throw headlessly.
        for (int i = 0; i < 30; i++) {
            barrel.update(0.1);
        }

        assertTrue(barrel.isDead());
    }

    @Test
    void explosionIsCollectedAsSubRenderableOnlyAfterDeath() {
        final ExplosiveBarrel barrel = new ExplosiveBarrel();
        final RenderableState before = mock(RenderableState.class);

        barrel.collectSubRenderables(before);
        verify(before, never()).add(org.mockito.ArgumentMatchers.any());

        barrel.hit(DamageSourceType.PLAYER, 4);
        final RenderableState after = mock(RenderableState.class);
        barrel.collectSubRenderables(after);
        verify(after).add(org.mockito.ArgumentMatchers.any());
    }

    @Test
    void dimensionsAndRemovability() {
        final ExplosiveBarrel barrel = new ExplosiveBarrel();

        assertEquals(64 * 3, barrel.getWidth());
        assertEquals(64 * 3, barrel.getHeight());
        assertFalse(barrel.canRemove());
    }
}
