package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.InterpolatedPosition;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4i;

import java.util.EnumMap;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;

public abstract class AbstractMinimapRoom<E> {
    public final static int DIMENSIONS = 16;

    protected int x;
    protected int y;
    protected boolean visited;
    protected final Animation animation;
    protected final InterpolatedPosition transform;
    private final Map<RoomDirection, E> neighbours = new EnumMap<>(RoomDirection.class);
    // used for shader
    protected final Vector2f normSpriteSize;
    protected final Vector2f normSubImagePos;
    protected final Vector4i connections; // left, right, top, bottom, 1 if connected, 0 if not
    protected final float randomOffset = ThreadLocalRandom.current().nextFloat(10);
    public AbstractMinimapRoom(final RoomType roomType,
                               final int scale) {
        final TextureAtlas.Entry entry = AssetsService.getTexture("ingame/minimap_icons.tga");
        this.animation = Animation.builder()
                .addState(AnimationState.IDLE, 0, 6, DIMENSIONS, DIMENSIONS, -1)
                .setTotal(1, 6)
                .texture(entry)
                .scale(scale)
                .build();
        this.animation.setCurrentFrame(getTextureFrame(roomType));

        this.normSpriteSize = new Vector2f(16f / entry.atlasWidth(), 16f / entry.atlasHeight());
        this.normSubImagePos = entry.normalizedStartPositions();
        this.connections = new Vector4i(0, 0, 0, 0);
        this.transform = new InterpolatedPosition(1/5f);

    }
    protected int getTextureFrame(final RoomType type) {
        return switch (type) {
            case START -> 0;
            case NORMAL -> 1;
            case TREASURE -> 2;
            case SHOP -> 3;
            case BOSS -> 4;
            case SACRIFICE -> 5;
            default -> throw new IllegalStateException("Unknown room type: " + type);
        };
    }

    public void addConnection(RoomDirection direction, E minimapRoom) {
        switch (direction) {
            case LEFT -> connections.x = 1;
            case RIGHT -> connections.y = 1;
            case TOP -> connections.z = 1;
            case BOTTOM -> connections.w = 1;
        }
        neighbours.put(direction, minimapRoom);
    }

    public void visit() {
        visited = true;
    }
    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }
    public boolean isVisited() {
        return visited;
    }

    public Map<RoomDirection, E> getNeighbours() {
        return neighbours;
    }
    public void setMapPosition(final int x, final int y) {
        this.x = x;
        this.y = y;
    }
}
