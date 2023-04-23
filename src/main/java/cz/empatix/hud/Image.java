package cz.empatix.hud;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.service.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.render.domain.Vertex;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class Image implements HUDElement {
    private final TextureAtlas.Entry texture;
    private final Vector2i dimensions;
    private final Vector2f position;

    public Image(final String textureName, final Vector2f position, final int width, final int height) {
        this(textureName, position, width, height, true, 1);
    }
    public Image(final String textureName, final Vector2f position, final int width, final int height, final float scale) {
        this(textureName, position, width, height, true, scale);
    }
    public Image(final String textureName, final Vector2f position, final int width, final int height, final boolean centered) {
        this(textureName, position, width, height, centered, 1);
    }
    public Image(final String textureName, final Vector2f position, final int width, final int height, final boolean centered, final float scale) {
        this.dimensions = new Vector2i(width, height);
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;
        if (centered) {
            position.sub(new Vector2f(dimensions.x(), dimensions.y()).div(2f));
        }
        this.texture = AssetsService.getTexture(textureName);
        this.position = new Vector2f(position);
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
                new CommonBinding(Camera.getProjection()),
                RenderData.RenderDataType.STATIC
        );
    }

    @Override
    public boolean shouldRender() {
        return position.x() + dimensions.x() > Camera.getX() && position.x() < Camera.getTargetWidth() + Camera.getX() &&
                position.y() + dimensions.y() > Camera.getY() && position.y() < Camera.getTargetHeight() + Camera.getY();
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public void update(double dt) {

    }
}
