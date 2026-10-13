package cz.empatix.map.domain;

import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Tests the pure geometry/state behaviour of the {@link RoomGenInfo} hierarchy.
 *
 * <p>All concrete subclasses share the same dimensions (rows=15, cols=20) after
 * {@link RoomGenInfo#initDimensions()}, so a {@link StartRoomGenInfo} (with its
 * no-arg constructor) is used as the canonical fixture for the shared base-class
 * methods. Subclass-specific behaviour ({@code getRoomType}, {@code canCreateNeighbours})
 * is exercised separately.
 *
 * <p>NOT exercised here: {@code constructRoom()} on any subclass — it instantiates
 * Room/TileMap implementations which pull in OpenGL/asset state, which is out of
 * scope for headless tests. {@link RoomGenInfo#setOrientationAround(RoomGenInfo)}
 * is exercised only for non-throwing/geometry-consistency since it relies on
 * {@code ThreadLocalRandom} and is therefore non-deterministic in its placement.
 */
class RoomGenInfoTest {

    private static final int TILE = TileImpl.TILE_SIZE * TileImpl.SCALE; // 64
    // Default subclass dimensions: rows = 15, cols = 20.
    private static final int EXPECTED_WIDTH = 20 * TILE;  // 1280
    private static final int EXPECTED_HEIGHT = 15 * TILE; // 960

    private RoomGenInfo newRoom() {
        final RoomGenInfo room = new StartRoomGenInfo();
        room.initDimensions();
        return room;
    }

    @Test
    void initDimensionsProducesExpectedWidthAndHeight() {
        final RoomGenInfo room = new StartRoomGenInfo();
        // before initDimensions the room has zero extent
        assertEquals(0, room.getWidth());
        assertEquals(0, room.getHeight());

        room.initDimensions();

        assertEquals(EXPECTED_WIDTH, room.getWidth());
        assertEquals(EXPECTED_HEIGHT, room.getHeight());
    }

    @Test
    void setCoordinatesStoresOffsets() {
        final RoomGenInfo room = newRoom();

        room.setCoordinates(100, 250);

        assertEquals(100, room.getXOffset());
        assertEquals(250, room.getYOffset());
    }

    @Test
    void centersAreOffsetPlusHalfExtent() {
        final RoomGenInfo room = newRoom();
        room.setCoordinates(100, 200);

        assertEquals(100 + EXPECTED_WIDTH / 2, room.getWidthCenter());
        assertEquals(200 + EXPECTED_HEIGHT / 2, room.getHeightCenter());
    }

    @Test
    void allDirectionsAvailableWhenNoNeighbours() {
        final RoomGenInfo room = newRoom();

        final List<RoomDirection> available = room.getAvailableDirections();

        assertEquals(RoomDirection.values().length, available.size());
        assertTrue(available.containsAll(List.of(RoomDirection.values())));
    }

    @Test
    void addNeighbourRemovesThatDirectionFromAvailable() {
        final RoomGenInfo room = newRoom();
        final RoomGenInfo other = newRoom();

        room.addNeighbour(other, RoomDirection.TOP);

        final List<RoomDirection> available = room.getAvailableDirections();
        assertFalse(available.contains(RoomDirection.TOP));
        assertEquals(RoomDirection.values().length - 1, available.size());
        assertSame(other, room.getNeighbours().get(RoomDirection.TOP));
    }

    @Test
    void addingAllFourNeighboursLeavesNoneAvailable() {
        final RoomGenInfo room = newRoom();
        for (final RoomDirection dir : RoomDirection.values()) {
            room.addNeighbour(newRoom(), dir);
        }

        assertTrue(room.getAvailableDirections().isEmpty());
        assertEquals(RoomDirection.values().length, room.getNeighbours().size());
    }

    @Test
    void addNeighbourOnSameDirectionOverwrites() {
        final RoomGenInfo room = newRoom();
        final RoomGenInfo first = newRoom();
        final RoomGenInfo second = newRoom();

        room.addNeighbour(first, RoomDirection.LEFT);
        room.addNeighbour(second, RoomDirection.LEFT);

        assertEquals(1, room.getNeighbours().size());
        assertSame(second, room.getNeighbours().get(RoomDirection.LEFT));
    }

    @Test
    void intersectsReturnsTrueForOverlappingRooms() {
        final RoomGenInfo a = newRoom();
        final RoomGenInfo b = newRoom();
        a.setCoordinates(0, 0);
        b.setCoordinates(EXPECTED_WIDTH / 2, EXPECTED_HEIGHT / 2); // overlaps a's bottom-right quadrant

        assertTrue(a.intersects(b));
        // intersection is symmetric
        assertTrue(b.intersects(a));
    }

    @Test
    void intersectsReturnsTrueForIdenticalPlacement() {
        final RoomGenInfo a = newRoom();
        final RoomGenInfo b = newRoom();
        a.setCoordinates(500, 500);
        b.setCoordinates(500, 500);

        assertTrue(a.intersects(b));
    }

    @Test
    void intersectsReturnsFalseWhenApartHorizontally() {
        final RoomGenInfo a = newRoom();
        final RoomGenInfo b = newRoom();
        a.setCoordinates(0, 0);
        b.setCoordinates(EXPECTED_WIDTH + 10, 0); // fully to the right, no overlap

        assertFalse(a.intersects(b));
        assertFalse(b.intersects(a));
    }

    @Test
    void intersectsReturnsFalseWhenApartVertically() {
        final RoomGenInfo a = newRoom();
        final RoomGenInfo b = newRoom();
        a.setCoordinates(0, 0);
        b.setCoordinates(0, EXPECTED_HEIGHT + 10);

        assertFalse(a.intersects(b));
    }

    @Test
    void intersectsReturnsFalseWhenEdgesTouchExactly() {
        final RoomGenInfo a = newRoom();
        final RoomGenInfo b = newRoom();
        a.setCoordinates(0, 0);
        // b's left edge is exactly a's right edge; the comparison is strict (<, >)
        b.setCoordinates(EXPECTED_WIDTH, 0);

        assertFalse(a.intersects(b));
    }

    @Test
    void intersectsWithNullIsFalse() {
        final RoomGenInfo a = newRoom();

        assertFalse(a.intersects(null));
    }

    @Test
    void setOrientationAroundDoesNotThrowAndKeepsDimensions() {
        final RoomGenInfo parent = newRoom();
        parent.setCoordinates(1000, 1000);
        final RoomGenInfo child = newRoom();

        // Placement direction is chosen via ThreadLocalRandom, so we only assert
        // the call succeeds and leaves the child's extent unchanged. Run enough
        // iterations to cover all four random branches with high probability.
        for (int i = 0; i < 200; i++) {
            child.setOrientationAround(parent);
            assertEquals(EXPECTED_WIDTH, child.getWidth());
            assertEquals(EXPECTED_HEIGHT, child.getHeight());
        }
    }

    @Test
    void roomTypesMatchSubclass() {
        // Subclasses requiring service dependencies only store them in their
        // constructors; getRoomType() never touches them, so null is safe here.
        assertEquals(RoomType.START, new StartRoomGenInfo().getRoomType());
        assertEquals(RoomType.NORMAL, new CommonRoomGenInfo(null, null, null, null).getRoomType());
        assertEquals(RoomType.BOSS, new BossRoomGenInfo(null, null, null, null, null, null, null).getRoomType());
        assertEquals(RoomType.SHOP, new ShopRoomGenInfo(null, null, null, null).getRoomType());
        assertEquals(RoomType.SACRIFICE, new SacrificeRoomGenInfo(null, null, null).getRoomType());
        assertEquals(RoomType.TREASURE, new TreasureRoomGenInfo(null).getRoomType());
        assertEquals(RoomType.PATH, new PathRoomGenInfo().getRoomType());
    }

    @Test
    void canCreateNeighboursBySubclass() {
        // START and NORMAL allow neighbours while under the max; the rest never do.
        final RoomGenInfo start = new StartRoomGenInfo();
        final RoomGenInfo common = new CommonRoomGenInfo(null, null, null, null);
        assertTrue(start.canCreateNeighbours());
        assertTrue(common.canCreateNeighbours());

        assertFalse(new BossRoomGenInfo(null, null, null, null, null, null, null).canCreateNeighbours());
        assertFalse(new ShopRoomGenInfo(null, null, null, null).canCreateNeighbours());
        assertFalse(new SacrificeRoomGenInfo(null, null, null).canCreateNeighbours());
        assertFalse(new TreasureRoomGenInfo(null).canCreateNeighbours());
        assertFalse(new PathRoomGenInfo().canCreateNeighbours());
    }

    @Test
    void startRoomStopsAllowingNeighboursAtMax() {
        final RoomGenInfo start = new StartRoomGenInfo();
        for (final RoomDirection dir : RoomDirection.values()) {
            assertTrue(start.canCreateNeighbours());
            start.addNeighbour(new StartRoomGenInfo(), dir);
        }
        // All four directions now occupied -> at the MAX_ROOM_NEIGHBOURS limit.
        assertFalse(start.canCreateNeighbours());
    }

    @Test
    void pathRoomGenInfoUsesMinimalDimensions() {
        final RoomGenInfo path = new PathRoomGenInfo();
        path.initDimensions();

        assertEquals(PathRoomGenInfo.MIN_COLS * TILE, path.getWidth());
        assertEquals(PathRoomGenInfo.MIN_ROWS * TILE, path.getHeight());
    }
}
