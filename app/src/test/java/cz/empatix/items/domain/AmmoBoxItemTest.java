package cz.empatix.items.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;
import java.util.function.BiConsumer;
import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Drives the real {@link AmmoBoxItem} pickup/effect logic headlessly (see {@link HeadlessAssets}). */
@ExtendWith(HeadlessAssets.class)
class AmmoBoxItemTest {

    @SuppressWarnings("unchecked")
    @Test
    void pickupAddsConfiguredPercentAmountAndMarksPickedUp() {
        final Predicate<AmmoBoxType> canPickUp = mock(Predicate.class);
        when(canPickUp.test(AmmoBoxType.SHOTGUN)).thenReturn(true);
        final BiConsumer<Float, AmmoBoxType> pickUp = mock(BiConsumer.class);

        final AmmoBoxItem item = new AmmoBoxItem(AmmoBoxType.SHOTGUN, canPickUp, pickUp);

        assertFalse(item.isPickedUp());
        assertTrue(item.canInteract(Set.of()));

        item.interact(Set.of());

        assertTrue(item.isPickedUp());
        assertTrue(item.canRemove()); // picked up -> removable
        // SHOTGUN box refills 25%
        verify(pickUp).accept(eq(0.25f), eq(AmmoBoxType.SHOTGUN));
    }

    @SuppressWarnings("unchecked")
    @Test
    void cannotInteractWhenGunAlreadyFull() {
        final Predicate<AmmoBoxType> canPickUp = mock(Predicate.class);
        when(canPickUp.test(AmmoBoxType.PISTOL)).thenReturn(false); // already full
        final BiConsumer<Float, AmmoBoxType> pickUp = mock(BiConsumer.class);

        final AmmoBoxItem item = new AmmoBoxItem(AmmoBoxType.PISTOL, canPickUp, pickUp);

        assertFalse(item.canInteract(Set.of()));
    }

    @SuppressWarnings("unchecked")
    @Test
    void universalBoxIsLargerAndRefillsHalf() {
        final Predicate<AmmoBoxType> canPickUp = mock(Predicate.class);
        when(canPickUp.test(AmmoBoxType.UNIVERSAL)).thenReturn(true);
        final BiConsumer<Float, AmmoBoxType> pickUp = mock(BiConsumer.class);

        final AmmoBoxItem universal = new AmmoBoxItem(AmmoBoxType.UNIVERSAL, canPickUp, pickUp);
        final AmmoBoxItem pistol = new AmmoBoxItem(AmmoBoxType.PISTOL, t -> true, (a, b) -> {});

        // UNIVERSAL uses the 32*1.5 footprint, PISTOL the 24*1.5 footprint.
        assertEquals(48, universal.getWidth());
        assertEquals(48, universal.getHeight());
        assertEquals(36, pistol.getWidth());
        assertEquals(36, pistol.getHeight());

        universal.interact(Set.of());
        verify(pickUp).accept(eq(0.5f), eq(AmmoBoxType.UNIVERSAL));
    }

    @Test
    void despawnableFlagControlsCanRemove() {
        final AmmoBoxItem item = new AmmoBoxItem(AmmoBoxType.PISTOL, t -> true, (a, b) -> {});

        item.setDespawnable(false); // despawn time pushed to MAX -> never despawns
        assertFalse(item.canRemove());
        assertFalse(item.isPickedUp());
    }
}
