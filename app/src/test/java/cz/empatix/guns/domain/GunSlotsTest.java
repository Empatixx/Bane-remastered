package cz.empatix.guns.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Drives the pure slot bookkeeping of {@link GunSlots}: add/drop/switch/capacity/ammo fan-out.
 * {@link Gun} collaborators are mocked so only the slot logic is under test. The class touches
 * {@code AudioService.getInstance()} on add/switch, made headless via {@link HeadlessAssets}.
 */
@ExtendWith(HeadlessAssets.class)
class GunSlotsTest {

    @Test
    void emptySlotsThrowWhenReadingCurrentGun() {
        final GunSlots slots = new GunSlots();
        assertThrows(IllegalStateException.class, slots::getCurrentGun);
    }

    @Test
    void addGunFillsCurrentSlotThenNextFreeSlot() {
        final GunSlots slots = new GunSlots();
        final Gun first = mock(Gun.class);
        final Gun second = mock(Gun.class);

        slots.addGun(first);
        assertSame(first, slots.getCurrentGun());
        assertFalse(slots.hasFullSlots());

        slots.addGun(second);
        assertTrue(slots.hasFullSlots());
        // current slot still 0 -> still the first gun
        assertSame(first, slots.getCurrentGun());
    }

    @Test
    void switchSlotChangesCurrentGun() {
        final GunSlots slots = new GunSlots();
        final Gun first = mock(Gun.class);
        final Gun second = mock(Gun.class);
        slots.addGun(first);
        slots.addGun(second);

        slots.switchSlot(1);
        assertSame(second, slots.getCurrentGun());

        // switching to the same slot is a no-op (no throw, stays put)
        slots.switchSlot(1);
        assertSame(second, slots.getCurrentGun());
    }

    @Test
    void switchSlotIgnoresOutOfRangeIndices() {
        final GunSlots slots = new GunSlots();
        final Gun first = mock(Gun.class);
        slots.addGun(first);

        slots.switchSlot(99);  // beyond maxSlots -> ignored
        slots.switchSlot(-1);  // negative -> ignored
        assertSame(first, slots.getCurrentGun());
    }

    @Test
    void switchNextWrapsAroundSlots() {
        final GunSlots slots = new GunSlots();
        final Gun first = mock(Gun.class);
        final Gun second = mock(Gun.class);
        slots.addGun(first);
        slots.addGun(second);

        slots.switchNextSlot(); // 0 -> 1
        assertSame(second, slots.getCurrentGun());
        slots.switchNextSlot(); // 1 -> 0 (wrap)
        assertSame(first, slots.getCurrentGun());
    }

    @Test
    void switchPreviousWrapsAroundFromZero() {
        final GunSlots slots = new GunSlots();
        final Gun first = mock(Gun.class);
        final Gun second = mock(Gun.class);
        slots.addGun(first);
        slots.addGun(second);

        slots.switchPreviousSlot(); // 0 -> maxSlots-1 == 1
        assertSame(second, slots.getCurrentGun());
        slots.switchPreviousSlot(); // 1 -> 0
        assertSame(first, slots.getCurrentGun());
    }

    @Test
    void dropCurrentGunReturnsAndClearsSlot() {
        final GunSlots slots = new GunSlots();
        final Gun gun = mock(Gun.class);
        slots.addGun(gun);

        final Gun dropped = slots.dropCurrentGun();
        assertSame(gun, dropped);
        assertFalse(slots.hasFullSlots());
        assertThrows(IllegalStateException.class, slots::getCurrentGun);
    }

    @Test
    void dropCurrentGunThrowsWhenSlotEmpty() {
        final GunSlots slots = new GunSlots();
        assertThrows(IllegalStateException.class, slots::dropCurrentGun);
    }

    @Test
    void containsGunReflectsStoredGuns() {
        final GunSlots slots = new GunSlots();
        final Gun gun = mock(Gun.class);
        final Gun other = mock(Gun.class);

        assertFalse(slots.containsGun(gun));
        slots.addGun(gun);
        assertTrue(slots.containsGun(gun));
        assertFalse(slots.containsGun(other));
    }

    @Test
    void addSlotIncreasesCapacityBeyondDefaultTwo() {
        final GunSlots slots = new GunSlots();
        slots.addGun(mock(Gun.class));
        slots.addGun(mock(Gun.class));
        assertTrue(slots.hasFullSlots());

        slots.addSlot(); // third slot opens up

        assertFalse(slots.hasFullSlots());
        slots.addGun(mock(Gun.class));
        assertTrue(slots.hasFullSlots());
    }

    @Test
    void hasNotFullAmmoByTypeMatchesStoredGunOfThatType() {
        final GunSlots slots = new GunSlots();
        final Gun pistol = mock(Gun.class);
        when(pistol.getType()).thenReturn(GunType.PISTOL);
        when(pistol.hasFullAmmo()).thenReturn(false);
        slots.addGun(pistol);

        assertTrue(slots.hasNotFullAmmo(GunType.PISTOL));
        assertFalse(slots.hasNotFullAmmo(GunType.SHOTGUN));
        assertTrue(slots.hasNotFullAmmo()); // any gun not full
    }

    @Test
    void hasNotFullAmmoIsFalseWhenAllGunsFull() {
        final GunSlots slots = new GunSlots();
        final Gun pistol = mock(Gun.class);
        when(pistol.getType()).thenReturn(GunType.PISTOL);
        when(pistol.hasFullAmmo()).thenReturn(true);
        slots.addGun(pistol);

        assertFalse(slots.hasNotFullAmmo(GunType.PISTOL));
        assertFalse(slots.hasNotFullAmmo());
    }

    @Test
    void addAmmoOnlyTargetsGunsOfMatchingType() {
        final GunSlots slots = new GunSlots();
        final Gun pistol = mock(Gun.class);
        when(pistol.getType()).thenReturn(GunType.PISTOL);
        final Gun shotgun = mock(Gun.class);
        when(shotgun.getType()).thenReturn(GunType.SHOTGUN);
        slots.addGun(pistol);
        slots.addGun(shotgun);

        slots.addAmmo(GunType.PISTOL, 50f);

        verify(pistol).addPercentAmmo(50f);
        verify(shotgun, never()).addPercentAmmo(50f);
    }

    @Test
    void hasFullSlotsFalseForFreshDefaultSlots() {
        final GunSlots slots = new GunSlots();
        assertFalse(slots.hasFullSlots());
        assertEquals(false, slots.containsGun(mock(Gun.class)));
    }
}
