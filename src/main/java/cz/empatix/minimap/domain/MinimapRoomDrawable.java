package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.MiniMapBinding;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4i;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class MinimapRoomDrawable implements Drawable {
    private final static int TWEEN = 10;
    private final static float ALPHA_DECREASE = 0.25f;
    public final static int DIMENSIONS = 16;
    private final static int SCALE = 1;
    private boolean visited;
    private boolean current;
    private final Animation animation;
    // used for shader
    private final Vector2f normSpriteSize;
    private final Vector2f normSubImagePos;
    private final Vector4i connections; // left, right, top, bottom, 1 if connected, 0 if not
    private final float randomOffset = ThreadLocalRandom.current().nextFloat(10);
    private final InterpolatedPosition transform;
    private int roomsAwayFromCenter;

    public MinimapRoomDrawable(final MinimapRoom room) {
        final TextureAtlas.Entry entry = AssetsService.getTexture("ingame/minimap_icons.tga");
        this.animation = Animation.builder()
                .addState(AnimationState.IDLE, 0, 5, DIMENSIONS, DIMENSIONS, -1)
                .setTotal(1, 5)
                .texture(entry)
                .scale(SCALE)
                .build();
        this.animation.setCurrentFrame(getTextureFrame(room.getRoomType()));
        this.normSpriteSize = new Vector2f(16f / entry.atlasWidth(), 16f / entry.atlasHeight());
        this.normSubImagePos = entry.normalizedStartPositions();
        this.connections = getConnections(room.getRoomDirections());
        this.transform = new InterpolatedPosition();
    }

    private Vector4i getConnections(final List<RoomDirection> roomDirections) {
        final Vector4i connections = new Vector4i();
        roomDirections.forEach(direction -> {
            switch (direction) {
                case LEFT -> connections.x = 1;
                case RIGHT -> connections.y = 1;
                case TOP -> connections.z = 1;
                case BOTTOM -> connections.w = 1;
            }
        });
        return connections;
    }

    private int getTextureFrame(final RoomType type) {
        return switch (type) {
            case START -> 0;
            case NORMAL -> 1;
            case TREASURE -> 2;
            case SHOP -> 3;
            case BOSS -> 4;
            default -> throw new IllegalStateException("Unknown room type: " + type);
        };
    }

    @Override
    public RenderData renderData() {
        return new RenderData(animation.getVertices(transform.getInterpolatedPosition()),
                animation.getTextureId(),
                new MiniMapBinding(Camera.getOriginWithoutZoomProjection(), normSubImagePos, normSpriteSize,
                        System.currentTimeMillis() % 1000000 / 1000f, randomOffset, visited, current,
                        calculateAlpha(), animation.getCurrentTextureIndex(), connections));
    }


    private float calculateAlpha() {
        return 1 - ALPHA_DECREASE * (float) Math.pow(roomsAwayFromCenter, 1.25) - (roomsAwayFromCenter > 1 && !visited ? 0.35f : 0);
    }

    public void setRoomsAwayFromCenter(int roomsAwayFromCenter) {
        this.roomsAwayFromCenter = roomsAwayFromCenter;
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.HUD;
    }

    @Override
    public int getZIndex() {
        return 5;
    }

    @Override
    public boolean canRender() {
        return calculateAlpha() > 0;
    }
}
