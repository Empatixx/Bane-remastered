package cz.empatix.keyboard.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class KeyboardImage implements HUDElement {
    private static final int SCALE = 3;
    private static final int WIDTH = 11 * SCALE;
    private static final int HEIGHT = 12 * SCALE;
    private final Vector2f position;
    private final Animation animation;
    public KeyboardImage(final KeyboardType type){
        this.animation = Animation.builder()
                .setTotal(1,2)
                .addState(AnimationState.IDLE, 0, 2, WIDTH, HEIGHT,200)
                .texture(AssetsService.getTexture(type.getTexturePath()))
                .build();
        animation.setState(AnimationState.IDLE);
        this.position = new Vector2f();
    }
    public void setPosition(final float x, final float y){
        this.position.set(x,y);
    }
    @Override
    public void update(double dt) {
        animation.update();
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
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, WIDTH, HEIGHT);
    }

    @Override
    public RenderScene renderScene(){
        return RenderScene.SCENE;
    }

    public Vector2f getPosition() {
        return position;
    }
    public int getWidth(){
        return WIDTH;
    }
    public int getHeight(){
        return HEIGHT;
    }
}
