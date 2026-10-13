package cz.empatix.rooms.service;

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
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless coverage for the real {@link TreasureRoom}: geometry from its backing tile map, the
 * TREASURE room type, the default OPEN state inherited from {@link Room} and neighbour bookkeeping.
 */
@ExtendWith(HeadlessAssets.class)
class TreasureRoomTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    private static TreasureRoom room(final int rows, final int cols, final int xOff, final int yOff) {
        return new TreasureRoom(rows, cols, xOff, yOff,
                EnumSet.noneOf(RoomDirection.class), mock(ItemService.class));
    }

    @Test
    void geometryDerivesFromRowsColsAndOffset() {
        final TreasureRoom room = room(14, 18, 32, 96);

        assertEquals(18 * SCALED, room.getWidth());
        assertEquals(14 * SCALED, room.getHeight());
        assertEquals(32, room.getX());
        assertEquals(96, room.getY());
        assertEquals(32 + 18 * SCALED, room.getMaxX());
        assertEquals(96 + 14 * SCALED, room.getMaxY());
    }

    @Test
    void typeStateAndEmptyObjectsBeforeInit() {
        final TreasureRoom room = room(13, 17, 0, 0);

        assertEquals(RoomType.TREASURE, room.getRoomType());
        assertEquals(RoomState.OPEN, room.getState());
        assertTrue(room.getRoomObjects().isEmpty());
    }

    @Test
    void addNeighbourIsStoredAndRetrievable() {
        final TreasureRoom room = room(13, 17, 0, 0);
        final Room neighbour = mock(Room.class);

        room.addNeighbour(RoomDirection.RIGHT, neighbour);

        assertSame(neighbour, room.getNeighboursMap().get(RoomDirection.RIGHT));
    }
}
