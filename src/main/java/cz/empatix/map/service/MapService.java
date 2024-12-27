package cz.empatix.map.service;

import cz.empatix.hud.service.SceneBackground;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
import cz.empatix.rooms.domain.RoomEvent;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.TileImpl;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Collection;
import java.util.List;
import java.util.function.Consumer;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class MapService {
    private final static Logger LOGGER = LoggerFactory.getLogger(MapService.class);
    private final SceneBackground sceneBackground;
    private List<Room> rooms;
    private List<Room> pathRooms;
    private final RoomEventObserver roomEventObserver;
    private final RoomVisitorObserver roomVisitorObserver;
    private Room currentRoom;
    private Room previousRoom;
    private boolean alreadyEntered;

    public MapService() {
        sceneBackground = new SceneBackground("ingame/tilebackground.tga");

        roomEventObserver = new RoomEventObserver();
        roomVisitorObserver = new RoomVisitorObserver();
    }

    public void apply(final MapGenerationResult mapGenerationResult){
        final int xMin = mapGenerationResult.startRoom().getMinX();
        final int yMin = mapGenerationResult.startRoom().getMinY();
        final int xMax = mapGenerationResult.startRoom().getMaxX();
        final int yMax = mapGenerationResult.startRoom().getMaxY();

        Camera.getInstance().setMinPosition(xMin, yMin);
        Camera.getInstance().setMaxPosition(xMax, yMax);

        pathRooms = mapGenerationResult.paths();
        rooms = mapGenerationResult.rooms();

        roomEventObserver.apply(mapGenerationResult);

        currentRoom = mapGenerationResult.startRoom();
        roomVisitorObserver.observe(currentRoom);
    }
    public List<Renderable> toRender(final double dt) {
        final List<Renderable> renderables = Stream.of(pathRooms, rooms)
                .flatMap(Collection::stream)
                .map(Room::toRender)
                .flatMap(List::stream)
                .collect(Collectors.toList());
        renderables.add(sceneBackground);
        return renderables;
    }

    public void update(final double dt,
                       final Player player) {
        rooms.forEach(room -> room.update(dt));
        updateCurrentRoom(player, (float) dt);
    }
    public void fixedUpdate(final double dt) {
        rooms.forEach(room -> room.fixedUpdate(dt));
        roomEventObserver.observe();
    }

    private void updateCurrentRoom(final Player p, float dt) {
        try {
            final Vector2f position = new Vector2f(p.getPosition());
            final Room previousRoom = currentRoom;
            currentRoom = pathRooms
                    .stream()
                    .filter(r -> r.isInRoom(position.x() + p.getWidth()/2f, position.y() + p.getHeight()/2f))
                    .findFirst()
                    .orElseGet(() -> rooms
                            .stream()
                            .filter(r -> r.isInRoom(position.x(), position.y()))
                            .findFirst()
                            .orElseThrow(() -> new IllegalStateException("Could not match any room for player")));

            final int xMin = currentRoom.getMinX();
            final int yMin = currentRoom.getMinY();
            final int xMax = currentRoom.getMaxX();
            final int yMax = currentRoom.getMaxY();

            Camera.getInstance().setSmoothlyMinPosition(xMin, yMin, dt);
            Camera.getInstance().setSmoothlyMaxPosition(xMax, yMax, dt);

            if (previousRoom != null && previousRoom != currentRoom) {
                previousRoom.leaveEvent(p);
                alreadyEntered = false;
            }
            // not in leave event, because we want enter room with little offset
            if (!alreadyEntered){
                if (currentRoom.isInRoom(position.x() + p.getWidth()/2f, position.y() + p.getWidth()/2f, TileImpl.SCALE_TILE_SIZE)){
                    LOGGER.info("Player entered room: {}", currentRoom);
                    currentRoom.enterEvent(p);
                    roomVisitorObserver.observe(currentRoom);
                    alreadyEntered = true;
                }
            }
        } catch (final IllegalStateException e) {
            LOGGER.error("No room found!", e);
        }
    }
    public void cleanUp(){
        rooms.forEach(Room::cleanUp);
        pathRooms.forEach(Room::cleanUp);
    }

    public List<Room> getRooms() {
        return rooms;
    }

    public void onRoomEventChange(final Consumer<RoomEvent> listener) {
        roomEventObserver.addListener(listener);
    }
    public void onVisit(final Consumer<Room> listener) {
        roomVisitorObserver.addListener(listener);
    }

}
