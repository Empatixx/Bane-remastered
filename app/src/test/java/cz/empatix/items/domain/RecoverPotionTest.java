package cz.empatix.items.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Supplier;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Drives the real {@link RecoverPotion} pickup/heal logic headlessly. */
@ExtendWith(HeadlessAssets.class)
class RecoverPotionTest {

    @SuppressWarnings("unchecked")
    @Test
    void smallPotionAddsOneAndExposesTypeAndAmount() {
        final Supplier<Boolean> hasFull = mock(Supplier.class);
        when(hasFull.get()).thenReturn(false);
        final Consumer<Integer> add = mock(Consumer.class);

        final RecoverPotion potion = new RecoverPotion(RecoverSize.SMALL, RecoverType.HEALTH, hasFull, add);

        assertSame(RecoverType.HEALTH, potion.getType());
        assertEquals(1, potion.getAmount());
        assertTrue(potion.canInteract(Set.of()));

        potion.interact(Set.of());

        assertTrue(potion.isPickedUp());
        verify(add).accept(eq(1));
    }

    @SuppressWarnings("unchecked")
    @Test
    void largePotionAddsFourAndHasLargerFootprint() {
        final Consumer<Integer> add = mock(Consumer.class);

        final RecoverPotion large = new RecoverPotion(RecoverSize.LARGE, RecoverType.ARMOR, () -> false, add);
        final RecoverPotion small = new RecoverPotion(RecoverSize.SMALL, RecoverType.ARMOR, () -> false, i -> {});

        assertEquals(4, large.getAmount());
        assertSame(RecoverType.ARMOR, large.getType());
        // LARGE = 32*2 = 64; SMALL = 24*2.5 = 60
        assertEquals(64, large.getWidth());
        assertEquals(64, large.getHeight());
        assertEquals(60, small.getWidth());
        assertEquals(60, small.getHeight());

        large.interact(Set.of());
        verify(add).accept(eq(4));
    }

    @SuppressWarnings("unchecked")
    @Test
    void cannotInteractWhenAlreadyFull() {
        final Supplier<Boolean> hasFull = mock(Supplier.class);
        when(hasFull.get()).thenReturn(true); // already at max

        final RecoverPotion potion = new RecoverPotion(RecoverSize.SMALL, RecoverType.HEALTH, hasFull, i -> {});

        assertFalse(potion.canInteract(Set.of()));
        assertFalse(potion.isPickedUp());
    }
}
