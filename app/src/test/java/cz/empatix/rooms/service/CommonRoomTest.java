package cz.empatix.rooms.service;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.enemies.service.DifficultyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.EnumSet;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless coverage for the real {@link CommonRoom}: geometry derived from the backing tile map,
 * the default OPEN state of a freshly-built (unlocked) room, room-type identity and neighbour
 * bookkeeping. The room is built with an empty neighbour set so the lock service spawns no gates
 * and no enemies are created.
 */
@ExtendWith(HeadlessAssets.class)
class CommonRoomTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    private static CommonRoom room(final int rows, final int cols, final int xOff, final int yOff) {
        return new CommonRoom(mock(ItemService.class), rows, cols, xOff, yOff,
                EnumSet.noneOf(RoomDirection.class), mock(StatisticsService.class), new DifficultyService(), new RngService(0L));
    }

    @Test
    void geometryDerivesFromRowsColsAndOffset() {
        final CommonRoom room = room(8, 11, 64, 128);

        assertEquals(11 * SCALED, room.getWidth());
        assertEquals(8 * SCALED, room.getHeight());
        assertEquals(64, room.getX());
        assertEquals(128, room.getY());
        assertEquals(64, room.getMinX());
        assertEquals(128, room.getMinY());
        assertEquals(64 + 11 * SCALED, room.getMaxX());
        assertEquals(128 + 8 * SCALED, room.getMaxY());
    }

    @Test
    void freshRoomIsOpenAndReportsNormalType() {
        final CommonRoom room = room(6, 6, 0, 0);

        assertEquals(RoomState.OPEN, room.getState());
        assertEquals(RoomType.NORMAL, room.getRoomType());
        assertTrue(room.getEnemies().isEmpty());
        assertTrue(room.getRoomObjects().isEmpty());
    }

    @Test
    void isInRoomTrueInsideBoundsFalseOutside() {
        final CommonRoom room = room(6, 6, 100, 100);

        // Centre point is inside.
        assertTrue(room.isInRoom(100 + 3 * SCALED, 100 + 3 * SCALED));
        // A point past maxX is outside.
        assertFalse(room.isInRoom(100 + 6 * SCALED + 1, 100 + 3 * SCALED));
        // A point before the origin is outside.
        assertFalse(room.isInRoom(99, 100));
    }

    @Test
    void addNeighbourIsStoredAndRetrievable() {
        final CommonRoom room = room(6, 6, 0, 0);
        final Room neighbour = mock(Room.class);

        room.addNeighbour(RoomDirection.LEFT, neighbour);

        assertEquals(1, room.getNeighboursMap().size());
        assertSame(neighbour, room.getNeighboursMap().get(RoomDirection.LEFT));
    }
}
