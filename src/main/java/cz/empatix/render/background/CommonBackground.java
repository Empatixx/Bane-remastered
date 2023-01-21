package cz.empatix.render.background;

import cz.empatix.assets.domain.Metadata;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;

import static org.lwjgl.opengl.GL20.*;

public class CommonBackground implements Background {
    private final Shader shader;
    private final int texture;
    private final int verticesVbo;
    private final int texturesVbo;

    public CommonBackground(final String texture) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(texture);
        final Metadata metadata = AssetsService.getMetadata("background-overlay.json");
        verticesVbo = metadata.verticesVbo();
        texturesVbo = metadata.getTexturesVbos(0).get(0);
    }
    @Override
    public RenderData renderData() {
        return new RenderData(
                verticesVbo,
                texturesVbo,
                texture,
                shader,
                this::prepareShader
        );
    }
    private void prepareShader(final Shader shader){
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection());
    }
    @Override
    public boolean shouldRender() {
        return true;
    }

    public void update(){
    }
}
