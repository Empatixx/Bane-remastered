package cz.empatix.rooms.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link LobbyRoom}: the fixed 29x10 lobby geometry, its UNKNOWN
 * room type and the documented refusal to participate in neighbour wiring.
 */
class LobbyRoomTest {

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
    void lobbyHasFixedGeometryAndZeroOrigin() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final LobbyRoom room = new LobbyRoom();

            assertEquals(29 * SCALED, room.getWidth());
            assertEquals(10 * SCALED, room.getHeight());
            assertEquals(0, room.getX());
            assertEquals(0, room.getY());
            assertEquals(RoomType.UNKNOWN, room.getRoomType());
            assertTrue(room.getRoomObjects().isEmpty());
        }
    }

    @Test
    void lobbyRejectsNeighbourWiring() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final LobbyRoom room = new LobbyRoom();

            assertThrows(UnsupportedOperationException.class,
                    () -> room.addNeighbour(RoomDirection.TOP, mock(Room.class)));
            assertThrows(UnsupportedOperationException.class, room::getNeighboursMap);
        }
    }
}
