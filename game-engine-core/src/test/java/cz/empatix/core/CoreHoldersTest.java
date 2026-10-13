package cz.empatix.core;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/** Covers the small shared core holders/constants. */
class CoreHoldersTest {

    @Test
    void viewportConfigureStoresTargetResolution() {
        Viewport.configure(1280, 720);
        assertEquals(1280, Viewport.TARGET_WIDTH);
        assertEquals(720, Viewport.TARGET_HEIGHT);

        // re-configurable
        Viewport.configure(960, 540);
        assertEquals(960, Viewport.TARGET_WIDTH);
        assertEquals(540, Viewport.TARGET_HEIGHT);
    }

    @Test
    void graphicsCapabilitiesHoldsMaxTextureSize() {
        GraphicsCapabilities.maxTextureSize = 8192;
        assertEquals(8192, GraphicsCapabilities.maxTextureSize);
    }

    @Test
    void engineTimestepIsSixtyHertz() {
        assertEquals(1.0 / 60.0, EngineTimestep.FIXED_DELTA_TIME, 1e-12);
    }

    @Test
    void assetPathsResolveUnderBase() {
        assertEquals(AssetPaths.BASE.resolve("textures"), AssetPaths.TEXTURES);
        assertEquals(AssetPaths.BASE.resolve("audio"), AssetPaths.AUDIO);
        assertEquals(AssetPaths.BASE.resolve("shaders"), AssetPaths.SHADERS);
        assertEquals(AssetPaths.BASE.resolve("fonts"), AssetPaths.FONTS);
        // all sit beneath the shared base directory
        assertTrue(AssetPaths.TEXTURES.startsWith(AssetPaths.BASE));
    }
}
