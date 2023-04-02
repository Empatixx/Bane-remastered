package cz.empatix.ui;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.service.IntersectionResolver;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.CommonShaderBinding;
import cz.empatix.hud.Text;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;

import java.util.List;

public class Button implements UIElement {
    private final static String DEFAULT_TEXTURE = "menu/Menu_bar.tga";
    private final static Vector3i DEFAULT_COLOR = new Vector3i(222, 112, 38);
    private final static int DEFAULT_WIDTH = 100;
    private final static int DEFAULT_HEIGHT = 50;
    private final Vector2f position;
    private final Vector2i dimensions;
    private final Text text;
    private final Animation animation;
    private boolean hover;
    private Runnable onClick;

    public Button(final String text, final Vector2f position) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR, DEFAULT_WIDTH, DEFAULT_HEIGHT, 1);
    }
    public Button(final String text, final Vector2f position, final float scale) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR, DEFAULT_WIDTH, DEFAULT_HEIGHT, scale);
    }

    public Button(final String text,
                  final Vector2f position,
                  final String texture,
                  final Vector3i rgb,
                  final int width,
                  final int height,
                  final float scale) {
        this.dimensions = new Vector2i(width, height);
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;

        final TextureAtlas.Entry textureImage = AssetsService.getTexture(texture);
        animation = Animation.builder()
                .addAnimationState(AnimationState.STATIC, 0, 1, dimensions.x(), dimensions.y(), -1)
                .addAnimationState(AnimationState.HOVER, 1, 1, dimensions.x(), dimensions.y(), -1)
                .texture(textureImage)
                .build();

        animation.setAnimationState(AnimationState.STATIC);

        this.position = position.sub(new Vector2f(dimensions).div(2f));
        final Vector2f textPosition = new Vector2f(position)
                .add(0, dimensions.y() / 2f);
        this.text = new Text(text, "PixelFJVerdana12pt.ttf", textPosition, 1);
        this.text.setCenter(position.x, position.x + dimensions.x());
        this.text.setColorRGB(rgb.x(), rgb.y(), rgb.z());
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonShaderBinding(Camera.getProjection()),
                RenderData.RenderDataType.DYNAMIC
        );
    }

    public void setOnClick(final Runnable onClick) {
        this.onClick = onClick;
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

    boolean mouseIntersectsUI(final double mouseX, final double mouseY) {
        return IntersectionResolver.newInstance()
                .addShape(position, new Vector2f(dimensions.x(), dimensions.y()))
                .addShape((float) mouseX, (float) mouseY)
                .resolve();
    }

    @Override
    public void mousePressed(final int button, double mouseX, double mouseY) {
        if (onClick == null) {
            return;
        }
        if (!mouseIntersectsUI(mouseX, mouseY)) {
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
