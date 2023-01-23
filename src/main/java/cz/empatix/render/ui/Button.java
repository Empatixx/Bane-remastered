package cz.empatix.render.ui;

import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.IntersectionResolver;
import cz.empatix.render.Camera;
import cz.empatix.render.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import org.joml.Vector2f;

import java.util.List;

public class Button implements UIElement {
    private final Vector2f dimensions;
    private final Vector2f position;
    private final String text;
    private final int verticesVbo;
    private final int textureId;
    private final Animation animation;
    private final Shader shader;
    private boolean hover;

    public Button(final String text, final Vector2f position) {
        this.text = text;
        this.shader = AssetsService.getShader("shader");
        final Metadata metadata = AssetsService.getMetadata("button.json");
        verticesVbo = metadata.verticesVbo();
        animation = new Animation(metadata.texturesVbos());
        animation.setDelayBetweenAnimations(-1);
        textureId = AssetsService.getTexture("Menu/Menu_bar.tga");
        dimensions = new Vector2f(metadata.width(), metadata.height());
        this.position = position.sub(metadata.width() / 2, metadata.height() / 2);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                verticesVbo,
                animation.getCurrentTextureVbo(),
                textureId,
                shader,
                this::prepareShader
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(position.x,position.y,0));
    }

    @Override
    public boolean shouldRender() {
        return true;
    }

    @Override
    public void update(final double dt, final double mouseX, final double mouseY) {
        hover = IntersectionResolver.newInstance()
                .addShape(position, dimensions)
                .addShape((float) mouseX, (float) mouseY)
                .resolve();
        animation.setAnimationState(hover ? 1 : 0);
    }

    @Override
    public void mousePressed(final int button) {

    }

    @Override
    public void mouseReleased(final int button) {

    }

    @Override
    public void keyPressed(final int key) {

    }

    @Override
    public void keyReleased(final int key) {

    }

    @Override
    public void mouseScrolled(final double xoffset, final double yoffset) {

    }

    @Override
    public boolean isHovered() {
        return hover;
    }
}
