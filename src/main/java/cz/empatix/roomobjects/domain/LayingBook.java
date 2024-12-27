package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class LayingBook implements RoomObject{
    private final static float SCALE = 4f;
    private final static int DIMENSION_WIDTH = (int) (16 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (16 * SCALE);
    private final Animation animation;
    private final Vector2f position;

    public LayingBook() {
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.REVERSE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1, true)
                .texture(AssetsService.getTexture("ingame/laying_book.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        if (Math.random() > 0.5) {
            animation.setState(AnimationState.REVERSE);
        }
        position = new Vector2f();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void update(double dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
    }

    @Override
    public int getWidth() {
        return DIMENSION_WIDTH;
    }
    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }
}
