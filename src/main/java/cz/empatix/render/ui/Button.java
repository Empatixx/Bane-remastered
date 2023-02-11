package cz.empatix.render.ui;

import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.physics.IntersectionResolver;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.hud.Text;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.util.List;

public class Button implements UIElement {
    private final static String DEFAULT_TEXTURE = "Menu/Menu_bar.tga";
    private final static Vector3i DEFAULT_COLOR = new Vector3i(222,112,38);
    private final static int WIDTH = 200;

    private final Vector2f dimensions;
    private final Vector2f position;
    private final Text text;
    private final Animation animation;
    private final Shader shader;
    private final int textureId;
    private boolean hover;
    private Runnable onClick;

    public Button(final String text, final Vector2f position) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR);
    }
    public Button(final String text,
                  final Vector2f position,
                  final String texture,
                  final Vector3i rgb){
        this.shader = AssetsService.getShader("shader");
        animation = Animation.builder()
                        .textureDimensions(1,2)
                                .addAnimationState(AnimationState.STATIC, 0, 1, )
        animation.setDelayBetweenAnimations(-1);
        textureId = AssetsService.getTexture("Menu/Menu_bar.tga");
        dimensions = new Vector2f(metadata.width(), metadata.height());
        this.position = position.sub(metadata.width() / 2, metadata.height() / 2);

        final Vector2f textPosition = new Vector2f(position)
                .add(0, metadata.height() / 2);
        this.text = new Text(text, "PixelFJVerdana12pt.ttf", textPosition, 1);
        this.text.setCenter(position.x,position.x+metadata.width());
        this.text.setColorRGB(rgb.x(), rgb.y(), rgb.z());
    }

    @Override
    public CommonRenderData renderData() {
        return new CommonRenderData(
                metadata.vao(),
                textureId,
                shader,
                this::prepareShader
        );
    }

    public void setOnClick(final Runnable onClick) {
        this.onClick = onClick;
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(position.x, position.y, 0f));
        shader.setUniform("totalColumns", metadata.totalColumns());
        shader.setUniform("totalRows", metadata.totalRows());
        shader.setUniform("currentRow", animation.getCurrentTextureRow());
        shader.setUniform("currentColumn", animation.getCurrentTextureColumn());
    }

    @Override
    public List<Drawable> getSubDrawables() {
        return List.of(text);
    }

    @Override
    public boolean shouldRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public void update(final double dt, final double mouseX, final double mouseY) {
        hover = mouseIntersectsUI(mouseX, mouseY);
        animation.setAnimationState(hover ? AnimationState.HOVER : AnimationState.STATIC);
    }

    boolean mouseIntersectsUI(final double mouseX, final double mouseY){
        return IntersectionResolver.newInstance()
                .addShape(position, dimensions)
                .addShape((float) mouseX, (float) mouseY)
                .resolve();
    }

    @Override
    public void mousePressed(final int button, double mouseX, double mouseY) {
        if(onClick == null){
            return;
        }
        if(!mouseIntersectsUI(mouseX, mouseY)){
            return;
        }
        onClick.run();
    }

    @Override
    public void mouseReleased(final int button, double mouseX, double mouseY) {

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
