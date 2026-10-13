package cz.empatix.minimap.service;
import cz.empatix.core.Viewport;

import cz.empatix.sound.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.domain.AbstractMinimapRoom;
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
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private static final Logger LOGGER = LoggerFactory.getLogger(BigMinimapRenderer.class);
    private static final float MINIMAP_LAYOUT_POSITION_X = Viewport.TARGET_WIDTH /2f - 200;
    private static final float MINIMAP_LAYOUT_POSITION_Y = Viewport.TARGET_HEIGHT /2f - 200;
    public final static float MINIMAP_POSITION_X = Viewport.TARGET_WIDTH /2f - (BigMinimapRoom.DIMENSIONS * 4)/2f;
    public final static float MINIMAP_POSITION_Y = Viewport.TARGET_HEIGHT /2f - (BigMinimapRoom.DIMENSIONS * 4)/2f;
    private Map<Room, BigMinimapRoom> minimapRooms;
    private final CommonImage minimapLayout;
    private final Vector2f dragPosition = new Vector2f();
    private boolean dragging;
    private boolean show;
    public BigMinimapRenderer(){
        this.minimapLayout = new CommonImage("ingame/minimap_layout.tga", new Vector2f(MINIMAP_LAYOUT_POSITION_X, MINIMAP_LAYOUT_POSITION_Y), 4f, 4);
    }

    @Override
    public void apply(MapGenerationResult result) {
        minimapRooms = MinimapRoomFactory.createMinimap(result.rooms(), result.startRoom(), BigMinimapRoom::new);
        visit(result.startRoom());
        update(1, 0, 0);
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
    public void visitAll() {
        minimapRooms.values().forEach(BigMinimapRoom::visit);
        minimapRooms.values().stream()
                .filter(r -> r.getRenderType() != BigMinimapRenderType.CURRENT)
                .forEach(r -> r.setRenderType(BigMinimapRenderType.VISITED));
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
        if (!show) {
            return;
        }
        dragPosition.set(mouseX, mouseY);
        dragging = true;
    }

    @Override
    public void activate() {
        show = true;
        CursorService.setCursor(CursorType.DEFAULT);
        AUDIO_SERVICE.play(SoundEffect.SHOW_MAP);
    }

    @Override
    public void deactivate() {
        show = false;
        CursorService.setCursor(CursorType.CROSSHAIR);
    }


    public boolean isActive() {
        return show;
    }
}
