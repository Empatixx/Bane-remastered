package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.testsupport.HeadlessAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Headless coverage for the real {@link LobbyRoom}: the fixed 29x10 lobby geometry, its UNKNOWN
 * room type and the documented refusal to participate in neighbour wiring.
 */
@ExtendWith(HeadlessAssets.class)
class LobbyRoomTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    @Test
    void lobbyHasFixedGeometryAndZeroOrigin() {
        final LobbyRoom room = new LobbyRoom();

        assertEquals(29 * SCALED, room.getWidth());
        assertEquals(10 * SCALED, room.getHeight());
        assertEquals(0, room.getX());
        assertEquals(0, room.getY());
        assertEquals(RoomType.UNKNOWN, room.getRoomType());
        assertTrue(room.getRoomObjects().isEmpty());
    }

    @Test
    void lobbyRejectsNeighbourWiring() {
        final LobbyRoom room = new LobbyRoom();

        assertThrows(UnsupportedOperationException.class,
                () -> room.addNeighbour(RoomDirection.TOP, mock(Room.class)));
        assertThrows(UnsupportedOperationException.class, room::getNeighboursMap);
    }
}
