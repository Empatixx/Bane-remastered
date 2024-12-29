package cz.empatix.hud.service.image;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.HudCommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class AnimatedImage implements HUDElement {
    private final static RenderScene DEFAULT_SCENE = RenderScene.HUD;
    private final static int DEFAULT_Z_INDEX = 1;
    private final static float DEFAULT_SCALE = 1f;
    private final TextureAtlas.Entry texture;
    private final Animation animation;
    private final Vector2f position;
    private final Vector2i dimensions;
    private final RenderScene scene;
    private final int zDepth;

    public AnimatedImage(final String textureName, final Vector2f position, final int frames, final boolean centered) {
        this(textureName, position, DEFAULT_SCALE, DEFAULT_Z_INDEX, DEFAULT_SCENE, frames, centered);
    }

    public AnimatedImage(final String textureName, final Vector2f position, final float scale, final int zDepth, final RenderScene scene, final int frames) {
        this(textureName, position, scale, zDepth, scene, frames, false);
    }

    public AnimatedImage(final String textureName, final Vector2f position, final float scale, final int zDepth, final RenderScene scene, final int frames, final boolean centered) {
        texture = AssetsService.getTexture(textureName);
        this.dimensions = new Vector2i(texture.width() / frames, texture.height());
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;

        if (centered) {
            position.sub(new Vector2f(dimensions.x(), dimensions.y()).div(2f));
        }
        this.animation = Animation.builder()
                .setTotal(1, frames)
                .addState(AnimationState.STATIC, 0, frames, dimensions.x(), dimensions.y(), 100)
                .texture(texture)
                .build();
        this.animation.setState(AnimationState.STATIC);
        this.position = position;
        this.zDepth = zDepth;
        this.scene = scene;
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public Vector2f getPosition() {
        return position;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                texture.textureId(),
                new HudCommonBinding()
        );
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return zDepth;
    }

    @Override
    public RenderScene renderScene(){
        return scene;
    }

    @Override
    public void update(double dt) {
        animation.update();
    }

    public int getWidth() {
        return dimensions.x;
    }
}
