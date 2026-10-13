package cz.empatix.tiles.domain;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;

/** Headless coverage for {@link CarpetTile} factory math and constant-derived render metadata. */
@ExtendWith(HeadlessAssets.class)
class CarpetTileTest {

    private static final int SCALED = 32 * 2;

    @Test
    void positionIsOffsetPlusTileTimesScaledTileSize() {
        final TextureAtlas.Entry texture = MockedAssets.entry();

        final CarpetTile tile = CarpetTile.from(5, 4, 6, 10, 20, texture);

        assertEquals(10 + 4 * SCALED, tile.position().x, 0.0001f);
        assertEquals(20 + 6 * SCALED, tile.position().y, 0.0001f);
        assertEquals(texture.textureId(), tile.textureId());
    }

    @Test
    void renderMetadataUsesSceneLayerOne() {
        final CarpetTile tile = CarpetTile.from(0, 0, 0, 0, 0, MockedAssets.entry());

        assertEquals(RenderScene.SCENE, tile.renderScene());
        assertEquals(1, tile.getZIndex());
    }

    @Test
    void destroyIsANoOp() {
        final CarpetTile tile = CarpetTile.from(0, 0, 0, 0, 0, MockedAssets.entry());

        tile.destroy(); // must not throw
    }
}
