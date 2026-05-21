package cz.empatix.upgrades.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.HudCommonBinding;
import org.joml.Vector2f;

import java.util.function.Supplier;

public class TalentPath implements Drawable {
    private Animation animation;
    private final Vector2f position;
    private final Supplier<Boolean> isLocked;
    private final TreeType type;
    public TalentPath(final TreeType type, final Supplier<Boolean> isLocked){
        position = new Vector2f();
        this.type = type;
        animation = Animation.builder()
                .texture(AssetsService.getTexture("ingame/talent_path.tga"))
                .setTotal(4,1)
                .addState(AnimationState.TYPE0, 0, 1, 8,15,-1)
                .addState(AnimationState.TYPE1, 1, 1, 8,15,-1)
                .addState(AnimationState.TYPE2, 2, 1, 8,15,-1)
                .addState(AnimationState.TYPE3, 3, 1, 8,15,-1)
                .scale(2f)
                .build();
        this.isLocked = isLocked;
        setImage();
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
    public void update(final double dt){
        setImage();
    }
    private void setImage(){
        if (isLocked.get()){
            animation.setState(AnimationState.TYPE3);
        } else {
            switch (type){
                case UTILITY -> animation.setState(AnimationState.TYPE0);
                case OFFENSIVE -> animation.setState(AnimationState.TYPE1);
                case DEFENSIVE -> animation.setState(AnimationState.TYPE2);
            }
        }
    }
}
