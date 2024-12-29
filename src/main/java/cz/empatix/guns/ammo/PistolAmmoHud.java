package cz.empatix.guns.ammo;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.HudCommonBinding;
import org.joml.Vector2f;

public class PistolAmmoHud implements Drawable {
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int)(16 *  SCALE);
    private final static int DIMENSION_HEIGHT = (int)(16 *  SCALE);
    private final Animation animation;
    private final Vector2f position;
    public PistolAmmoHud() {
        position = new Vector2f();
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/ammo_hud.tga"))
                .setTotal(2, 2)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.EMPTY, 1, 2, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .build();
        animation.setState(AnimationState.IDLE);
    }
    public void setVisible(final boolean visible) {
        if (visible) {
            animation.setState(AnimationState.IDLE);
        } else {
            animation.setState(AnimationState.EMPTY);
        }
    }
    public void update(final float dt){
        if (animation.getCurrentState() == AnimationState.EMPTY && animation.isLastFrame()){
            return;
        }
        animation.update();
    }
    public void setPosition(final float x, final float y){
        position.set(x, y);
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new HudCommonBinding()
        );
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
