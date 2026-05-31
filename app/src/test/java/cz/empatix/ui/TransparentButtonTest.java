package cz.empatix.ui;

import cz.empatix.audio.init.ALInit;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.MockedAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mockStatic;

/**
 * Exercises the REAL {@link TransparentButton}. Its hit-box is derived from text metrics; with the
 * GL-free fake font those metrics are zero, leaving the constant 10px bonus offset, so a button at
 * (100,100) accepts hits strictly inside (90,110) on both axes.
 */
class TransparentButtonTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @Test
    void mousePressedInsideRunsTheClickHandler() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);
            final AtomicInteger clicks = new AtomicInteger();
            button.onClick(clicks::incrementAndGet);

            button.mousePressed(0, 100, 100);

            assertEquals(1, clicks.get(), "a press inside the bonus-offset box fires onClick");
        }
    }

    @Test
    void mousePressedOutsideDoesNotRunTheClickHandler() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);
            final AtomicInteger clicks = new AtomicInteger();
            button.onClick(clicks::incrementAndGet);

            button.mousePressed(0, 100, 200);

            assertEquals(0, clicks.get(), "a press outside the box does not fire onClick");
        }
    }

    @Test
    void mousePressedWithNoHandlerIsANoOp() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

            button.mousePressed(0, 100, 100); // no onClick — must not throw

            assertFalse(button.isHovered(), "no hover is computed until update() runs");
        }
    }

    @Test
    void updateTogglesHoverFromMousePosition() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

            button.update(0.016, 100, 100);
            assertTrue(button.isHovered(), "mouse on the button hovers it");

            button.update(0.016, 100, 200);
            assertFalse(button.isHovered(), "mouse off the button clears hover");
        }
    }

    @Test
    void rendersOnOverlaysSceneAndProducesNoRenderData() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TransparentButton button = new TransparentButton("Back", new Vector2f(100, 100), 3);

            assertEquals(RenderScene.OVERLAYS, button.renderScene(), "transparent button lives on OVERLAYS");
            assertFalse(button.canRender(), "it draws only its text sub-renderable, not itself");
            assertNull(button.renderData(), "renderData is null for a transparent button");
            assertEquals(3, button.getZIndex(), "z-depth comes from the constructor");
        }
    }
}
