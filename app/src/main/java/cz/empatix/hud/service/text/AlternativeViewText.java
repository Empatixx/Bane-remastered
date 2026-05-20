package cz.empatix.hud.service.text;


import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.RenderScene;
import org.joml.*;

public class AlternativeViewText implements HUDElement {
    private String text;
    private final Font font;
    private final Vector2f position;
    private final Vector3f color;
    private final int zIndex;
    private final RenderScene scene;

    public AlternativeViewText(final String text, final String font, final Vector2f position, final int prevZIndex, final FontSize fontSize,
                               final RenderScene scene) {
        this.scene = scene;
        this.text = text;
        this.font = AssetsService.getFont(font, fontSize);
        this.position = position;
        zIndex = prevZIndex + 1;
        color = new Vector3f(1f, 1f, 1f);
    }

    public AlternativeViewText(final String text, final String font, final Vector2f position, final int prevZIndex, final FontSize fontSize) {
        this(text, font, position, prevZIndex, fontSize, RenderScene.HUD);
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
        state.addAll(font.getDrawables(text, position, color, zIndex, scene, true));
    }

    @Override
    public RenderScene renderScene() {
        return scene;
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

    public void setPosition(final Vector2f position) {
        this.position.set(position);
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public float getWidth() {
        return font.getWidth(text);
    }

    public float getHeight() {
        return font.getHeight(text);
    }

    public float getX() {
        return position.x();
    }

    public float getY() {
        return position.y();
    }

    public void setPositionY(final float v) {
        position.y = v;
    }

    public Vector2f getPosition() {
        return position;
    }
}
