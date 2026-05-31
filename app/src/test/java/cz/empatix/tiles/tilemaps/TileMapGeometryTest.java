package cz.empatix.tiles.tilemaps;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.EnumSet;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless geometry coverage for the real {@link TileMap} implementations: width/height derive from
 * cols/rows times the scaled tile size, the offset getters echo construction args, and the default
 * {@code getTotalRows/Columns} round-trip cleanly. {@code toRender}/{@code cleanUp} are exercised to
 * cover the stream plumbing without asserting GL output.
 */
class TileMapGeometryTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    @Test
    void proceduralTileMapDimensionsAndOffsetsDeriveFromConstructorArgs() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ProceduralTileMap map = new ProceduralTileMap(8, 10, 32, 64,
                    EnumSet.noneOf(RoomDirection.class));

            assertEquals(10 * SCALED, map.getWidth());
            assertEquals(8 * SCALED, map.getHeight());
            assertEquals(32, map.getX());
            assertEquals(64, map.getY());
            assertEquals(10, map.getTotalCollumns());
            assertEquals(8, map.getTotalRows());

            map.cleanUp();
        }
    }

    @Test
    void commonTileMapDimensionsAndRenderListAreConsistent() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final CommonTileMap map = new CommonTileMap(6, 7, 0, 0,
                    EnumSet.noneOf(RoomDirection.class));

            assertEquals(7 * SCALED, map.getWidth());
            assertEquals(6 * SCALED, map.getHeight());
            // createTilemapFromClamp strips the clamp border: a 6x7 room yields 6x7 tiles.
            assertEquals(6 * 7, map.toRender().size());

            map.cleanUp();
        }
    }

    @Test
    void lobbyTileMapHasFixedDimensionsAndZeroOffset() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final LobbyTileMap map = new LobbyTileMap();

            assertEquals(29 * SCALED, map.getWidth());
            assertEquals(10 * SCALED, map.getHeight());
            assertEquals(0, map.getX());
            assertEquals(0, map.getY());
            assertEquals(29 * 10, map.toRender().size());

            map.cleanUp();
        }
    }

    @Test
    void pathTileMapHorizontalDimensionsFollowRowsAndCols() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final PathTileMap map = new PathTileMap(16, 48, 4, 6, true);

            assertEquals(6 * SCALED, map.getWidth());
            assertEquals(4 * SCALED, map.getHeight());
            assertEquals(16, map.getX());
            assertEquals(48, map.getY());
            // 4x6 clamp -> 4x6 tiles.
            assertEquals(4 * 6, map.toRender().size());

            map.cleanUp();
        }
    }

    @Test
    void treasureTileMapDimensionsFollowRowsAndCols() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final TreasureTileMap map = new TreasureTileMap(13, 17, 10, 20,
                    EnumSet.of(RoomDirection.LEFT));

            assertEquals(17 * SCALED, map.getWidth());
            assertEquals(13 * SCALED, map.getHeight());
            assertEquals(10, map.getX());
            assertEquals(20, map.getY());

            map.cleanUp();
        }
    }

    @Test
    void sacrificialTileMapDimensionsFollowRowsAndCols() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SacrificialTileMap map = new SacrificialTileMap(13, 19, 0, 0,
                    EnumSet.of(RoomDirection.TOP));

            assertEquals(19 * SCALED, map.getWidth());
            assertEquals(13 * SCALED, map.getHeight());

            map.cleanUp();
        }
    }

    @Test
    void carpetTileMapsDimensionsFollowRowsAndColsAndRenderEveryCell() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ShopCarpetTileMap shop = new ShopCarpetTileMap(12, 10, 0, 0,
                    EnumSet.noneOf(RoomDirection.class));
            final TreasureCarpetTileMap treasure = new TreasureCarpetTileMap(16, 18, 5, 5,
                    EnumSet.noneOf(RoomDirection.class));
            final ProceduralCarpetTileMap procedural = new ProceduralCarpetTileMap(9, 11, 0, 0,
                    EnumSet.noneOf(RoomDirection.class));

            assertEquals(10 * SCALED, shop.getWidth());
            assertEquals(12 * SCALED, shop.getHeight());
            // CarpetTileFactory keeps the full grid (no clamp border stripped).
            assertEquals(12 * 10, shop.toRender().size());

            assertEquals(18 * SCALED, treasure.getWidth());
            assertEquals(16 * SCALED, treasure.getHeight());
            assertEquals(16 * 18, treasure.toRender().size());

            assertEquals(11 * SCALED, procedural.getWidth());
            assertEquals(9 * SCALED, procedural.getHeight());
            assertEquals(9 * 11, procedural.toRender().size());

            shop.cleanUp();
            treasure.cleanUp();
            procedural.cleanUp();
        }
    }

    @Test
    void isValidWallRejectsAWallSurroundedByEmptyTiles() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ProceduralTileMap map = new ProceduralTileMap(5, 5, 0, 0,
                    EnumSet.noneOf(RoomDirection.class));

            final boolean[][] grid = new boolean[3][3];
            grid[1][1] = true; // a lone wall with no vertical neighbours

            // No top and no bottom neighbour -> not a valid wall.
            assertFalse(map.isValidWall(grid, 1, 1));

            map.cleanUp();
        }
    }
}
