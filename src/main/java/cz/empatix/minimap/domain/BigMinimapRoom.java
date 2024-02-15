package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.service.BigMinimapRenderer;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.BigMiniMapBinding;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4i;

import java.util.EnumMap;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;

public class BigMinimapRoom implements Drawable {
    public final static int DIMENSIONS = 16;
    private final static int SCALE = 4;
    private final int x;
    private final int y;
    private boolean visited;
    private final Animation animation;
    private final InterpolatedPosition transform;
    private final Map<RoomDirection, BigMinimapRoom> neighbours = new EnumMap<>(RoomDirection.class);
    // used for shader
    private final Vector2f normSpriteSize;
    private final Vector2f normSubImagePos;
    private final Vector4i connections; // left, right, top, bottom, 1 if connected, 0 if not
    private final float randomOffset = ThreadLocalRandom.current().nextFloat(10);
    private BigMinimapRenderType renderType;

    public BigMinimapRoom(final int x,
                          final int y,
                          final Room room) {
        this.x = x;
        this.y = y;

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
        this.connections = new Vector4i(0, 0, 0, 0);
        this.transform = new InterpolatedPosition(1/5f);

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
                new BigMiniMapBinding(Camera.getOriginWithoutZoomProjection(), normSubImagePos, normSpriteSize,
                        System.currentTimeMillis() % 1000000 / 1000f, randomOffset, visited, renderType == BigMinimapRenderType.CURRENT,
                        calculateAlpha(), animation.getCurrentTextureIndex(), connections));
    }


    private float calculateAlpha() {
        return renderType.alpha();
    }

    public void setRenderType(final BigMinimapRenderType renderType) {
        this.renderType = renderType;
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

    public void visit() {
        visited = true;
    }

    public BigMinimapRenderType getRenderType() {
        return renderType;
    }
    public void update(final float dt) {
        transform.update(dt);
    }
    public void setPosition(final int centerX, final int centerY) {
        final float nextX = BigMinimapRenderer.MINIMAP_POSITION_X + (this.x - centerX) * DIMENSIONS * SCALE;
        final float nextY = BigMinimapRenderer.MINIMAP_POSITION_Y + (this.y - centerY) * DIMENSIONS * SCALE;
        transform.setInterpolatedPosition(nextX, nextY);
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public void addConnection(RoomDirection direction, BigMinimapRoom minimapRoom) {
        switch (direction) {
            case LEFT -> connections.x = 1;
            case RIGHT -> connections.y = 1;
            case TOP -> connections.z = 1;
            case BOTTOM -> connections.w = 1;
        }
        neighbours.put(direction, minimapRoom);
    }

    public Map<RoomDirection, BigMinimapRoom> getNeighbours() {
        return neighbours;
    }

    public void translate(float dx, float dy) {
        transform.setInterpolatedPosition(transform.getX() + dx, transform.getY() + dy);
    }
    public boolean isVisited() {
        return visited;
    }
}
