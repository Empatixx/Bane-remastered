package cz.empatix.hud.service.image;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.render.shaderbindings.HudCommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class AlternativeViewImage implements HUDElement {
    private final Vector2i dimensions;
    private final RenderScene scene;
    private final int zDepth;
    private final Animation animation;
    private final Vector2f position;
    public AlternativeViewImage(final String texturePath, final Vector2f position, final float scale, final int zDepth, final RenderScene scene) {
        final TextureAtlas.Entry texture = AssetsService.getTexture(texturePath);
        this.dimensions = new Vector2i(texture.width(), texture.height());
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;

        this.zDepth = zDepth;
        this.scene = scene;
        this.position = position;

        this.animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0,1, dimensions.x(), dimensions.y(), -1)
                .texture(texture)
                .build();

    }
    public AlternativeViewImage(final String texturePath, final Vector2f position, final float scale, final int zDepth, final RenderScene scene, final Matrix4f cameraMatrix,
                                final int frames, final int delayBetweenAnimations) {
        final TextureAtlas.Entry texture = AssetsService.getTexture(texturePath);
        this.dimensions = new Vector2i(texture.width(), texture.height());
        this.dimensions.x *= scale / frames;
        this.dimensions.y *= scale;

        this.zDepth = zDepth;
        this.scene = scene;
        this.position = position;

        this.animation = Animation.builder()
                .setTotal(1, frames)
                .addState(AnimationState.IDLE, 0,frames, dimensions.x(), dimensions.y(), delayBetweenAnimations)
                .texture(texture)
                .build();
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
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
    public void update(double dt) {
        animation.update();
    }
    public int getWidth() {
        return dimensions.x;
    }
    public int getHeight() {
        return dimensions.y;
    }

    @Override
    public RenderScene renderScene() {
        return scene;
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public Vector2f getPosition() {
        return position;
    }

    public float getX() {
        return position.x();
    }
    public float getY() {
        return position.y();
    }
}
