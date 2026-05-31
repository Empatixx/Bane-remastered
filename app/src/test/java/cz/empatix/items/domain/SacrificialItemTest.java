package cz.empatix.items.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.lwjgl.glfw.GLFW;
import org.mockito.ArgumentCaptor;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Drives the real {@link SacrificialItem} buy-with-health logic over a mocked inner drop. */
@ExtendWith(HeadlessAssets.class)
class SacrificialItemTest {

    private static ItemDrop innerDrop() {
        final ItemDrop drop = mock(ItemDrop.class);
        when(drop.getPosition()).thenReturn(new Vector2f());
        when(drop.getWidth()).thenReturn(16);
        when(drop.getHeight()).thenReturn(16);
        return drop;
    }

    @SuppressWarnings("unchecked")
    @Test
    void buyRemovesHealthEqualToRandomisedPriceWhenAffordable() {
        final ItemDrop drop = innerDrop();
        when(drop.canInteract(Set.of(GLFW.GLFW_KEY_E))).thenReturn(true);
        final Predicate<Integer> hasEnoughHealth = mock(Predicate.class);
        when(hasEnoughHealth.test(anyInt())).thenReturn(true);
        final Consumer<Integer> removeHealth = mock(Consumer.class);

        final SacrificialItem item = new SacrificialItem(drop, hasEnoughHealth, removeHealth);
        final Runnable onBuy = mock(Runnable.class);
        item.onBuy(onBuy);

        item.interact(Set.of(GLFW.GLFW_KEY_E));

        final ArgumentCaptor<Integer> price = ArgumentCaptor.forClass(Integer.class);
        verify(removeHealth).accept(price.capture());
        final int paid = price.getValue();
        assertTrue(paid >= 1 && paid <= 3, "health price within [1,3] but was " + paid);
        verify(onBuy).run();
        verify(drop).interact(Set.of(GLFW.GLFW_KEY_E));
    }

    @SuppressWarnings("unchecked")
    @Test
    void noPurchaseWhenNotEnoughHealth() {
        final ItemDrop drop = innerDrop();
        final Predicate<Integer> hasEnoughHealth = mock(Predicate.class);
        when(hasEnoughHealth.test(anyInt())).thenReturn(false);
        final Consumer<Integer> removeHealth = mock(Consumer.class);
        final Runnable onBuy = mock(Runnable.class);

        final SacrificialItem item = new SacrificialItem(drop, hasEnoughHealth, removeHealth);
        item.onBuy(onBuy);

        item.interact(Set.of(GLFW.GLFW_KEY_E));

        verify(removeHealth, never()).accept(anyInt());
        verify(onBuy, never()).run();
        verify(drop, never()).interact(Set.of(GLFW.GLFW_KEY_E));
    }

    @SuppressWarnings("unchecked")
    @Test
    void nonEKeyDoesNothing() {
        final ItemDrop drop = innerDrop();
        final Consumer<Integer> removeHealth = mock(Consumer.class);

        final SacrificialItem item = new SacrificialItem(drop, i -> true, removeHealth);

        item.interact(Set.of(GLFW.GLFW_KEY_W));

        verify(removeHealth, never()).accept(anyInt());
    }

    @Test
    void delegatesLifecycleToInnerDrop() {
        final ItemDrop drop = innerDrop();
        when(drop.canRemove()).thenReturn(true);
        when(drop.isPickedUp()).thenReturn(true);

        final SacrificialItem item = new SacrificialItem(drop, i -> true, c -> {});

        assertTrue(item.canRemove());
        assertTrue(item.isPickedUp());
        item.update(0.016f);
        verify(drop).update(0.016f);
    }
}
