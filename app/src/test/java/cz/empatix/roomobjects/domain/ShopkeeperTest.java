package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.verifyNoInteractions;

/**
 * Headless coverage for the real {@link Shopkeeper}: the idle/reverse animation flip in
 * {@link Shopkeeper#update(double)} and the "say box" visibility window. A freshly built shopkeeper's
 * last text change is three seconds in the past, so {@code collectSubRenderables} takes its early
 * return and never reaches the font drawables (which are unavailable headlessly).
 */
class ShopkeeperTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    @Test
    void freshShopkeeperHidesSayBox() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Shopkeeper shopkeeper = new Shopkeeper();
            final RenderableState state = mock(RenderableState.class);

            shopkeeper.collectSubRenderables(state); // stale -> early return, no font access

            verifyNoInteractions(state);
        }
    }

    @Test
    void updateRunsAnimationFlipWithoutThrowing() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Shopkeeper shopkeeper = new Shopkeeper();
            shopkeeper.setPosition(120f, 130f);

            for (int i = 0; i < 30; i++) {
                shopkeeper.update(0.1);
            }

            assertFalse(shopkeeper.canRemove());
            assertEquals(120f, shopkeeper.getPosition().x, 0.0001f);
            assertEquals(130f, shopkeeper.getPosition().y, 0.0001f);
        }
    }

    @Test
    void geometry() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Shopkeeper shopkeeper = new Shopkeeper();

            assertEquals(80 * 4, shopkeeper.getWidth());
            assertEquals(90 * 4, shopkeeper.getHeight());
        }
    }
}
