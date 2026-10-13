package cz.empatix.minimap.service;

import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class SmallMinimapRenderer implements MinimapRenderer {
    private static final Logger LOGGER = LoggerFactory.getLogger(SmallMinimapRenderer.class);
    private final static int OFFSET_FROM_BORDER = 32;
    public final static int MINIMAP_POSITION_X = 835 - OFFSET_FROM_BORDER + 62 - MinimapRoom.DIMENSIONS / 2;
    public final static int MINIMAP_POSITION_Y = OFFSET_FROM_BORDER + 62 - MinimapRoom.DIMENSIONS / 2;
    private final static int MINIMAP_LAYOUT_POSITION_X = 835 - OFFSET_FROM_BORDER;
    private final static int MINIMAP_LAYOUT_POSITION_Y = OFFSET_FROM_BORDER;
    private final CommonImage minimapLayout;
    private Map<Room, MinimapRoom> minimapRooms;

    public SmallMinimapRenderer() {
        this.minimapLayout = new CommonImage("ingame/minimap_layout.tga", new Vector2f(MINIMAP_LAYOUT_POSITION_X, MINIMAP_LAYOUT_POSITION_Y), 1.25f, 4);
        this.minimapRooms = new HashMap<>();
    }


    @Override
    public void apply(MapGenerationResult result) {
        minimapRooms = MinimapRoomFactory.createMinimap(result.rooms(), result.startRoom(), MinimapRoom::new);
        visit(result.startRoom());
        update(1, 0, 0);
    }

    @Override
    public void visit(final Room room) {
        final MinimapRoom currentRoom = minimapRooms.get(room);
        if (currentRoom == null) {
            LOGGER.error("Room {} is not present in the minimap", room);
            return;
        }
        currentRoom.visit();
        currentRoom.setRoomsAwayFromCenter(0);

        final Queue<MinimapRoom> roomsToVisit = new ArrayDeque<>(minimapRooms.size());
        final Set<MinimapRoom> visited = new HashSet<>(minimapRooms.size());
        roomsToVisit.add(currentRoom);

        while (!roomsToVisit.isEmpty()) {
            final MinimapRoom r = roomsToVisit.poll();
            visited.add(r);
            r.getNeighbours().forEach((direction, neighbour) -> {
                if (!visited.contains(neighbour)) {
                    LOGGER.info("Visiting room: {}, level {}", neighbour, r.getRoomsAwayFromCenter() + 1);
                    neighbour.setRoomsAwayFromCenter(r.getRoomsAwayFromCenter() + 1);
                    roomsToVisit.add(neighbour);
                }
            });
        }
        minimapRooms.values().forEach(r -> {
            r.setPosition(currentRoom.getX(), currentRoom.getY());
        });
    }

    @Override
    public void visitAll() {
        minimapRooms.values().forEach(MinimapRoom::visit);
    }

    @Override
    public List<Renderable> toRender() {
        final List<Renderable> renderables = new ArrayList<>(minimapRooms.size() + 1);
        renderables.add(minimapLayout);
        renderables.addAll(minimapRooms.values());
        return renderables;
    }

    @Override
    public void mouseReleased(int button, float mouseX, float mouseY) {

    }

    @Override
    public void mousePressed(int button, float mouseX, float mouseY) {

    }

    @Override
    public void activate() {

    }

    @Override
    public void deactivate() {

    }


    @Override
    public void update(float dt, final float mouseX, final float mouseY) {
        minimapRooms.values().forEach(minimapRoom -> minimapRoom.update(dt));
    }

}
