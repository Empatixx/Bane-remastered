package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Covers the visibility/render-type bookkeeping of {@link BigMinimapRoom} (an
 * {@link AbstractMinimapRoom}). The constructor fetches a minimap atlas entry, so the test runs
 * under a mocked {@link AssetsService}.
 */
@ExtendWith(HeadlessAssets.class)
class BigMinimapRoomTest {

    private BigMinimapRoom roomOfType(final RoomType type) {
        final Room room = mock(Room.class);
        when(room.getRoomType()).thenReturn(type);
        return new BigMinimapRoom(room);
    }

    @Test
    void newRoomIsUnvisitedWithDefaultMapPosition() {
        final BigMinimapRoom room = roomOfType(RoomType.NORMAL);

        assertFalse(room.isVisited());
        assertEquals(0, room.getX());
        assertEquals(0, room.getY());
        assertTrue(room.getNeighbours().isEmpty());
    }

    @Test
    void visitFlipsTheVisitedFlag() {
        final BigMinimapRoom room = roomOfType(RoomType.START);
        room.visit();

        assertTrue(room.isVisited());
    }

    @Test
    void setMapPositionIsStored() {
        final BigMinimapRoom room = roomOfType(RoomType.SHOP);
        room.setMapPosition(3, -2);

        assertEquals(3, room.getX());
        assertEquals(-2, room.getY());
    }

    @Test
    void renderTypeDrivesAlphaVisibility() {
        final BigMinimapRoom room = roomOfType(RoomType.NORMAL);

        // UNSEEN -> alpha 0 -> not rendered.
        room.setRenderType(BigMinimapRenderType.UNSEEN);
        assertFalse(room.canRender());
        assertSame(BigMinimapRenderType.UNSEEN, room.getRenderType());

        // CURRENT -> alpha 1 -> rendered.
        room.setRenderType(BigMinimapRenderType.CURRENT);
        assertTrue(room.canRender());
    }

    @Test
    void addConnectionRegistersNeighbourUnderDirection() {
        final BigMinimapRoom room = roomOfType(RoomType.NORMAL);
        final BigMinimapRoom neighbour = roomOfType(RoomType.BOSS);

        room.addConnection(RoomDirection.LEFT, neighbour);

        assertEquals(1, room.getNeighbours().size());
        assertSame(neighbour, room.getNeighbours().get(RoomDirection.LEFT));
    }

    @Test
    void treasureAndSacrificeAndBossTypesConstructWithoutError() {
        // Exercises every branch of getTextureFrame that has a frame.
        assertFalse(roomOfType(RoomType.TREASURE).isVisited());
        assertFalse(roomOfType(RoomType.SACRIFICE).isVisited());
        assertFalse(roomOfType(RoomType.BOSS).isVisited());
    }
}
