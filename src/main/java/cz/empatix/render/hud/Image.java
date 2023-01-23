package cz.empatix.render.hud;

import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import org.joml.Vector2f;

public class Image implements HUDElement {
    private final Shader shader;
    private final int verticesVbo;
    private final int texturesVbo;
    private final int textureId;
    private final Vector2f position;
    public Image(final String textureName, final String metadataName, final Vector2f position) {
        this.shader = AssetsService.getShader("shader");
        final Metadata metadata = AssetsService.getMetadata(metadataName);
        texturesVbo = metadata.texturesVbos().get(0).get(0);
        verticesVbo = metadata.verticesVbo();
        textureId = AssetsService.getTexture(textureName);
        this.position = position.sub(metadata.width() / 2, metadata.height() / 2);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                verticesVbo,
                texturesVbo,
                textureId,
                shader,
                this::prepareShader
        );
    }
    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(position.x,position.y,0)
        );
    }

    @Override
    public boolean shouldRender() {
        return true;
    }

    @Override
    public void update(double dt) {

    }
}
