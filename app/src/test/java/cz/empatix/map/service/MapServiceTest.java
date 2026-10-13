package cz.empatix.map.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.anyDouble;
import static org.mockito.ArgumentMatchers.anyFloat;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Covers the room registration / current-room tracking of {@link MapService}. The constructor builds
 * a {@code SceneBackground} (asset), so the test runs under a mocked {@link AssetsService};
 * {@code Camera} is headless-safe. The GL render contents are not asserted beyond list composition.
 */
@ExtendWith(HeadlessAssets.class)
class MapServiceTest {

    private Room boundedRoom() {
        final Room room = mock(Room.class);
        when(room.getMinX()).thenReturn(0);
        when(room.getMinY()).thenReturn(0);
        when(room.getMaxX()).thenReturn(1000);
        when(room.getMaxY()).thenReturn(1000);
        // RoomEventObserver.apply collects rooms into a Map keyed by state (Collectors.toMap rejects
        // null values); observe()/toRender touch these too — give the mock non-null answers.
        when(room.getState()).thenReturn(RoomState.OPEN);
        when(room.getRoomType()).thenReturn(RoomType.NORMAL);
        when(room.toRender()).thenReturn(List.of());
        return room;
    }

    @Test
    void applyRegistersRoomsAndExposesThem() {
        final MapService service = new MapService();
        final Room start = boundedRoom();
        final Room other = boundedRoom();

        service.apply(new MapGenerationResult(start, List.of(start, other), List.of()));

        assertEquals(List.of(start, other), service.getRooms());
    }

    @Test
    void applyFiresTheVisitorObserverForTheStartRoom() {
        final MapService service = new MapService();
        final Room start = boundedRoom();

        final java.util.List<Room> visited = new java.util.ArrayList<>();
        service.onVisit(visited::add);

        service.apply(new MapGenerationResult(start, List.of(start), List.of()));

        assertEquals(1, visited.size());
        assertSame(start, visited.get(0));
    }

    @Test
    void toRenderFlattensRoomRenderablesAndAppendsBackground() {
        final MapService service = new MapService();
        final Room start = boundedRoom();
        final Renderable roomRenderable = mock(Renderable.class);
        when(start.toRender()).thenReturn(List.of(roomRenderable));

        service.apply(new MapGenerationResult(start, List.of(start), List.of()));

        final List<Renderable> rendered = service.toRender(0.016);
        // One room renderable + the scene background.
        assertEquals(2, rendered.size());
        assertTrue(rendered.contains(roomRenderable));
    }

    @Test
    void updateAdvancesRoomStateAndEntersTheCurrentRoom() {
        final MapService service = new MapService();
        final Room start = boundedRoom();
        when(start.isInRoom(anyFloat(), anyFloat())).thenReturn(true);
        when(start.isInRoom(anyFloat(), anyFloat(), anyFloat())).thenReturn(true);

        service.apply(new MapGenerationResult(start, List.of(start), List.of()));

        final Player player = mock(Player.class);
        when(player.getPosition()).thenReturn(new Vector2f(100, 100));
        when(player.getWidth()).thenReturn(32);
        when(player.getHeight()).thenReturn(32);

        service.update(0.016, player);

        // Each room is ticked, and the player enters the resolved current room.
        verify(start).update(anyDouble());
        verify(start).enterEvent(player);
    }

    @Test
    void fixedUpdateTicksRoomsAndPollsEvents() {
        final MapService service = new MapService();
        final Room start = boundedRoom();
        service.apply(new MapGenerationResult(start, List.of(start), List.of()));

        service.fixedUpdate(0.016);

        verify(start).fixedUpdate(anyDouble());
    }

    @Test
    void cleanUpReleasesRoomsAndPaths() {
        final MapService service = new MapService();
        final Room start = boundedRoom();
        final Room path = boundedRoom();
        service.apply(new MapGenerationResult(start, List.of(start), List.of(path)));

        service.cleanUp();

        verify(start).cleanUp();
        verify(path).cleanUp();
    }
}
