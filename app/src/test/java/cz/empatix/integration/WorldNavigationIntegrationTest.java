package cz.empatix.integration;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.service.MapService;
import cz.empatix.player.Player;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless INTEGRATION test of the world-navigation pipeline: a real {@link MapService} resolves which
 * room a real {@link Player} stands in from the player's actual physics position, and fans a visit out
 * through its {@link cz.empatix.map.service.RoomVisitorObserver} when the player crosses into a new
 * room. Drives the real position -> room-resolution -> enter/leave -> visit chain that the existing
 * MapService unit test (which only covers apply()) does not.
 */
@Tag("integration")
@ExtendWith(HeadlessAssets.class)
class WorldNavigationIntegrationTest {

    /** A room occupying the horizontal band [xMin, xMax] with real in-bounds logic. */
    private static Room horizontalRoom(final int xMin, final int xMax) {
        final Room room = mock(Room.class);
        when(room.getMinX()).thenReturn(xMin);
        when(room.getMaxX()).thenReturn(xMax);
        when(room.getMinY()).thenReturn(0);
        when(room.getMaxY()).thenReturn(400);
        when(room.getState()).thenReturn(RoomState.OPEN);
        when(room.getRoomType()).thenReturn(RoomType.NORMAL);
        when(room.isInRoom(anyFloat(), anyFloat()))
                .thenAnswer(i -> withinX(i.getArgument(0), xMin, xMax));
        when(room.isInRoom(anyFloat(), anyFloat(), anyFloat()))
                .thenAnswer(i -> withinX(i.getArgument(0), xMin, xMax));
        return room;
    }

    private static boolean withinX(final float x, final int xMin, final int xMax) {
        return x >= xMin && x <= xMax;
    }

    @Test
    void walkingIntoANewRoomFiresAVisitForThatRoom() {
        final Room roomA = horizontalRoom(0, 100);
        final Room roomB = horizontalRoom(200, 300);

        final MapService mapService = new MapService();
        final List<Room> visited = new ArrayList<>();
        mapService.onVisit(visited::add);

        mapService.apply(new MapGenerationResult(roomA, List.of(roomA, roomB), List.of()));

        final Player player = new Player();

        player.setPosition(50, 50);   // inside room A
        mapService.update(1.0 / 60.0, player);
        assertTrue(visited.contains(roomA), "entering the start room A fires its visit");

        player.setPosition(250, 50);  // walked across into room B
        mapService.update(1.0 / 60.0, player);

        assertTrue(visited.contains(roomB), "crossing into room B fires room B's visit");
        assertSame(roomB, visited.get(visited.size() - 1), "the most recent visit is the newly-entered room");
    }
}
