package cz.empatix.render.hud;

import cz.empatix.common.assets.domain.ImageTexture;
import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.Vertex;
import org.joml.Vector2f;

public class Image implements HUDElement {
    private final Shader shader;
    private final ImageTexture texture;
    private final Vector2f position;

    public Image(final String textureName, final Vector2f position) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(textureName);
        this.position = new Vector2f(position.add(texture.width()/2f, texture.height()/2f));
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                Vertex.builder()
                        .position(position)
                        .posDimensions(texture.width(), texture.height())
                        .textureDimensions(1,1)
                        .texturePosition(0,0)
                        .buildQuad(),
                texture.textureId(),
                shader,
                this::prepareShader,
                RenderData.RenderDataType.STATIC
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("projection", Camera.getProjection());
    }

    @Override
    public boolean shouldRender() {
        return position.x() + texture.width() > Camera.getX() && position.x() < Camera.getTargetWidth() + Camera.getX() &&
                position.y() + texture.height() > Camera.getY() && position.y() < Camera.getTargetHeight() + Camera.getY();
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public void update(double dt) {

    }
}
