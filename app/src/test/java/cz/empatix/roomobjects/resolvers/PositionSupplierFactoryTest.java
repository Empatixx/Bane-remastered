package cz.empatix.roomobjects.resolvers;

import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.junit.jupiter.api.Test;

import java.util.function.Supplier;

import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Pure-logic coverage for {@link PositionSupplierFactory}. The suppliers draw random points; we
 * assert the points always fall inside the room/offset bounds (sampling many draws) rather than any
 * exact value. {@link Room} and {@link TileMap} are mocks supplying only the geometry the factory
 * reads — no physics or assets involved.
 */
class PositionSupplierFactoryTest {

    private static final int SAMPLES = 200;

    // The Room geometry accessors used by the factory are interface default methods; a plain Mockito
    // mock does not run their bodies, so we stub the min/max corners directly.
    private static Room room(final int x, final int y, final int width, final int height) {
        final Room room = mock(Room.class);
        when(room.getMinX()).thenReturn(x);
        when(room.getMinY()).thenReturn(y);
        when(room.getMaxX()).thenReturn(x + width);
        when(room.getMaxY()).thenReturn(y + height);
        return room;
    }

    @Test
    void positionSupplierStaysWithinTheOffsetRoomBounds() {
        final Room room = room(100, 200, 1000, 800);
        final Vector2i offset = new Vector2i(50, 40);
        final int xMin = 100 + 50, xMax = 100 + 1000 - 50;
        final int yMin = 200 + 40, yMax = 200 + 800 - 40;

        final Supplier<Vector2f> supplier = PositionSupplierFactory.positionSupplier(room, offset);

        for (int i = 0; i < SAMPLES; i++) {
            final Vector2f p = supplier.get();
            assertTrue(p.x >= xMin && p.x < xMax, "x within [" + xMin + "," + xMax + ") but was " + p.x);
            assertTrue(p.y >= yMin && p.y < yMax, "y within [" + yMin + "," + yMax + ") but was " + p.y);
        }
    }

    @Test
    void positionSupplierProducesFiniteValues() {
        final Supplier<Vector2f> supplier =
                PositionSupplierFactory.positionSupplier(room(0, 0, 500, 500), new Vector2i(10, 10));

        for (int i = 0; i < SAMPLES; i++) {
            final Vector2f p = supplier.get();
            assertTrue(Float.isFinite(p.x) && Float.isFinite(p.y));
        }
    }

    @Test
    void tileBasedSupplierSnapsInsideTheTileGrid() {
        final Room room = room(64, 128, 0, 0); // only min corner is read by the tile supplier
        final TileMap tileMap = mock(TileMap.class);
        // getTotalCollumns/Rows are interface defaults; stub them directly so the mock returns them.
        when(tileMap.getTotalCollumns()).thenReturn(20);
        when(tileMap.getTotalRows()).thenReturn(15);
        final Vector2i offset = new Vector2i(2, 3);

        final Supplier<Vector2f> supplier =
                PositionSupplierFactory.tileBasedPositionSupplier(room, tileMap, offset);

        final int tile = TileImpl.SCALE_TILE_SIZE;
        final int xMin = 64 + offset.x() * tile;
        final int xMax = 64 + (20 - offset.x()) * tile;
        final int yMin = 128 + offset.y() * tile;
        final int yMax = 128 + (15 - offset.y()) * tile;

        for (int i = 0; i < SAMPLES; i++) {
            final Vector2f p = supplier.get();
            assertTrue(p.x >= xMin && p.x < xMax, "x within tile bounds but was " + p.x);
            assertTrue(p.y >= yMin && p.y < yMax, "y within tile bounds but was " + p.y);
            // positions are tile-aligned relative to the room origin
            assertTrue((p.x - 64) % tile == 0f);
            assertTrue((p.y - 128) % tile == 0f);
        }
    }
}
