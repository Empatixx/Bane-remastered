package cz.empatix.render.background;

import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.renderdata.CommonRenderData;
import cz.empatix.render.domain.Shader;

public class AnimatedBackground implements Background {
    private final Animation animation;
    private final Shader shader;
    private final int texture;
    private final Metadata metadata;

    public AnimatedBackground(final String texture, final String metadata) {
        this.shader = AssetsService.getShader("shader");
        this.texture = AssetsService.getTexture(texture);
        this.metadata = AssetsService.getMetadata(metadata);
        animation = new Animation(this.metadata.sprites());
    }

    @Override
    public CommonRenderData renderData() {
        return new CommonRenderData(
                metadata.vao(),
                texture,
                shader,
                this::prepareShader
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(0, 0, 0f));
        shader.setUniform("totalColumns", metadata.totalColumns());
        shader.setUniform("totalRows", metadata.totalRows());
        shader.setUniform("currentRow", animation.getCurrentTextureRow());
        shader.setUniform("currentColumn", animation.getCurrentTextureColumn());

    }

    @Override
    public boolean shouldRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return -1;
    }

    public void update() {
        animation.update();
    }
}
