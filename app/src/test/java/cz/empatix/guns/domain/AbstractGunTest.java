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
        BareGun() {
            super(AudioService.getInstance());
        }

        @Override
        protected SoundEffect reloadSound() {
            return SoundEffect.PISTOL_RELOAD;
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
}
