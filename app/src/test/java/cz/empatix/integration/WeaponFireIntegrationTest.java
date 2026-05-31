package cz.empatix.integration;

import cz.empatix.audio.service.AudioService;
import cz.empatix.collision.DamageSource;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.domain.Pistol;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.guns.service.GunDamageEventComposite;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless INTEGRATION test of the offensive-fire pipeline: a real {@link Pistol} firing through a
 * REAL {@link BulletFactory} (real {@link StatisticsService} + {@link GunDamageEventComposite}) spawns
 * a real {@link cz.empatix.guns.ammo.Bullet} — a physics body in the shared headless dyn4j world that
 * carries real damage. Unlike the gun unit tests (which mock the factory), this drives the real
 * gun -> factory -> bullet -> physics chain and advances the simulation on it.
 */
@ExtendWith(HeadlessAssets.class)
class WeaponFireIntegrationTest {

    @Test
    void firingAPistolSpawnsARealDamagingBulletInTheWorld() {
        final BulletFactory bulletFactory =
                new BulletFactory(new StatisticsService(), new GunDamageEventComposite());
        final Pistol pistol = new Pistol(bulletFactory, mock(AudioService.class));

        pistol.setShooting(true);
        final boolean fired = pistol.shoot(0, 0, 100, 0); // fire toward +x

        assertTrue(fired, "a ready pistol fires");
        final List<Renderable> bullets = pistol.toRender();
        assertEquals(1, bullets.size(), "exactly one real bullet was spawned");

        final DamageSource bullet = assertInstanceOf(DamageSource.class, bullets.get(0));
        assertTrue(bullet.damage() > 0, "the spawned bullet carries damage");

        // The bullet is a body in the shared world; advancing physics + the gun is stable and the
        // bullet survives a single tick (it hasn't hit anything or expired yet).
        PhysicsService.getInstance().fixedUpdate(1.0 / 60.0);
        pistol.update(0.016);
        assertEquals(1, pistol.toRender().size(), "the live bullet persists after one simulation tick");
    }
}
