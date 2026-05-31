package cz.empatix.guns.domain;

import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.ammo.Bullet;
import cz.empatix.guns.factory.BulletFactory;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.extension.ExtendWith;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.params.provider.Arguments.arguments;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Constructs and exercises every concrete {@link AbstractGun} as the REAL class (not a test double),
 * headlessly, by stubbing the static asset accessors its HUD sub-objects touch in their constructors.
 * BulletFactory is mocked so no physics/lighting-coupled {@link Bullet} is built.
 */
@ExtendWith(HeadlessAssets.class)
class AllGunsHeadlessTest {

    @FunctionalInterface
    interface GunFactory {
        AbstractGun create(BulletFactory bulletFactory, AudioService audioService);
    }

    static Stream<Arguments> guns() {
        return Stream.of(
                arguments("Pistol", (GunFactory) Pistol::new),
                arguments("Revolver", (GunFactory) Revolver::new),
                arguments("Luger", (GunFactory) Luger::new),
                arguments("Shotgun", (GunFactory) Shotgun::new),
                arguments("Thompson", (GunFactory) Thompson::new),
                arguments("Uzi", (GunFactory) Uzi::new),
                arguments("M4", (GunFactory) M4::new)
        );
    }

    private static BulletFactory bulletFactoryReturningMocks() {
        final BulletFactory bulletFactory = mock(BulletFactory.class);
        when(bulletFactory.createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat()))
                .thenReturn(mock(Bullet.class));
        when(bulletFactory.createBullet(anyInt(), anyInt(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat(), anyFloat()))
                .thenReturn(mock(Bullet.class));
        return bulletFactory;
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("guns")
    void constructsWithFullClipAndAmmo(final String name, final GunFactory factory) {
        final AbstractGun gun = factory.create(bulletFactoryReturningMocks(), mock(AudioService.class));
        final Gun g = (Gun) gun; // Gun-interface methods live on the concrete classes, fields on AbstractGun

        assertNotNull(g.getType(), name + " should expose a gun type");
        assertTrue(g.getMaxAmmo() > 0, name + " should have positive max ammo");
        assertEquals(gun.clipSize, gun.currentClip, name + " starts with a full clip");
        assertTrue(g.hasFullAmmo(), name + " starts with full ammo");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("guns")
    void firingConsumesOneRoundFromTheClip(final String name, final GunFactory factory) {
        final AbstractGun gun = factory.create(bulletFactoryReturningMocks(), mock(AudioService.class));
        final Gun g = (Gun) gun;
        final int clipBefore = gun.currentClip;

        g.setShooting(true);
        final boolean fired = g.shoot(100, 100, 200, 200);
        g.update(0.016); // exercises the real update()/updateReload() path with mock bullets

        assertTrue(fired, name + " fires the first shot from a full clip");
        assertEquals(clipBefore - 1, gun.currentClip, name + " consumes exactly one clip round per pull");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("guns")
    void reloadRefillsClipAfterReloadTime(final String name, final GunFactory factory) {
        final AbstractGun gun = factory.create(bulletFactoryReturningMocks(), mock(AudioService.class));
        final Gun g = (Gun) gun;
        gun.currentClip = 1;
        gun.ammo = gun.maxAmmo;

        g.reload();
        assertTrue(gun.reloading, name + " enters the reloading state");

        gun.lastReloadTime = System.currentTimeMillis() - (gun.reloadTime + 1000);
        g.update(0.016);

        assertEquals(gun.clipSize, gun.currentClip, name + " tops the clip back up after reloading");
    }

    @ParameterizedTest(name = "{0}")
    @MethodSource("guns")
    void addPercentAmmoIsCappedAtMax(final String name, final GunFactory factory) {
        final AbstractGun gun = factory.create(bulletFactoryReturningMocks(), mock(AudioService.class));
        final Gun g = (Gun) gun;
        gun.ammo = 0;

        g.addPercentAmmo(10f); // way more than 100%

        assertTrue(gun.ammo <= gun.maxAmmo, name + " never exceeds max ammo");
        assertTrue(gun.ammo > 0, name + " gains ammo");
    }
}
