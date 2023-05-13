package cz.empatix.map.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Stream;

public class MapService {
    private final static Logger LOGGER = LoggerFactory.getLogger(MapService.class);
    private final List<Room> rooms;
    private final List<Room> pathRooms;
    private Room currentRoom;
    private boolean isPathRoom = false;

    public MapService(final List<Room> rooms,
                      final List<Room> pathRooms) {
        this.pathRooms = pathRooms;
        this.rooms = rooms;
    }

    public List<Drawable> toRender(double dt) {
            return Stream.of(pathRooms, rooms)
                .flatMap(List::stream)
                .map(Room::toRender)
                .flatMap(List::stream)
                .toList();
    }

    public void update(final double dt,
                       final Vector2f playerPosition) {
        rooms.forEach(room -> room.update(dt));
        updateCurrentRoom(playerPosition);
    }

    private void updateCurrentRoom(final Vector2f position) {
        try{
            currentRoom = pathRooms
                    .stream()
                    .filter(r -> r.isInRoom(position.x(), position.y()))
                    .findFirst()
                    .orElseGet(() -> rooms
                            .stream()
                            .filter(r -> r.isInRoom(position.x(), position.y()))
                            .findFirst()
                            .orElseThrow(() -> new RuntimeException("Could not match any room for player")));

            final int x = currentRoom.getX();
            final int y = currentRoom.getY();

            final int width = currentRoom.getWidth();
            final int height = currentRoom.getHeight();

            Camera.setMinDimensions(x-10000,y-10000);
            Camera.setMaxDimensions(x+width+10000, y+height+10000);
        } catch (RuntimeException e){
            LOGGER.error("No room found!", e);
        }
    }
}
