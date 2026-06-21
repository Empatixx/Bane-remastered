package cz.empatix.rooms.service;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.CoinService;
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
 * Headless coverage for the real {@link ShopRoom}: geometry from its backing common tile map, the
 * SHOP room type, the default OPEN state and neighbour bookkeeping. The {@link CoinService}
 * collaborator (which would otherwise build HUD widgets) is mocked because it is only consumed
 * inside {@code init()}, which is not exercised here.
 */
@ExtendWith(HeadlessAssets.class)
class ShopRoomTest {

    private static final int SCALED = TileImpl.TILE_SIZE * TileImpl.SCALE;

    private static ShopRoom room(final int rows, final int cols, final int xOff, final int yOff) {
        return new ShopRoom(mock(ItemService.class), rows, cols, xOff, yOff,
                EnumSet.noneOf(RoomDirection.class), mock(StatisticsService.class),
                mock(CoinService.class), new RngService(0L));
    }

    @Test
    void geometryDerivesFromRowsColsAndOffset() {
        final ShopRoom room = room(12, 16, 64, 0);

        assertEquals(16 * SCALED, room.getWidth());
        assertEquals(12 * SCALED, room.getHeight());
        assertEquals(64, room.getX());
        assertEquals(0, room.getY());
    }

    @Test
    void typeStateAndEmptyObjectsBeforeInit() {
        final ShopRoom room = room(12, 14, 0, 0);

        assertEquals(RoomType.SHOP, room.getRoomType());
        assertEquals(RoomState.OPEN, room.getState());
        assertTrue(room.getRoomObjects().isEmpty());
    }

    @Test
    void addNeighbourIsStoredAndRetrievable() {
        final ShopRoom room = room(12, 14, 0, 0);
        final Room neighbour = mock(Room.class);

        room.addNeighbour(RoomDirection.TOP, neighbour);

        assertSame(neighbour, room.getNeighboursMap().get(RoomDirection.TOP));
    }
}
