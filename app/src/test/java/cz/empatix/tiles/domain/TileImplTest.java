package cz.empatix.tiles.domain;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;

/**
 * Headless coverage for the real {@link TileImpl} factory math: world-space position from
 * tile/offset, the row-driven collision rule (tiles on row {@code > 0} of the atlas get a physics
 * body, floor tiles do not) and the constant-derived render metadata.
 */
@ExtendWith(HeadlessAssets.class)
class TileImplTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    @Test
    void positionIsOffsetPlusTileTimesScaledTileSize() {
        final TextureAtlas.Entry texture = MockedAssets.entry();

        // tileNumber 5 is on row 0 (5 / 21 == 0): a floor tile without collision.
        final TileImpl tile = TileImpl.from(5, 2, 3, 100, 200, texture);

        assertEquals(100 + 2 * SCALED, tile.position().x, 0.0001f);
        assertEquals(200 + 3 * SCALED, tile.position().y, 0.0001f);
    }

    @Test
    void floorTileFromTopAtlasRowHasNoCollisionBody() {
        // tileNumber 0..20 are atlas row 0 -> no collision.
        final TileImpl tile = TileImpl.from(6, 0, 0, 0, 0, MockedAssets.entry());

        assertNull(tile.collision());
    }

    @Test
    void wallTileFromSecondAtlasRowHasCollisionBodyWiredBackToTile() {
        // tileNumber 41 -> row 41/21 == 1 -> wall with a collision body.
        final TileImpl tile = TileImpl.from(41, 1, 1, 0, 0, MockedAssets.entry());

        assertNotNull(tile.collision());
        // The compact constructor sets the body user-data back to the owning tile.
        assertSame(tile, tile.collision().getUserData());
    }

    @Test
    void textureIdAndZIndexComeFromTheAtlasEntryAndConstants() {
        final TextureAtlas.Entry texture = MockedAssets.entry();

        final TileImpl tile = TileImpl.from(6, 0, 0, 0, 0, texture);

        assertEquals(texture.textureId(), tile.textureId());
        assertEquals(1, tile.getZIndex());
        assertEquals(2, TileImpl.SCALE);
        assertEquals(32, TileImpl.TILE_SIZE);
        assertEquals(SCALED, TileImpl.SCALE_TILE_SIZE);
    }

    @Test
    void destroyOnFloorTileIsANoOpAndDoesNotThrow() {
        final TileImpl tile = TileImpl.from(6, 0, 0, 0, 0, MockedAssets.entry());

        tile.destroy(); // no collision body -> simply returns
        assertNull(tile.collision());
    }
}
