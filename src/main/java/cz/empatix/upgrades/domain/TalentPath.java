package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

public class TalentPath implements Drawable {
    private Animation animation;
    private final Vector2f position;
    public TalentPath(final TreeType type, final boolean isLocked){
        position = new Vector2f();
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/talent_path.tga"))
                .setTotal(4,1)
                .addState(AnimationState.TYPE0, 0, 1, 8,15,-1)
                .addState(AnimationState.TYPE1, 1, 1, 8,15,-1)
                .addState(AnimationState.TYPE2, 2, 1, 8,15,-1)
                .addState(AnimationState.TYPE3, 3, 1, 8,15,-1)
                .scale(2f)
                .build();
        if (isLocked){
            animation.setState(AnimationState.TYPE3);
        } else {
            switch (type){
                case UTILITY -> animation.setState(AnimationState.TYPE0);
                case OFFENSIVE -> animation.setState(AnimationState.TYPE1);
                case DEFENSIVE -> animation.setState(AnimationState.TYPE2);
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
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }
    public void setPosition(final float x, final float y){
        position.set(x, y);
    }
}
