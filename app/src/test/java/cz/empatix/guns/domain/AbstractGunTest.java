package cz.empatix.guns.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Exercises the shared ammo arithmetic in {@link AbstractGun}. This is only reachable
 * headlessly because AudioService is now injected (no static getInstance at class load);
 * a minimal subclass with no HUD lets us drive {@link AbstractGun#reloadAmmo()} directly.
 */
class AbstractGunTest {

    /** Bare gun with no rendering/HUD; audio is unused by reloadAmmo so we pass null. */
    private static final class BareGun extends AbstractGun {
        BareGun() {
            super(null);
        }
    }

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
}
