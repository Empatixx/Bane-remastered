package cz.empatix.ui;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector2i;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector3i;

public class StickyHoverButton implements UIElement, SelectableUI {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private static final int DEFAULT_Z_DEPTH = 1;
    private final static String DEFAULT_TEXTURE = "menu/Menu_bar2.tga";
    private final static Vector3i DEFAULT_COLOR = new Vector3i(200, 92, 18);
    private final static FontSize DEFAULT_FONT_SIZE = FontSize.FONTSIZE_22;
    private final static int DEFAULT_WIDTH = 84;
    private final static int DEFAULT_HEIGHT = 42;
    private final Vector2f position;
    private final Vector2i dimensions;
    private final CommonText text;
    private final Animation animation;
    private boolean hover;
    private final int zDepth;
    private Runnable onClick;

    public StickyHoverButton(final String text, final Vector2f position) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR, DEFAULT_WIDTH, DEFAULT_HEIGHT, 1, DEFAULT_Z_DEPTH, DEFAULT_FONT_SIZE);
    }

    public StickyHoverButton(final String text, final Vector2f position, final float scale) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR, DEFAULT_WIDTH, DEFAULT_HEIGHT, scale, DEFAULT_Z_DEPTH, DEFAULT_FONT_SIZE);
    }

    public StickyHoverButton(final String text, final String texture, final int width, final int height, final Vector2f position, final float scale, final FontSize fontSize) {
        this(text, position, texture, DEFAULT_COLOR, width, height, scale, DEFAULT_Z_DEPTH, fontSize);
    }

    public StickyHoverButton(final String text, final Vector2f position, final float scale, final int zDepth) {
        this(text, position, DEFAULT_TEXTURE, DEFAULT_COLOR, DEFAULT_WIDTH, DEFAULT_HEIGHT, scale, zDepth, DEFAULT_FONT_SIZE);
    }

    public StickyHoverButton(final String text,
                             final Vector2f position,
                             final String texture,
                             final Vector3i rgb,
                             final int width,
                             final int height,
                             final float scale,
                             final int zDepth,
                             final FontSize fontSize) {
        this.zDepth = zDepth;
        this.dimensions = new Vector2i(width, height);
        this.dimensions.x *= scale;
        this.dimensions.y *= scale;

        final TextureAtlas.Entry textureImage = AssetsService.getTexture(texture);
        animation = Animation.builder()
                .setTotal(2, 1)
                .addState(AnimationState.STATIC, 0, 1, dimensions.x(), dimensions.y(), -1)
                .addState(AnimationState.HOVER, 1, 1, dimensions.x(), dimensions.y(), -1)
                .texture(textureImage)
                .build();

        animation.setState(AnimationState.STATIC);

        this.position = position;
        final Vector2f textPosition = new Vector2f(position)
                .add(dimensions.x() / 2f, dimensions.y() / 2f)
                .add(0, AssetsService.getFont("PixelFJVerdana12pt.ttf", fontSize).getHeight(text) / 2f);
        this.text = new CommonText(text, "PixelFJVerdana12pt.ttf", textPosition, zDepth, fontSize);
        this.text.setCenterX(position.x, position.x + dimensions.x());
        this.text.setColorRGB(rgb);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    public void onClick(final Runnable onClick) {
        this.onClick = onClick;
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        text.collectSubRenderables(state);
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return zDepth;
    }

    @Override
    public void update(final double dt, final double mouseX, final double mouseY) {
        animation.setState(hover ? AnimationState.HOVER : AnimationState.STATIC);
    }

    boolean mouseIntersectsUI(final double mouseX, final double mouseY) {
        return mouseX >= position.x && mouseX <= position.x + dimensions.x
                && mouseY >= position.y && mouseY <= position.y + dimensions.y;
    }

    @Override
    public void mousePressed(final int button, double mouseX, double mouseY) {
        if (!mouseIntersectsUI(mouseX, mouseY)) {
            return;
        }
        hover = true;
        AUDIO_SERVICE.play(SoundEffect.CLICK);
        if (onClick == null) {
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

    @Override
    public boolean isSelected() {
        return hover;
    }

    @Override
    public void unselect() {
        hover = false;
    }

    @Override
    public void select() {
        hover = true;
    }

    public int getHeight() {
        return dimensions.y();
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public Vector2f getPosition() {
        return position;
    }

    public int getWidth() {
        return dimensions.x;
    }
}
