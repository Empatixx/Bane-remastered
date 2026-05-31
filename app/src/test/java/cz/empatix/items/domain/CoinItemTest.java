package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.Set;
import java.util.function.Consumer;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verify;

/** Drives the real {@link CoinItem} pickup logic headlessly. */
class CoinItemTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @SuppressWarnings("unchecked")
    @Test
    void pickupAddsOneCoinAndIsRemovable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
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
}
