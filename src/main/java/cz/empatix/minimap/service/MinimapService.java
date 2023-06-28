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
        final Room startRoom = mapGenerationResult.rooms()
                .stream()
                .filter(r -> r instanceof StartRoom)
                .findFirst().orElseThrow(); //todo: make enums to rooms, this is not fancy
        this.minimapRooms = createMinimap(mapGenerationResult.rooms(), startRoom);
        visit(startRoom);
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
        return minimapRooms;
    }

    public void visit(final Room room) {
        final MinimapRoom minimapRoom = minimapRooms.get(room);
        if (minimapRoom != null) {
            if(currentRoom != null) {
                currentRoom.setCurrent(false);
            }

            currentRoom = minimapRoom;
            currentRoom.visit();
            currentRoom.setCurrent(true);

            final Queue<Room> roomsToVisit = new ArrayDeque<>(minimapRooms.size());
            final Map<Room, Integer> levels = new HashMap<>(minimapRooms.size());
            final List<Room> visited = new ArrayList<>(minimapRooms.size());

            roomsToVisit.add(room);
            levels.put(room, 0);

            while(!roomsToVisit.isEmpty()){
                final Room r = roomsToVisit.poll();
                final int level = levels.get(r);
                final MinimapRoom minimapRoomNeighbour = minimapRooms.get(r);
                minimapRoomNeighbour.setOffset(currentRoom.getX(), currentRoom.getY());
                minimapRoomNeighbour.setRoomsAwayFromCenter(level);
                visited.add(r);
                r.getNeighboursMap().forEach((direction, neighbour) -> {
                    if(!visited.contains(neighbour)){
                        roomsToVisit.add(neighbour);
                        levels.put(neighbour, level + 1);
                    }
                });
            }
        }
    }

    public List<Renderable> toRender(final double dt) {
        final Collection<MinimapRoom> values = minimapRooms.values();
        return List.copyOf(values);
    }
}
