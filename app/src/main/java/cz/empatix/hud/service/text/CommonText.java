package cz.empatix.hud.service.text;


import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.joml.Vector3i;

public class CommonText implements HUDElement {
    private final static FontSize DEFAULT_FONT_SIZE = FontSize.FONTSIZE_22;
    private final static RenderScene DEFAULT_SCENE = RenderScene.HUD;
    private String text;
    private final Font font;
    private final Vector2f position;
    private final Vector3f color;
    private final int zIndex;
    private final RenderScene renderScene;
    public CommonText(final String text, final String font, final Vector2f position, final int prevZIndex, final RenderScene renderScene) {
        this(text, font, position, prevZIndex, renderScene, DEFAULT_FONT_SIZE);
    }
    public CommonText(final String text, final String font, final Vector2f position, final int prevZIndex) {
        this(text, font, position, prevZIndex, DEFAULT_SCENE, DEFAULT_FONT_SIZE);
    }
    public CommonText(final String text, final String font, final Vector2f position, final int prevZIndex, final FontSize fontSize) {
        this(text, font, position, prevZIndex, DEFAULT_SCENE, fontSize);
    }
    public CommonText(final String text, final String font, final Vector2f position, final int prevZIndex,
                      final RenderScene renderScene, final FontSize fontSize) {
        this(text, AssetsService.getFont(font, fontSize), position, prevZIndex, renderScene);
    }

    /** Render-data-injected variant: the caller supplies the already-resolved {@link Font}. */
    public CommonText(final String text, final Font font, final Vector2f position, final int prevZIndex) {
        this(text, font, position, prevZIndex, DEFAULT_SCENE);
    }

    public CommonText(final String text, final Font font, final Vector2f position, final int prevZIndex,
                      final RenderScene renderScene) {
        this.text = text;
        this.font = font;
        this.position = position;
        this.renderScene = renderScene;
        zIndex = prevZIndex + 1;
        color = new Vector3f(1f, 1f, 1f);
    }

    public void setCenterX(final float xMin, final float xMax) {
        position.x = (xMin + xMax) / 2 - font.getWidth(text) / 2;
    }
    public void setCenterY(final float yMin, final float yMax) {
        position.y = (yMin + yMax) / 2 + font.getHeight(text) / 2;
    }

    public void setColorRGB(final int r, final int g, final int b) {
        color.set(r / 255f, g / 255f, b / 255f);
    }

    public void setColorRGB(final Vector3i rgb) {
        color.set(rgb.x() / 255f, rgb.y() / 255f, rgb.z() / 255f);
    }

    @Override
    public RenderData renderData() {
        throw new UnsupportedOperationException("Should not be called, call getSubRenderables instead");
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.addAll(font.getDrawables(text, position, color, zIndex, renderScene, false));
    }

    @Override
    public boolean canRender() {
        throw new UnsupportedOperationException("Should not be called");
    }

    @Override
    public void update(double dt) {

    }

    @Override
    public int getZIndex() {
        return zIndex;
    }

    public void setText(final String text) {
        this.text = text;
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public float getWidth() {
        return font.getWidth(text);
    }
    public float getHeight(){
        return font.getHeight(text);
    }

    public float getX() {
        return position.x();
    }
    public float getY(){
        return position.y();
    }

    public Vector2f getPosition() {
        return position;
    }

    public String getText() {
        return text;
    }
}
