package cz.empatix.items.domain;

import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.lwjgl.glfw.GLFW;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.Mockito.atLeastOnce;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/** Drives the real {@link ShopItem} buy logic over a mocked inner drop. */
@ExtendWith(HeadlessAssets.class)
class ShopItemTest {

    private static ItemDrop innerDrop() {
        final ItemDrop drop = mock(ItemDrop.class);
        when(drop.getPosition()).thenReturn(new Vector2f());
        when(drop.getWidth()).thenReturn(16);
        when(drop.getHeight()).thenReturn(16);
        return drop;
    }

    @SuppressWarnings("unchecked")
    @Test
    void buyDeductsPriceWhenAffordableAndForwardsPickup() {
        final ItemDrop drop = innerDrop();
        when(drop.canInteract(Set.of(GLFW.GLFW_KEY_E))).thenReturn(true);
        final Predicate<Integer> hasEnough = mock(Predicate.class);
        when(hasEnough.test(anyInt())).thenReturn(true);
        final Consumer<Integer> removeCoins = mock(Consumer.class);

        final ShopItem item = new ShopItem(drop, hasEnough, removeCoins);
        item.setPrice(ShopType.AMMUNITION, 1); // deterministic-ish; price >= 1
        final Runnable onBuy = mock(Runnable.class);
        item.setOnBuy(onBuy);

        item.interact(Set.of(GLFW.GLFW_KEY_E));

        verify(removeCoins, atLeastOnce()).accept(anyInt());
        verify(onBuy).run();
        // After buying, the inner drop's pickup is forwarded.
        verify(drop).interact(Set.of(GLFW.GLFW_KEY_E));
    }

    @SuppressWarnings("unchecked")
    @Test
    void cannotBuyWithoutEnoughCoins() {
        final ItemDrop drop = innerDrop();
        final Predicate<Integer> hasEnough = mock(Predicate.class);
        when(hasEnough.test(anyInt())).thenReturn(false);
        final Consumer<Integer> removeCoins = mock(Consumer.class);
        final Runnable onBuy = mock(Runnable.class);

        final ShopItem item = new ShopItem(drop, hasEnough, removeCoins);
        item.setPrice(ShopType.OFFENSIVE, 0);
        item.setOnBuy(onBuy);

        item.interact(Set.of(GLFW.GLFW_KEY_E));

        verify(removeCoins, never()).accept(anyInt());
        verify(onBuy, never()).run();
        verify(drop, never()).interact(Set.of(GLFW.GLFW_KEY_E));
    }

    @SuppressWarnings("unchecked")
    @Test
    void nonEKeyDoesNotBuy() {
        final ItemDrop drop = innerDrop();
        final Predicate<Integer> hasEnough = mock(Predicate.class);
        when(hasEnough.test(anyInt())).thenReturn(true);
        final Consumer<Integer> removeCoins = mock(Consumer.class);

        final ShopItem item = new ShopItem(drop, hasEnough, removeCoins);
        item.setPrice(ShopType.DEFENSIVE, 2);

        item.interact(Set.of(GLFW.GLFW_KEY_W));

        verify(removeCoins, never()).accept(anyInt());
    }

    @SuppressWarnings("unchecked")
    @Test
    void canRemoveOnlyAfterBought() {
        final ItemDrop drop = innerDrop();
        when(drop.canRemove()).thenReturn(true);
        when(drop.canInteract(Set.of(GLFW.GLFW_KEY_E))).thenReturn(true);
        final Predicate<Integer> hasEnough = mock(Predicate.class);
        when(hasEnough.test(anyInt())).thenReturn(true);

        final ShopItem item = new ShopItem(drop, hasEnough, c -> {});
        item.setPrice(ShopType.AMMUNITION, 0);

        // Inner drop can be removed, but until bought the shop wrapper holds it.
        assertFalse(item.canRemove());

        item.interact(Set.of(GLFW.GLFW_KEY_E));
        assertTrue(item.canRemove());
    }

    @Test
    void discountReducesPriceAndNeverBelowOne() {
        final ItemDrop drop = innerDrop();
        when(drop.canInteract(Set.of(GLFW.GLFW_KEY_E))).thenReturn(true);

        // Heavy discount floors the price at 1; the follow-up buy then removes exactly 1 coin.
        final java.util.concurrent.atomic.AtomicInteger removed = new java.util.concurrent.atomic.AtomicInteger();
        final ShopItem item = new ShopItem(drop, i -> true, removed::set);
        item.setPrice(ShopType.OFFENSIVE, 0); // some price in [1,4]*1*2
        item.discount(0.0);                   // price -> 0 -> clamped to 1
        item.interact(Set.of(GLFW.GLFW_KEY_E));

        org.junit.jupiter.api.Assertions.assertEquals(1, removed.get());
    }
}
