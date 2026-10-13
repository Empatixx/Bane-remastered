package cz.empatix.minimap.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Covers the reveal / distance-from-center bookkeeping of {@link SmallMinimapRenderer} headlessly.
 * {@code apply} builds the minimap from a {@link MapGenerationResult}, visits the start room and runs a
 * BFS that stamps every room with its hop distance ({@code roomsAwayFromCenter}); {@code visit} re-roots
 * that BFS on a new current room. {@link MinimapRoom} fetches a minimap atlas entry in its constructor,
 * so the test runs under a mocked {@code AssetsService} via {@link HeadlessAssets}. Only computed
 * values (distances, visited flags, render-list size) are asserted, never GL draw output.
 */
@ExtendWith(HeadlessAssets.class)
class SmallMinimapRendererTest {

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

    @SuppressWarnings("unchecked")
    private static Map<Room, MinimapRoom> minimap(final SmallMinimapRenderer renderer) {
        try {
            final java.lang.reflect.Field field = SmallMinimapRenderer.class.getDeclaredField("minimapRooms");
            field.setAccessible(true);
            return (Map<Room, MinimapRoom>) field.get(renderer);
        } catch (final ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        }
    }

    @Test
    void applyVisitsStartAndMarksItAtDistanceZero() {
        final Room start = room(RoomType.START);
        final Room neighbour = room(RoomType.NORMAL);
        link(start, RoomDirection.RIGHT, neighbour);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, neighbour), List.of()));

        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        assertTrue(rooms.get(start).isVisited());
        assertEquals(0, rooms.get(start).getRoomsAwayFromCenter());
    }

    @Test
    void applyStampsBfsHopDistanceFromStart() {
        // start - near - far : distances 0, 1, 2 from the current (start) room.
        final Room start = room(RoomType.START);
        final Room near = room(RoomType.NORMAL);
        final Room far = room(RoomType.BOSS);
        link(start, RoomDirection.RIGHT, near);
        link(near, RoomDirection.RIGHT, far);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, near, far), List.of()));

        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        assertEquals(0, rooms.get(start).getRoomsAwayFromCenter());
        assertEquals(1, rooms.get(near).getRoomsAwayFromCenter());
        assertEquals(2, rooms.get(far).getRoomsAwayFromCenter());
    }

    @Test
    void visitReRootsDistancesOnNewCurrentRoom() {
        final Room start = room(RoomType.START);
        final Room near = room(RoomType.NORMAL);
        final Room far = room(RoomType.BOSS);
        link(start, RoomDirection.RIGHT, near);
        link(near, RoomDirection.RIGHT, far);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, near, far), List.of()));

        // move the "you are here" centre to the far room
        renderer.visit(far);

        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        assertEquals(0, rooms.get(far).getRoomsAwayFromCenter());
        assertEquals(1, rooms.get(near).getRoomsAwayFromCenter());
        assertEquals(2, rooms.get(start).getRoomsAwayFromCenter());
    }

    @Test
    void visitOnlyMarksTheVisitedRoomNotItsNeighbours() {
        final Room start = room(RoomType.START);
        final Room neighbour = room(RoomType.NORMAL);
        link(start, RoomDirection.RIGHT, neighbour);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, neighbour), List.of()));

        // only start has been visited so far (apply visits the start room)
        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        assertTrue(rooms.get(start).isVisited());
        assertFalse(rooms.get(neighbour).isVisited());

        renderer.visit(neighbour);
        assertTrue(rooms.get(neighbour).isVisited());
    }

    @Test
    void visitingAnUnknownRoomIsIgnored() {
        final Room start = room(RoomType.START);
        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start), List.of()));

        // a room that is not part of the generated map must not throw or alter anything
        renderer.visit(room(RoomType.SHOP));

        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        assertEquals(1, rooms.size());
        assertTrue(rooms.get(start).isVisited());
    }

    @Test
    void visitAllMarksEveryRoomAsVisited() {
        final Room start = room(RoomType.START);
        final Room near = room(RoomType.NORMAL);
        final Room far = room(RoomType.BOSS);
        link(start, RoomDirection.RIGHT, near);
        link(near, RoomDirection.RIGHT, far);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, near, far), List.of()));

        renderer.visitAll();

        assertTrue(minimap(renderer).values().stream().allMatch(MinimapRoom::isVisited));
    }

    @Test
    void toRenderContainsLayoutPlusEveryMinimapRoom() {
        final Room start = room(RoomType.START);
        final Room near = room(RoomType.NORMAL);
        link(start, RoomDirection.RIGHT, near);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, near), List.of()));

        // layout image + 2 minimap rooms
        final List<Renderable> rendered = renderer.toRender();
        assertEquals(3, rendered.size());
    }

    @Test
    void currentRoomIsVisibleWhileFarUnvisitedRoomsFadeOut() {
        // start(0) - r1(1) - r2(2) - r3(3) : alpha drops with distance and the >1-away-unvisited penalty.
        final Room start = room(RoomType.START);
        final Room r1 = room(RoomType.NORMAL);
        final Room r2 = room(RoomType.NORMAL);
        final Room r3 = room(RoomType.NORMAL);
        link(start, RoomDirection.RIGHT, r1);
        link(r1, RoomDirection.RIGHT, r2);
        link(r2, RoomDirection.RIGHT, r3);

        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start, r1, r2, r3), List.of()));

        final Map<Room, MinimapRoom> rooms = minimap(renderer);
        // distance 0 -> alpha 1 -> always rendered
        assertTrue(rooms.get(start).canRender());
        // far enough unvisited rooms fade below zero alpha and stop rendering
        assertFalse(rooms.get(r3).canRender());
    }

    @Test
    void updateIsHeadlessSafeAndKeepsRenderList() {
        final Room start = room(RoomType.START);
        final SmallMinimapRenderer renderer = new SmallMinimapRenderer();
        renderer.apply(new MapGenerationResult(start, List.of(start), List.of()));

        renderer.update(0.016f, 0, 0);

        // layout + the single start room
        assertEquals(2, renderer.toRender().size());
        assertSame(start, minimap(renderer).keySet().iterator().next());
    }
}
