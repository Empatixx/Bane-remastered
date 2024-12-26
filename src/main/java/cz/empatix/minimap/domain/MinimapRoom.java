package cz.empatix.minimap.domain;

import cz.empatix.minimap.service.SmallMinimapRenderer;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.MiniMapBinding;
import cz.empatix.rooms.service.Room;

public class MinimapRoom extends AbstractMinimapRoom<MinimapRoom> implements Drawable {
    private final static float ALPHA_DECREASE = 0.25f;
    public final static int DIMENSIONS = 16;
    private final static int SCALE = 1;
    private int roomsAwayFromCenter;

    public MinimapRoom(final Room room) {
        super(room.getRoomType(), SCALE);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(animation.getVertices(transform.getInterpolatedPosition(), getZIndex()),
                animation.getTextureId(),
                new MiniMapBinding(Camera.getProjectionMatrix(), normSubImagePos, normSpriteSize,
                        System.currentTimeMillis() % 1000000 / 1000f, randomOffset, visited, roomsAwayFromCenter == 0,
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


    public int getRoomsAwayFromCenter() {
        return roomsAwayFromCenter;
    }

    public void update(final float dt) {
        transform.update(dt);
    }
    public void setPosition(final int centerX, final int centerY) {
        final float nextX = SmallMinimapRenderer.MINIMAP_POSITION_X + (this.x - centerX) * DIMENSIONS * SCALE;
        final float nextY = SmallMinimapRenderer.MINIMAP_POSITION_Y + (this.y - centerY) * DIMENSIONS * SCALE;
        transform.setInterpolatedPosition(nextX, nextY);
    }


}
