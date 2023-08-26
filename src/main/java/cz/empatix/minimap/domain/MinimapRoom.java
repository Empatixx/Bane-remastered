package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.physics.domain.transform.InterpolatedTransform;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.MiniMapBinding;
import cz.empatix.rooms.service.*;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4i;

import java.util.concurrent.ThreadLocalRandom;

public class MinimapRoom implements Drawable {
    private final static int TWEEN = 10;
    private final static float ALPHA_DECREASE = 0.25f;
    public final static int DIMENSIONS = 16;
    private final static int SCALE = 1;
    private final int x;
    private final int y;
    private boolean visited;
    private boolean current;
    private final Animation animation;
    // used for shader
    private final Vector2f normSpriteSize;
    private final Vector2f normSubImagePos;
    private final Vector4i connections; // left, right, top, bottom, 1 if connected, 0 if not
    private final float randomOffset = ThreadLocalRandom.current().nextFloat(10);
    private final InterpolatedTransform transform;
    private int roomsAwayFromCenter;

    public MinimapRoom(final int x, final int y, final Room room) {
        this.x = x;
        this.y = y;
        this.visited = false;
        final TextureAtlas.Entry entry = AssetsService.getTexture("ingame/minimap_icons.tga");
        this.animation = Animation.builder()
                .addState(AnimationState.IDLE, 0, 5, DIMENSIONS, DIMENSIONS, -1)
                .setTotal(1, 5)
                .texture(entry)
                .scale(SCALE)
                .build();
        this.animation.setCurrentFrame(getTextureFrame(room));
        this.normSpriteSize = new Vector2f(16f / entry.atlasWidth(), 16f / entry.atlasHeight());
        this.normSubImagePos = entry.normalizedStartPositions();
        this.connections = new Vector4i(0, 0, 0, 0);
        this.transform = new InterpolatedTransform();
    }

    public void setConnection(final RoomDirection direction) {
        switch (direction) {
            case LEFT -> connections.x = 1;
            case RIGHT -> connections.y = 1;
            case TOP -> connections.z = 1;
            case BOTTOM -> connections.w = 1;
        }
    }

    private int getTextureFrame(final Room r) {
        return switch (r.getRoomType()){
            case START -> 0;
            case NORMAL -> 1;
            case TREASURE -> 2;
            case SHOP -> 3;
            case BOSS -> 4;
            default -> throw new IllegalStateException("Unknown room type: " + r.getRoomType());
        };
    }

    public void visit() {
        visited = true;
    }

    public void setCurrent(final boolean current) {
        this.current = current;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public void update(final MinimapRoom room, final float dt) {
        transform.update(dt);
        setPositionToCenterRoom(dt, room.getX(), room.getY());
    }

    @Override
    public RenderData renderData() {
        return new RenderData(animation.getVertices(transform.getTranslationToRender()),
                animation.getTextureId(),
                new MiniMapBinding(Camera.getOriginWithoutZoomProjection(), normSubImagePos, normSpriteSize,
                        System.currentTimeMillis() % 1000000 / 1000f, randomOffset, visited, current,
                        calculateAlpha(), animation.getCurrentTextureIndex(), connections));
    }

    private float calculateAlpha() {
        return 1 - ALPHA_DECREASE * (float) Math.pow(roomsAwayFromCenter, 1.25) - (roomsAwayFromCenter > 1 && !visited ? 0.35f : 0);
    }

    public void setPositionToCenterRoom(final float dt, final int x, final int y) {
        final float nextX = MinimapService.MINIMAP_POSITION_X + (this.x - x) * DIMENSIONS * SCALE;
        final float nextY = MinimapService.MINIMAP_POSITION_Y + (this.y - y) * DIMENSIONS * SCALE;

        final float currentX = transform.getTranslation().x();
        final float currentY = transform.getTranslation().y();

        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }

        final float newX = (nextX - currentX) * tween + currentX;
        final float newY = (nextY - currentY) * tween + currentY;

        transform.setTranslation(newX, newY);
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
        return true;
    }
}
