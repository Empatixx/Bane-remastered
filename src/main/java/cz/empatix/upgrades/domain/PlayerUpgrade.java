package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class PlayerUpgrade implements Drawable {
    private Animation animation;
    private final Vector2f position;
    private boolean locked;
    public PlayerUpgrade(final QualityLevel level, final boolean isLocked){
        position = new Vector2f();
        locked = isLocked;
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/talent_frame.tga"))
                .setTotal(6,1)
                .addState(AnimationState.TYPE0, 0, 1, 32,32,-1)
                .addState(AnimationState.TYPE1, 1, 1, 32,32,-1)
                .addState(AnimationState.TYPE2, 2, 1, 32,32,-1)
                .addState(AnimationState.TYPE3, 3, 1, 32,32,-1)
                .addState(AnimationState.TYPE4, 4, 1, 32,32,-1)
                .addState(AnimationState.TYPE5, 5, 1, 32,32,-1)
                .scale(2f)
                .build();
        if (isLocked){
            switch (level){
                case COMMON -> animation.setState(AnimationState.TYPE1);
                case RARE -> animation.setState(AnimationState.TYPE3);
                case EPIC -> animation.setState(AnimationState.TYPE5);
            }
        } else {
            switch (level){
                case COMMON -> animation.setState(AnimationState.TYPE0);
                case RARE -> animation.setState(AnimationState.TYPE2);
                case EPIC -> animation.setState(AnimationState.TYPE4);
            }
        }
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getOriginWithoutZoomProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.OVERLAYS;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return true;
    }
    public void setPosition(final float x, final float y){
        position.set(x, y);
    }

    public boolean isLocked() {
        return locked;
    }
}
