package cz.empatix.items.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.Set;
import java.util.function.Consumer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

/** Drives the real {@link CoinItem} pickup logic headlessly. */
@ExtendWith(HeadlessAssets.class)
class CoinItemTest {

    @SuppressWarnings("unchecked")
    @Test
    void pickupAddsOneCoinAndIsRemovable() {
        final Consumer<Integer> coinAdd = mock(Consumer.class);

        final CoinItem coin = new CoinItem(coinAdd);

        assertTrue(coin.canInteract(Set.of()));
        assertFalse(coin.isPickedUp());
        assertEquals(24, coin.getWidth()); // 16 * 1.5
        assertEquals(24, coin.getHeight());
        assertSame(RenderScene.SCENE, coin.renderScene());
        assertEquals(DropSource.NORMAL, coin.getSource());

        coin.interact(Set.of());

        assertTrue(coin.isPickedUp());
        assertTrue(coin.canRemove());
        assertFalse(coin.canInteract(Set.of())); // can't pick up twice
        verify(coinAdd).accept(eq(1));
    }
}
