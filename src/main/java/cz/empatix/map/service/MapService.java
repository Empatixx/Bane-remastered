package cz.empatix.map.service;

import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class MapService {
    private final static Logger LOGGER = LoggerFactory.getLogger(MapService.class);
    private final List<Room> rooms;
    private final List<Room> pathRooms;
    private Room currentRoom;

    public MapService(final List<Room> rooms,
                      final List<Room> pathRooms) {
        this.pathRooms = pathRooms;
        this.rooms = rooms;
    }

    public List<Renderable> toRender(double dt) {
        return Stream.of(pathRooms, rooms)
                .flatMap(List::stream)
                .map(Room::toRender)
                .flatMap(List::stream)
                .collect(Collectors.toList());
    }

    public void update(final double dt,
                       final Player player) {
        rooms.forEach(room -> room.update(dt));
        updateCurrentRoom(player, (float) dt);
    }

    private void updateCurrentRoom(final Player p, float dt) {
        try {
            final Vector2f position = p.getPosition();
            final Room previousRoom = currentRoom;
            currentRoom = pathRooms
                    .stream()
                    .filter(r -> r.isInRoom(position.x(), position.y()))
                    .findFirst()
                    .orElseGet(() -> rooms
                            .stream()
                            .filter(r -> r.isInRoom(position.x(), position.y()))
                            .findFirst()
                            .orElseThrow(() -> new RuntimeException("Could not match any room for player")));

            final int xMin = currentRoom.getMinX();
            final int yMin = currentRoom.getMinY();
            final int xMax = currentRoom.getMaxX();
            final int yMax = currentRoom.getMaxY();

            Camera.setSmoothlyMinDimensions(xMin, yMin, dt);
            Camera.setSmoothlyMaxDimensions(xMax, yMax, dt);

            if(previousRoom != null && previousRoom != currentRoom) {
                LOGGER.info("Player entered room: {}", currentRoom);
                currentRoom.enterEvent(p);
                previousRoom.leaveEvent(p);
            }
        } catch (RuntimeException e) {
            LOGGER.error("No room found!", e);
        }
    }
}
