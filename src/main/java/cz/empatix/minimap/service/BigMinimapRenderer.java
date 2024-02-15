package cz.empatix.minimap.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.BigMinimapRenderType;
import cz.empatix.minimap.domain.BigMinimapRoom;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class BigMinimapRenderer implements MinimapRenderer{
    private static final Logger LOGGER = LoggerFactory.getLogger(BigMinimapRenderer.class);
    private static final float MINIMAP_LAYOUT_POSITION_X = WindowHandler.TARGET_WIDTH /2f - 200;
    private static final float MINIMAP_LAYOUT_POSITION_Y = WindowHandler.TARGET_HEIGHT /2f - 200;
    public final static float MINIMAP_POSITION_X = WindowHandler.TARGET_WIDTH /2f - (BigMinimapRoom.DIMENSIONS * 4)/2f;
    public final static float MINIMAP_POSITION_Y = WindowHandler.TARGET_HEIGHT /2f - (BigMinimapRoom.DIMENSIONS * 4)/2f;
    private final Map<Room, BigMinimapRoom> minimapRooms;
    private final CommonImage minimapLayout;
    private final Vector2f dragPosition = new Vector2f();
    private boolean dragging;
    private boolean show;
    public BigMinimapRenderer(final List<Room> rooms, final Room startRoom){
        this.minimapLayout = new CommonImage("ingame/minimap_layout.tga", new Vector2f(MINIMAP_LAYOUT_POSITION_X, MINIMAP_LAYOUT_POSITION_Y), 4f, 4);
        this.minimapRooms = createMinimap(rooms, startRoom);
    }
    private BigMinimapRoom createMapRoom(RoomDirection direction, Room neighbour, BigMinimapRoom minimapRoomParent) {
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
        return new BigMinimapRoom(x, y, neighbour);
    }
    private Map<Room, BigMinimapRoom> createMinimap(final List<Room> rooms, final Room startRoom) {
        final Queue<Room> roomsToVisit = new ArrayDeque<>(rooms.size());
        final Map<Room, BigMinimapRoom> minimapRooms = new HashMap<>(rooms.size());

        final BigMinimapRoom startMinimapRoom = new BigMinimapRoom(0, 0, startRoom);

        roomsToVisit.add(startRoom);
        minimapRooms.put(startRoom, startMinimapRoom);

        while (!roomsToVisit.isEmpty()) {
            final Room room = roomsToVisit.poll();
            final BigMinimapRoom minimapRoomParent = minimapRooms.get(room);
            room.getNeighboursMap().forEach((direction, neighbour) -> {
                if (!minimapRooms.containsKey(neighbour)) {
                    final BigMinimapRoom minimapRoom = createMapRoom(direction, neighbour, minimapRoomParent);
                    minimapRooms.put(neighbour, minimapRoom);
                    roomsToVisit.add(neighbour);

                    minimapRoomParent.addConnection(direction, minimapRoom);
                    minimapRoom.addConnection(direction.getOpposite(), minimapRoomParent);
                }
            });
        }
        return minimapRooms;
    }



    @Override
    public void visit(final Room room) {
        final BigMinimapRoom currentRoom = minimapRooms.get(room);
        if (currentRoom == null) {
            LOGGER.error("Room {} is not present in the minimap", room);
            return;
        }
        currentRoom.visit();
        currentRoom.setRenderType(BigMinimapRenderType.CURRENT);

        final Queue<BigMinimapRoom> roomsToVisit = new ArrayDeque<>(minimapRooms.size());
        final Set<BigMinimapRoom> visited = new HashSet<>(minimapRooms.size());
        roomsToVisit.add(currentRoom);

        while (!roomsToVisit.isEmpty()) {
            final BigMinimapRoom r = roomsToVisit.poll();
            visited.add(r);
            r.getNeighbours().forEach((direction, neighbour) -> {
                if (!visited.contains(neighbour)) {
                    final BigMinimapRenderType nextType = switch (r.getRenderType()){
                        case CURRENT, VISITED -> {
                            if (neighbour.isVisited()) {
                                yield BigMinimapRenderType.VISITED;
                            } else {
                                yield BigMinimapRenderType.UNVISITED;
                            }
                        }
                        case UNVISITED, UNSEEN -> BigMinimapRenderType.UNSEEN;
                    };
                    LOGGER.info("Setting room: {}, type {}", neighbour, nextType);
                    neighbour.setRenderType(nextType);
                    roomsToVisit.add(neighbour);
                }
            });
        }
        minimapRooms.values().forEach(r -> {
            r.setPosition(currentRoom.getX(), currentRoom.getY());
        });
    }

    @Override
    public void update(float dt, float mouseX, float mouseY) {
        minimapRooms.values().forEach(minimapRoom -> minimapRoom.update(dt));
        if (dragging) {
            final float dx = mouseX - dragPosition.x;
            final float dy = mouseY - dragPosition.y;
            minimapRooms.values().forEach(minimapRoom -> minimapRoom.translate(dx, dy));
            dragPosition.set(mouseX, mouseY);
        }
    }

    @Override
    public List<Renderable> toRender() {
        if (!show) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>(minimapRooms.size() + 1);
        renderables.add(minimapLayout);
        renderables.addAll(minimapRooms.values());
        return renderables;
    }

    @Override
    public void mouseReleased(int button, float mouseX, float mouseY) {
        dragging = false;
    }

    @Override
    public void mousePressed(int button, float mouseX, float mouseY) {
        dragPosition.set(mouseX, mouseY);
        dragging = true;
    }

    @Override
    public void keyPressed(int key) {
        if (key == GLFW.GLFW_KEY_TAB){
            show = !show;
        }
        if (show) {
            CursorService.setCursor(CursorType.DEFAULT);
        } else {
            CursorService.setCursor(CursorType.CROSSHAIR);
        }
    }

    @Override
    public void keyReleased(int key) {
    }

    public boolean isActive() {
        return show;
    }
}
