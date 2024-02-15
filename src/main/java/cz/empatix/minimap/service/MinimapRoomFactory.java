package cz.empatix.minimap.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.AbstractMinimapRoom;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.rooms.service.Room;

import java.util.*;
import java.util.function.Function;

public class MinimapRoomFactory {
    private static <E extends AbstractMinimapRoom<E>> E createMapRoom(RoomDirection direction, Room neighbour, E minimapRoomParent,
                                                               Function<Room, E> roomFactory) {
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
        final E created = roomFactory.apply(neighbour);
        created.setMapPosition(x, y);
        return created;
    }

    public static <E extends AbstractMinimapRoom<E>> Map<Room, E> createMinimap(final List<Room> rooms, final Room startRoom,
                                                                                Function<Room, E> roomFactory) {
        final Queue<Room> roomsToVisit = new ArrayDeque<>(rooms.size());
        final Map<Room, E> minimapRooms = new HashMap<>(rooms.size());

        final E startMinimapRoom = roomFactory.apply(startRoom);
        startMinimapRoom.setMapPosition(0, 0);

        roomsToVisit.add(startRoom);
        minimapRooms.put(startRoom, startMinimapRoom);

        while (!roomsToVisit.isEmpty()) {
            final Room room = roomsToVisit.poll();
            final E minimapRoomParent = minimapRooms.get(room);
            room.getNeighboursMap().forEach((direction, neighbour) -> {
                if (!minimapRooms.containsKey(neighbour)) {
                    final E minimapRoom = createMapRoom(direction, neighbour, minimapRoomParent, roomFactory);
                    minimapRooms.put(neighbour, minimapRoom);
                    roomsToVisit.add(neighbour);

                    minimapRoomParent.addConnection(direction, minimapRoom);
                    minimapRoom.addConnection(direction.getOpposite(), minimapRoomParent);
                }
            });
        }
        return minimapRooms;
    }
}
