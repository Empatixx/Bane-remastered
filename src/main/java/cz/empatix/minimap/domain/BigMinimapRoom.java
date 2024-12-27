package cz.empatix.minimap.domain;

import cz.empatix.minimap.service.BigMinimapRenderer;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.BigMiniMapBinding;
import cz.empatix.rooms.service.Room;

public class BigMinimapRoom extends AbstractMinimapRoom<BigMinimapRoom> implements Drawable {
    public final static int DIMENSIONS = 16;
    private final static int SCALE = 4;
    private BigMinimapRenderType renderType;

    public BigMinimapRoom(final Room room) {
        super(room.getRoomType(), SCALE);
    }
    @Override
    public RenderData renderData() {
        return new RenderData(animation.getVertices(transform.getInterpolatedPosition(), getZIndex()),
                animation.getTextureId(),
                new BigMiniMapBinding(Camera.getInstance().getProjectionMatrix(), normSubImagePos, normSpriteSize,
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

    public void translate(float dx, float dy) {
        transform.setInterpolatedPosition(transform.getX() + dx, transform.getY() + dy);
    }
}
