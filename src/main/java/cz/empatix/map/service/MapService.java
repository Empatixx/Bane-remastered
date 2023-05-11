package cz.empatix.map.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;

import java.util.List;
import java.util.Vector;

public class MapService {
    private final List<Room> rooms;
    private final List<Room> pathRooms;
    private Room currentRoom;

    public MapService(final List<Room> rooms,
                      final List<Room> pathRooms) {
        this.pathRooms = pathRooms;
        this.rooms = rooms;
    }

    public List<Drawable> toRender(double dt) {
        return rooms.stream()
                .map(Room::toRender)
                .flatMap(List::stream)
                .toList();
    }

    public void update(final double dt,
                       final Vector2f playerPosition) {
        rooms.forEach(room -> room.update(dt));
    }

    private void updateCurrentRoom(final Vector2f position) {
        currentRoom = pathRooms
                .stream()
                .filter(r -> r.isInRoom(position.x(), position.y()))
                .findFirst()
                .orElseGet(() -> rooms
                        .stream()
                        .filter(r -> r.isInRoom(position.x(), position.y()))
                        .findFirst()
                        .orElseThrow());

        final int x = currentRoom.getX();
        final int y = currentRoom.getY();

        final int width = currentRoom.getWidth();
        final int height = currentRoom.getHeight();

        Camera.setMinDimensions(x,y);
        Camera.setMaxDimensions(x+width, y+height);
    }
}
