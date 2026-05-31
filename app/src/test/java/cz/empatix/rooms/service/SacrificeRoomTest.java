package cz.empatix.rooms.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.testsupport.MockedAssets;
import cz.empatix.tiles.domain.TileImpl;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.EnumSet;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;

/**
 * Headless coverage for the real {@link SacrificeRoom}: geometry from its backing tile map, the
 * SACRIFICE room type, the default OPEN state and neighbour bookkeeping.
 */
class SacrificeRoomTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new PhysicsService();
    }

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    private static SacrificeRoom room(final int rows, final int cols, final int xOff, final int yOff) {
        return new SacrificeRoom(mock(ItemService.class), rows, cols, xOff, yOff,
                EnumSet.noneOf(RoomDirection.class), mock(StatisticsService.class));
    }

    @Test
    void geometryDerivesFromRowsColsAndOffset() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SacrificeRoom room = room(13, 19, 16, 48);

            assertEquals(19 * SCALED, room.getWidth());
            assertEquals(13 * SCALED, room.getHeight());
            assertEquals(16, room.getX());
            assertEquals(48, room.getY());
        }
    }

    @Test
    void typeStateAndEmptyObjectsBeforeInit() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SacrificeRoom room = room(13, 19, 0, 0);

            assertEquals(RoomType.SACRIFICE, room.getRoomType());
            assertEquals(RoomState.OPEN, room.getState());
            assertTrue(room.getRoomObjects().isEmpty());
        }
    }

    @Test
    void addNeighbourIsStoredAndRetrievable() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SacrificeRoom room = room(13, 19, 0, 0);
            final Room neighbour = mock(Room.class);

            room.addNeighbour(RoomDirection.BOTTOM, neighbour);

            assertSame(neighbour, room.getNeighboursMap().get(RoomDirection.BOTTOM));
        }
    }
}
