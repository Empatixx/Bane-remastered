package cz.empatix.render.background;

import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;

public class AnimatedBackground implements Background {
    private Animation animation;
    private final Shader shader;
    private final int texture;
    private final int verticesVbo;

    public AnimatedBackground(final String texture, final String metadata) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(texture);
        final Metadata meta = AssetsService.getMetadata(metadata);
        verticesVbo = meta.verticesVbo();
        animation = new Animation(meta.texturesVbos());
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                verticesVbo,
                animation.getCurrentTextureVbo(),
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
        animation.update();
    }
}
