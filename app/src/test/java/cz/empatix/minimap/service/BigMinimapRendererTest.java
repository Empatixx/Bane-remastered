package cz.empatix.minimap.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.BigMinimapRenderType;
import cz.empatix.minimap.domain.BigMinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.when;

/**
 * Covers the reveal/render-type logic of {@link BigMinimapRenderer} headlessly. {@code apply} builds
 * the minimap from a {@link MapGenerationResult}, marks the start room as CURRENT and propagates
 * VISITED / UNVISITED / UNSEEN render types outward. Audio is disabled so {@code activate} stays
 * GL/AL-free; the GL draw path ({@code toRender} contents beyond size) is not exercised.
 */
class BigMinimapRendererTest {

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
    void applyMarksStartAsCurrentAndVisited() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);
            final Room neighbour = room(RoomType.NORMAL);
            link(start, RoomDirection.RIGHT, neighbour);

            final BigMinimapRenderer renderer = new BigMinimapRenderer();
            renderer.apply(new MapGenerationResult(start, List.of(start, neighbour), List.of()));

            final Map<Room, BigMinimapRoom> minimap = currentMinimap(renderer);
            final BigMinimapRoom startMini = minimap.get(start);
            assertTrue(startMini.isVisited());
            assertSame(BigMinimapRenderType.CURRENT, startMini.getRenderType());
        }
    }

    @Test
    void unvisitedNeighbourOfCurrentBecomesUnvisited() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);
            final Room neighbour = room(RoomType.NORMAL);
            link(start, RoomDirection.RIGHT, neighbour);

            final BigMinimapRenderer renderer = new BigMinimapRenderer();
            renderer.apply(new MapGenerationResult(start, List.of(start, neighbour), List.of()));

            final BigMinimapRoom neighbourMini = currentMinimap(renderer).get(neighbour);
            // Directly adjacent to CURRENT but not yet visited -> UNVISITED.
            assertSame(BigMinimapRenderType.UNVISITED, neighbourMini.getRenderType());
            assertFalse(neighbourMini.isVisited());
        }
    }

    @Test
    void roomTwoStepsAwayFromCurrentIsUnseen() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            // start - near - far : far is behind an unvisited room, so it is UNSEEN.
            final Room start = room(RoomType.START);
            final Room near = room(RoomType.NORMAL);
            final Room far = room(RoomType.BOSS);
            link(start, RoomDirection.RIGHT, near);
            link(near, RoomDirection.RIGHT, far);

            final BigMinimapRenderer renderer = new BigMinimapRenderer();
            renderer.apply(new MapGenerationResult(start, List.of(start, near, far), List.of()));

            final Map<Room, BigMinimapRoom> minimap = currentMinimap(renderer);
            assertSame(BigMinimapRenderType.UNVISITED, minimap.get(near).getRenderType());
            assertSame(BigMinimapRenderType.UNSEEN, minimap.get(far).getRenderType());
        }
    }

    @Test
    void visitAllRevealsEveryRoomAsVisitedExceptCurrent() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);
            final Room near = room(RoomType.NORMAL);
            final Room far = room(RoomType.BOSS);
            link(start, RoomDirection.RIGHT, near);
            link(near, RoomDirection.RIGHT, far);

            final BigMinimapRenderer renderer = new BigMinimapRenderer();
            renderer.apply(new MapGenerationResult(start, List.of(start, near, far), List.of()));

            renderer.visitAll();

            final Map<Room, BigMinimapRoom> minimap = currentMinimap(renderer);
            assertTrue(minimap.values().stream().allMatch(BigMinimapRoom::isVisited));
            // The previously-CURRENT room keeps its CURRENT type; all others become VISITED.
            assertEquals(1, minimap.values().stream()
                    .filter(r -> r.getRenderType() == BigMinimapRenderType.CURRENT).count());
            assertEquals(2, minimap.values().stream()
                    .filter(r -> r.getRenderType() == BigMinimapRenderType.VISITED).count());
        }
    }

    @Test
    void activateDeactivateTogglesVisibilityAndRenderList() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);

            final Room start = room(RoomType.START);
            final BigMinimapRenderer renderer = new BigMinimapRenderer();
            renderer.apply(new MapGenerationResult(start, List.of(start), List.of()));

            assertFalse(renderer.isActive());
            assertTrue(renderer.toRender().isEmpty());

            renderer.activate();
            assertTrue(renderer.isActive());
            // Layout image + one minimap room.
            final List<Renderable> rendered = renderer.toRender();
            assertEquals(2, rendered.size());

            renderer.deactivate();
            assertFalse(renderer.isActive());
            assertTrue(renderer.toRender().isEmpty());
        }
    }

    @SuppressWarnings("unchecked")
    private static Map<Room, BigMinimapRoom> currentMinimap(final BigMinimapRenderer renderer) {
        try {
            final java.lang.reflect.Field field = BigMinimapRenderer.class.getDeclaredField("minimapRooms");
            field.setAccessible(true);
            return (Map<Room, BigMinimapRoom>) field.get(renderer);
        } catch (final ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        }
    }
}
