package cz.empatix.hud.service.image;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class AlternativeViewImage implements HUDElement {
    private final TextureAtlas.Entry texture;
    private final Vector2i dimensions;
    private final Vector2f position;
    private final RenderScene scene;
    private final int zDepth;
    private final Matrix4f cameraMatrix;

    public AlternativeViewImage(final String texture, final Vector2f position, final float scale, final int zDepth, final RenderScene scene, final Matrix4f cameraMatrix) {
        this.texture = AssetsService.getTexture(texture);
        this.dimensions = new Vector2i(this.texture.width(), this.texture.height());
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;
        this.cameraMatrix = cameraMatrix;
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
                new CommonBinding(cameraMatrix)
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
