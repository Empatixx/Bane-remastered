package cz.empatix.minimap.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.StartRoom;

import java.util.*;

public class MinimapService {
    private final Map<Room, MinimapRoom> minimapRooms;
    private MinimapRoom currentRoom;

    public MinimapService(final MapGenerationResult mapGenerationResult) {
        this.minimapRooms = createMinimap(mapGenerationResult.rooms());
    }

    private Map<Room, MinimapRoom> createMinimap(final List<Room> rooms) {
        final Queue<Room> roomsToVisit = new ArrayDeque<>(rooms.size());
        final Map<Room, MinimapRoom> minimapRooms = new HashMap<>(rooms.size());

        final Room startRoom = rooms
                .stream()
                .filter(r -> r instanceof StartRoom)
                .findFirst().orElseThrow(); //todo: make enums to rooms, this is not fancy
        final MinimapRoom startMinimapRoom = new MinimapRoom(0, 0, startRoom);
        currentRoom = startMinimapRoom;
        currentRoom.setCurrent(true);

        roomsToVisit.add(startRoom);
        minimapRooms.put(startRoom, startMinimapRoom);

        while (!roomsToVisit.isEmpty()) {
            final Room room = roomsToVisit.poll();
            final MinimapRoom minimapRoomParent = minimapRooms.get(room);
            room.getNeighboursMap().forEach((direction, neighbour) -> {
                if (!minimapRooms.containsKey(neighbour)) {
                    final int xNeighbour = switch (direction) {
                        case BOTTOM, TOP -> minimapRoomParent.getX();
                        case LEFT -> minimapRoomParent.getX() - 1;
                        case RIGHT -> minimapRoomParent.getX() + 1;
                    };
                    final int yNeighbour = switch (direction) {
                        case LEFT, RIGHT -> minimapRoomParent.getY();
                        case BOTTOM -> minimapRoomParent.getY() + 1;
                        case TOP -> minimapRoomParent.getY() - 1;
                    };
                    final MinimapRoom minimapRoom = new MinimapRoom(xNeighbour, yNeighbour, neighbour);
                    minimapRoom.setConnection(direction.getOpposite());
                    minimapRooms.put(neighbour, minimapRoom);
                    roomsToVisit.add(neighbour);
                } else {
                    final MinimapRoom minimapRoom = minimapRooms.get(neighbour);
                    minimapRoom.setConnection(direction.getOpposite());
                }
            });
        }
        startMinimapRoom.visit();
        return minimapRooms;
    }

    public void visit(final Room room) {
        final MinimapRoom minimapRoom = minimapRooms.get(room);
        if (minimapRoom != null) {
            currentRoom.setCurrent(false);

            currentRoom = minimapRoom;
            currentRoom.visit();
            currentRoom.setCurrent(true);

            minimapRooms.values().forEach(miniRoom -> miniRoom.setOffset(currentRoom.getX(), currentRoom.getY()));
        }
    }

    public List<Renderable> toRender(final double dt) {
        final Collection<MinimapRoom> values = minimapRooms.values();
        return List.copyOf(values);
    }
}
