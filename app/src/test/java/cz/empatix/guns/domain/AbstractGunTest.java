package cz.empatix.guns.domain;

import cz.empatix.audio.init.ALInit;
import cz.empatix.audio.service.AudioService;
import cz.empatix.sound.SoundEffect;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Exercises the shared ammo + reload logic in {@link AbstractGun}. This is only reachable
 * headlessly because AudioService is injected (no static getInstance at class load) and
 * AudioService.play() is a no-op without an OpenAL context. A minimal HUD-less subclass
 * lets us drive the logic directly.
 */
class AbstractGunTest {

    @BeforeAll
    static void disableOpenAl() {
        // No OpenAL device in the test JVM: this makes AudioService construction and
        // play() short-circuit to no-ops instead of touching native AL.
        ALInit.canUseAL = false;
    }

    /** Bare gun with no rendering/HUD; audio is the headless no-op singleton. */
    private static final class BareGun extends AbstractGun {
        int spawned;
        boolean semiAuto = true;

        BareGun() {
            super(AudioService.getInstance());
        }

        @Override
        protected SoundEffect reloadSound() {
            return SoundEffect.PISTOL_RELOAD;
        }

        @Override
        protected SoundEffect shootSound() {
            return SoundEffect.PISTOL_SHOOT;
        }

        @Override
        protected void spawnBullets(final float shooterX, final float shooterY,
                                    final float targetX, final float targetY) {
            spawned++;
        }

        @Override
        protected boolean isSemiAuto() {
            return semiAuto;
        }
    }

    // --- reloadAmmo() -------------------------------------------------------

    @Test
    void reloadFillsClipWhenAmmoSufficient() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 120;

        gun.reloadAmmo();

        assertEquals(7, gun.currentClip);
        assertEquals(115, gun.ammo); // spent 5 to top up the clip
    }

    @Test
    void reloadUsesAllRemainingAmmoWhenInsufficient() {
        final BareGun gun = new BareGun();
        gun.clipSize = 10;
        gun.currentClip = 3;
        gun.ammo = 4;

        gun.reloadAmmo();

        assertEquals(7, gun.currentClip); // 3 + the 4 remaining
        assertEquals(0, gun.ammo);
    }

    @Test
    void reloadIsNoOpWhenClipAlreadyFull() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 7;
        gun.ammo = 120;

        gun.reloadAmmo();

        assertEquals(7, gun.currentClip);
        assertEquals(120, gun.ammo);
    }

    // --- reload() trigger ---------------------------------------------------

    @Test
    void reloadStartsWhenAmmoAvailableAndClipNotFull() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 10;

        gun.reload();

        assertTrue(gun.reloading);
        assertTrue(gun.lastReloadTime > 0);
    }

    @Test
    void reloadDoesNotStartWhenClipFull() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 7;
        gun.ammo = 10;

        gun.reload();

        assertFalse(gun.reloading);
    }

    @Test
    void reloadDoesNotStartWithoutSpareAmmo() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 0;

        gun.reload();

        assertFalse(gun.reloading);
    }

    @Test
    void reloadIsIgnoredWhileAlreadyReloading() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 10;
        gun.reloading = true;
        gun.lastReloadTime = 12345L;

        gun.reload();

        assertTrue(gun.reloading);
        assertEquals(12345L, gun.lastReloadTime); // untouched
    }

    // --- updateReload() completion -----------------------------------------

    @Test
    void updateReloadCompletesAfterReloadTimeAndTopsUpClip() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 10;
        gun.reloadTime = 100;
        gun.reloading = true;
        gun.lastReloadTime = System.currentTimeMillis() - 1000; // well past reloadTime

        gun.updateReload();

        assertFalse(gun.reloading);
        assertEquals(7, gun.currentClip);
        assertEquals(5, gun.ammo);
    }

    @Test
    void updateReloadStaysReloadingBeforeReloadTimeElapses() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 2;
        gun.ammo = 10;
        gun.reloadTime = 100_000;
        gun.reloading = true;
        gun.lastReloadTime = System.currentTimeMillis();

        gun.updateReload();

        assertTrue(gun.reloading);
        assertEquals(2, gun.currentClip); // not topped up yet
    }

    // --- shoot() template ---------------------------------------------------

    private static BareGun readyGun() {
        final BareGun gun = new BareGun();
        gun.clipSize = 7;
        gun.currentClip = 5;
        gun.ammo = 50;
        gun.fireRateTime = 250;
        gun.shooting = true;
        gun.lastShootTime = 0; // long in the past relative to now
        return gun;
    }

    @Test
    void shootSpawnsBulletAndConsumesClipWhenReady() {
        final BareGun gun = readyGun();

        final boolean fired = gun.shoot(0, 0, 1, 1);

        assertTrue(fired);
        assertEquals(1, gun.spawned);
        assertEquals(4, gun.currentClip);
        assertTrue(gun.lastShootTime > 0);
        assertFalse(gun.shooting); // semi-auto clears the trigger after one shot
    }

    @Test
    void shootDoesNothingWhenNotShooting() {
        final BareGun gun = readyGun();
        gun.shooting = false;

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertEquals(0, gun.spawned);
        assertEquals(5, gun.currentClip);
    }

    @Test
    void shootDoesNothingWhileReloading() {
        final BareGun gun = readyGun();
        gun.reloading = true;

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertEquals(0, gun.spawned);
    }

    @Test
    void shootRespectsFireRate() {
        final BareGun gun = readyGun();
        gun.fireRateTime = 100_000;
        gun.lastShootTime = System.currentTimeMillis();

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertEquals(0, gun.spawned);
        assertEquals(5, gun.currentClip);
    }

    @Test
    void shootOnEmptyClipDoesNotSpawn() {
        final BareGun gun = readyGun();
        gun.currentClip = 0;

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertEquals(0, gun.spawned);
        assertFalse(gun.shooting); // semi-auto still clears the trigger on a dry click
    }

    @Test
    void fullAutoKeepsTriggerHeldAfterFiring() {
        final BareGun gun = readyGun();
        gun.semiAuto = false;

        assertTrue(gun.shoot(0, 0, 1, 1));
        assertTrue(gun.shooting); // full-auto keeps firing while held
    }

    @Test
    void fullAutoEmptyClickThrottlesViaLastShootTime() {
        final BareGun gun = readyGun();
        gun.semiAuto = false;
        gun.currentClip = 0;

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertTrue(gun.lastShootTime > 0); // empty click resets the fire-rate gate
        assertTrue(gun.shooting); // trigger stays held
    }

    @Test
    void semiAutoEmptyClickDoesNotResetFireRateGate() {
        final BareGun gun = readyGun();
        gun.currentClip = 0;
        gun.lastShootTime = 0;

        assertFalse(gun.shoot(0, 0, 1, 1));
        assertEquals(0, gun.lastShootTime); // semi-auto relies on the trigger reset, not the timer
    }
}
