package cz.empatix.minimap.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;

import java.util.*;
import java.util.stream.Stream;

public class MinimapService {
    private final Map<Room, MinimapRoom> minimapRooms;
    private MinimapRoom currentRoom;
    private final RightCornerMinimapRenderer minimapRenderer;
    public MinimapService(final MapGenerationResult mapGenerationResult) {
        this.minimapRooms = createMinimap(mapGenerationResult.rooms(), mapGenerationResult.startRoom());
        visit(mapGenerationResult.startRoom());
        update(1);

        this.minimapRenderer = new RightCornerMinimapRenderer(List.copyOf(minimapRooms.values()));
    }

    private Map<Room, MinimapRoom> createMinimap(final List<Room> rooms, final Room startRoom) {
        final Queue<Room> roomsToVisit = new ArrayDeque<>(rooms.size());
        final Map<Room, MinimapRoom> minimapRooms = new HashMap<>(rooms.size());

        final MinimapRoom startMinimapRoom = new MinimapRoom(0, 0, startRoom);

        roomsToVisit.add(startRoom);
        minimapRooms.put(startRoom, startMinimapRoom);

        while (!roomsToVisit.isEmpty()) {
            final Room room = roomsToVisit.poll();
            final MinimapRoom minimapRoomParent = minimapRooms.get(room);
            room.getNeighboursMap().forEach((direction, neighbour) -> {
                if (!minimapRooms.containsKey(neighbour)) {
                    final MinimapRoom minimapmRoom = createMapRoom(direction, neighbour, minimapRoomParent);
                    minimapRooms.put(neighbour, minimapRoom);
                    roomsToVisit.add(neighbour);
                }
            });
        }
        return minimapRooms;
    }


    private static MinimapRoom createMapRoom(RoomDirection direction, Room neighbour, MinimapRoom minimapRoomParent) {
        final int x = switch (direction) {
            case BOTTOM, TOP -> minimapRoomParent.getX();
            case LEFT -> minimapRoomParent.getX() - 1;
            case RIGHT -> minimapRoomParent.getX() + 1;
        };
        final int y = switch (direction) {
            case LEFT, RIGHT -> minimapRoomParent.getY();
            case BOTTOM -> minimapRoomParent.getY() + 1;
            case TOP -> minimapRoomParent.getY() - 1;
        };
        return new MinimapRoom(x, y, neighbour);
    }

    public void update(final double dt){
        minimapRooms.values().forEach(room -> room.update(currentRoom, (float)dt));
    }

    public void visit(final Room room) {
        final MinimapRoom minimapRoom = minimapRooms.get(room);
        if (minimapRoom != null) {
            if (currentRoom != null) {
                currentRoom.setCurrent(false);
            }

            currentRoom = minimapRoom;
            currentRoom.visit();
            currentRoom.setCurrent(true);
        }
    }

    public List<Renderable> toRender(final double dt) {
        return minimapRenderer.toRender();
    }
}
