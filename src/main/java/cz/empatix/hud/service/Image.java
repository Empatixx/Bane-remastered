package cz.empatix.hud.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class Image implements HUDElement {
    private final static RenderScene DEFAULT_SCENE = RenderScene.HUD;
    private final static int DEFAULT_Z_DEPTH = 1;
    private final TextureAtlas.Entry texture;
    private final Vector2i dimensions;
    private final Vector2f position;
    private final RenderScene scene;
    private final int zDepth;

    public Image(final String textureName, final Vector2f position, final int width, final int height) {
        this.dimensions = new Vector2i(width, height);
        this.texture = AssetsService.getTexture(textureName);
        this.position = new Vector2f(position);
        this.zDepth = DEFAULT_Z_DEPTH;
        this.scene = DEFAULT_SCENE;
    }

    public Image(final String textureName, final Vector2f position, final float scale) {
        this(AssetsService.getTexture(textureName), position, scale, DEFAULT_Z_DEPTH);
    }

    public Image(final String textureName, final Vector2f position, final float scale, final int zDepth) {
        this(AssetsService.getTexture(textureName), position, scale, zDepth);
    }

    public Image(final TextureAtlas.Entry texture, final Vector2f position, final float scale) {
        this(texture, position, scale, DEFAULT_Z_DEPTH);
    }

    public Image(final TextureAtlas.Entry texture, final Vector2f position, final float scale, final int zDepth) {
        this(texture, position, scale, zDepth, DEFAULT_SCENE);
    }

    public Image(final String texture, final Vector2f position, final float scale, final int zDepth, final RenderScene scene) {
        this(AssetsService.getTexture(texture), position, scale, zDepth, scene);
    }

    public Image(final TextureAtlas.Entry texture, final Vector2f position, final float scale, final int zDepth, final RenderScene scene) {
        this.dimensions = new Vector2i(texture.width(), texture.height());
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;
        this.texture = texture;
        this.position = position;
        this.zDepth = zDepth;
        this.scene = scene;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                Vertex.builder()
                        .position(position)
                        .posDimensions(dimensions.x(), dimensions.y())
                        .texture(texture)
                        .offset(dimensions.x(), dimensions.y())
                        .buildQuad(),
                texture.textureId(),
                new CommonBinding(Camera.getOriginWithoutZoomProjection())
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
    public float getX() {
        return position.x();
    }
    public float getY() {
        return position.y();
    }
}
