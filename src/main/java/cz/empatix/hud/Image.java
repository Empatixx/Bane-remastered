package cz.empatix.hud;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import cz.empatix.render.domain.Vertex;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;

public class Image implements HUDElement {
    private final TextureAtlas.Entry texture;
    private final Vector2i dimensions;
    private final Vector2f position;

    public Image(final String textureName, final Vector2f position, final int width, final int height) {
        this(textureName, position, width, height, true);
    }

    public Image(final String textureName, final Vector2f position, final int width, final int height, final boolean centered) {
        if (centered) {
            position.sub(new Vector2f(width, height).div(2f));
        }
        this.texture = AssetsService.getTexture(textureName);
        this.dimensions = new Vector2i(width, height);
        this.position = new Vector2f(position);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                Vertex.builder()
                        .position(position)
                        .posDimensions(dimensions.x(), dimensions.y())
                        .texture(texture)
                        .buildQuad(),
                texture.textureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
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
