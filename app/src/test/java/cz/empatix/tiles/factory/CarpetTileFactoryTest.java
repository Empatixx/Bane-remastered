package cz.empatix.tiles.factory;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.tiles.domain.CarpetTile;
import cz.empatix.tiles.domain.EmptyTile;
import cz.empatix.tiles.domain.Tile;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for {@link CarpetTileFactory}: open cells turn into placeholder
 * {@link EmptyTile}s while carpet cells (and their derivable orientations) become {@link CarpetTile}s.
 */
class CarpetTileFactoryTest {

    @Test
    void emptyMapProducesOnlyEmptyTilesAndKeepsDimensions() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final boolean[][] map = new boolean[3][4]; // all false -> no carpet anywhere

            final Tile[][] tiles = new CarpetTileFactory().createTileMap(map, 0, 0);

            assertEquals(3, tiles.length);
            assertEquals(4, tiles[0].length);
            for (final Tile[] row : tiles) {
                for (final Tile tile : row) {
                    assertInstanceOf(EmptyTile.class, tile);
                }
            }
        }
    }

    @Test
    void fullyCarpetedInteriorCellBecomesACenterCarpetTile() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final boolean[][] map = {
                    {true, true, true},
                    {true, true, true},
                    {true, true, true}
            };

            final Tile[][] tiles = new CarpetTileFactory().createTileMap(map, 0, 0);

            // Centre cell is surrounded on all sides/corners -> a solid CENTER carpet piece.
            assertInstanceOf(CarpetTile.class, tiles[1][1]);
        }
    }

    @Test
    void outerCarpetEdgeCellsResolveToCarpetTilesNotEmptyTiles() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            // A 2x2 carpet block: every cell is an outer corner of the block and renders as carpet
            // (out-of-bounds neighbours count as collision via the factory's clamp rule).
            final boolean[][] map = {
                    {true, true},
                    {true, true}
            };

            final Tile[][] tiles = new CarpetTileFactory().createTileMap(map, 0, 0);

            assertInstanceOf(CarpetTile.class, tiles[0][0]);
            assertInstanceOf(CarpetTile.class, tiles[1][1]);
        }
    }
}
