package cz.empatix.minimap.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.BigMinimapRoom;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.EnumMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.when;

/**
 * Covers the breadth-first grid placement in {@link MinimapRoomFactory#createMinimap}. The factory
 * walks {@link Room#getNeighboursMap()} from the start room and assigns each minimap room a relative
 * grid coordinate (start at the origin, RIGHT = +x, TOP = -y, etc.). The {@code BigMinimapRoom}
 * factory function fetches an atlas entry, so the test runs under a mocked {@link AssetsService}.
 */
class MinimapRoomFactoryTest {

    @BeforeAll
    static void headlessAudio() {
        cz.empatix.audio.init.ALInit.canUseAL = false;
    }

    private Room room(final RoomType type) {
        final Room r = mock(Room.class);
        when(r.getRoomType()).thenReturn(type);
        when(r.getNeighboursMap()).thenReturn(new EnumMap<>(RoomDirection.class));
        return r;
    }

    private void link(final Room from, final RoomDirection direction, final Room to) {
        from.getNeighboursMap().put(direction, to);
        to.getNeighboursMap().put(direction.getOpposite(), from);
    }

    @Test
    void singleRoomMapPlacesStartAtOrigin() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);

            final Map<Room, BigMinimapRoom> map =
                    MinimapRoomFactory.createMinimap(java.util.List.of(start), start, BigMinimapRoom::new);

            assertEquals(1, map.size());
            final BigMinimapRoom startMini = map.get(start);
            assertEquals(0, startMini.getX());
            assertEquals(0, startMini.getY());
            assertTrue(startMini.getNeighbours().isEmpty());
        }
    }

    @Test
    void neighboursArePlacedRelativeToParentAndCrossConnected() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);
            final Room rightRoom = room(RoomType.NORMAL);
            final Room topRoom = room(RoomType.SHOP);

            link(start, RoomDirection.RIGHT, rightRoom);
            link(start, RoomDirection.TOP, topRoom);

            final Map<Room, BigMinimapRoom> map = MinimapRoomFactory.createMinimap(
                    java.util.List.of(start, rightRoom, topRoom), start, BigMinimapRoom::new);

            assertEquals(3, map.size());

            // RIGHT neighbour -> x + 1, same y.
            final BigMinimapRoom right = map.get(rightRoom);
            assertEquals(1, right.getX());
            assertEquals(0, right.getY());

            // TOP neighbour -> same x, y - 1.
            final BigMinimapRoom top = map.get(topRoom);
            assertEquals(0, top.getX());
            assertEquals(-1, top.getY());

            // Connections are wired in both directions.
            final BigMinimapRoom startMini = map.get(start);
            assertSame(right, startMini.getNeighbours().get(RoomDirection.RIGHT));
            assertSame(top, startMini.getNeighbours().get(RoomDirection.TOP));
            assertSame(startMini, right.getNeighbours().get(RoomDirection.LEFT));
            assertSame(startMini, top.getNeighbours().get(RoomDirection.BOTTOM));
        }
    }

    @Test
    void chainedRoomsAccumulateOffsets() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            // start -RIGHT-> a -RIGHT-> b  => x coordinates 0, 1, 2.
            final Room start = room(RoomType.START);
            final Room a = room(RoomType.NORMAL);
            final Room b = room(RoomType.BOSS);
            link(start, RoomDirection.RIGHT, a);
            link(a, RoomDirection.RIGHT, b);

            final Map<Room, BigMinimapRoom> map = MinimapRoomFactory.createMinimap(
                    java.util.List.of(start, a, b), start, BigMinimapRoom::new);

            assertEquals(0, map.get(start).getX());
            assertEquals(1, map.get(a).getX());
            assertEquals(2, map.get(b).getX());
        }
    }
}
