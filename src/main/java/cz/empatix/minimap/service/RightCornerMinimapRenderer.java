package cz.empatix.minimap.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.minimap.domain.MinimapRoomDrawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;

import java.util.*;
import java.util.stream.Collectors;

public class RightCornerMinimapRenderer implements MinimapRenderer {
    private final static int OFFSET_FROM_BORDER = 32;
    public final static int MINIMAP_POSITION_X = 835 - OFFSET_FROM_BORDER + 62 - MinimapRoomDrawable.DIMENSIONS / 2;
    public final static int MINIMAP_POSITION_Y = OFFSET_FROM_BORDER + 62 - MinimapRoomDrawable.DIMENSIONS / 2;
    private final static int MINIMAP_LAYOUT_POSITION_X = 835 - OFFSET_FROM_BORDER;
    private final static int MINIMAP_LAYOUT_POSITION_Y = OFFSET_FROM_BORDER;
    private final CommonImage minimapLayout;
    private final Map<MinimapRoom, MinimapRoomDrawable> minimapRooms;

    public RightCornerMinimapRenderer(final List<MinimapRoom> rooms) {
        this.minimapLayout = new CommonImage("ingame/minimap_layout.tga", new Vector2f(MINIMAP_LAYOUT_POSITION_X, MINIMAP_LAYOUT_POSITION_Y), 1.25f, 4);
        this.minimapRooms = rooms.stream()
                .collect(Collectors.toMap(room -> room, MinimapRoomDrawable::new));
    }

    public void visit(final MinimapRoom minimapRoom){
        final Queue<MinimapRoom> roomsToVisit = new ArrayDeque<>(minimapRooms.size());
        final Map<MinimapRoom, Integer> levels = new HashMap<>(minimapRooms.size());
        final List<MinimapRoom> visited = new ArrayList<>(minimapRooms.size());

        roomsToVisit.add(minimapRoom);
        levels.put(minimapRoom, 0);

        while (!roomsToVisit.isEmpty()) {
            final MinimapRoom r = roomsToVisit.poll();
            final int level = levels.get(r);
            final MinimapRoomDrawable minimapRoomNeighbour = minimapRooms.get(r);
            minimapRoomNeighbour.setRoomsAwayFromCenter(level);
            visited.add(r);
            r.getRoomDirections().forEach(direction -> {
                if (!visited.contains(direction)) {
                    roomsToVisit.add(direction);
                    levels.put(direction, level + 1);
                }
            });
        }
    }

    public void visit(final Room currentRoom) {
        final Queue<Room> roomsToVisit = new ArrayDeque<>(minimapRooms.size());
        final Map<Room, Integer> levels = new HashMap<>(minimapRooms.size());
        final List<Room> visited = new ArrayList<>(minimapRooms.size());

        roomsToVisit.add(currentRoom);
        levels.put(currentRoom, 0);

        while (!roomsToVisit.isEmpty()) {
            final Room r = roomsToVisit.poll();
            final int level = levels.get(r);
            final MinimapRoomDrawable minimapRoomNeighbour = minimapRooms.get(r);
            minimapRoomNeighbour.setRoomsAwayFromCenter(level);
            visited.add(r);
            r.getNeighboursMap().forEach((direction, neighbour) -> {
                if (!visited.contains(neighbour)) {
                    roomsToVisit.add(neighbour);
                    levels.put(neighbour, level + 1);
                }
            });
        }
    }

    public List<Renderable> toRender() {
        final List<Renderable> renderables = new ArrayList<>(minimapRooms.size() + 1);
        renderables.add(minimapLayout);
        renderables.addAll(minimapRooms.values());
        return renderables;
    }
}
