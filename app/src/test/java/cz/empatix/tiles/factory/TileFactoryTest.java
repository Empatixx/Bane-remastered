package cz.empatix.tiles.factory;

import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

/**
 * Headless coverage for {@link TileFactory}: grid dimensioning, the out-of-bounds-as-collision
 * rule, and the floor-vs-wall classification driving whether a {@link TileImpl} carries a physics
 * body. Tile-number selection inside a branch is random, so we assert the structural invariants
 * (dimensions, collision presence and world position) rather than the exact atlas index.
 */
@ExtendWith(HeadlessAssets.class)
class TileFactoryTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    @Test
    void createTileMapKeepsTheInputGridDimensions() {
        final boolean[][] map = new boolean[4][5];

        final Tile[][] tiles = new TileFactory().createTileMap(map, 0, 0);

        assertEquals(4, tiles.length);
        assertEquals(5, tiles[0].length);
    }

    @Test
    void createTilemapFromClampShrinksByTheOneCellClampBorder() {
        // A 6x6 clamped map yields a 4x4 tile grid (border ring stripped).
        final boolean[][] clamped = new boolean[6][6];

        final Tile[][] tiles = new TileFactory().createTilemapFromClamp(clamped, 0, 0);

        assertEquals(4, tiles.length);
        assertEquals(4, tiles[0].length);
    }

    @Test
    void allFalseMapProducesFloorTilesWithoutCollisionBodies() {
        final boolean[][] map = new boolean[3][3]; // every cell open -> all floor

        final Tile[][] tiles = new TileFactory().createTileMap(map, 0, 0);

        for (final Tile[] row : tiles) {
            for (final Tile tile : row) {
                assertNull(((TileImpl) tile).collision(), "floor tiles must be passable");
            }
        }
    }

    @Test
    void interiorWallSurroundedByWallsBecomesACollisionTile() {
        // 3x3 fully-walled block: the centre cell sees collisions on every side/corner.
        final boolean[][] map = {
                {true, true, true},
                {true, true, true},
                {true, true, true}
        };

        final Tile[][] tiles = new TileFactory().createTileMap(map, 0, 0);

        assertNotNull(((TileImpl) tiles[1][1]).collision(),
                "an enclosed wall tile must be solid");
    }

    @Test
    void tileWorldPositionAccountsForOffsetAndGridIndex() {
        final boolean[][] map = new boolean[2][2];

        final Tile[][] tiles = new TileFactory().createTileMap(map, 64, 128);

        final TileImpl corner = (TileImpl) tiles[1][1]; // grid x=1, y=1
        assertEquals(64 + SCALED, corner.position().x, 0.0001f);
        assertEquals(128 + SCALED, corner.position().y, 0.0001f);
    }
}
