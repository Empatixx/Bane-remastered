package cz.empatix.hud.service;


import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.joml.Vector3i;

import java.util.List;

public class Text implements HUDElement {
    private final static RenderScene DEFAULT_SCENE = RenderScene.HUD;
    private String text;
    private final Font font;
    private final Vector2f position;
    private final Vector3f color;
    private final int zIndex;
    private final RenderScene renderScene;

    public Text(final String text, final String font, final Vector2f position, final int prevZIndex) {
        this.text = text;
        this.font = AssetsService.getFont(font, FontSize.FONTSIZE_20);
        this.position = position;
        zIndex = prevZIndex + 1;
        color = new Vector3f(1f, 1f, 1f);
        renderScene = DEFAULT_SCENE;
    }
    public Text(final String text, final String font, final Vector2f position, final int prevZIndex,
                final RenderScene renderScene) {
        this.text = text;
        this.font = AssetsService.getFont(font, FontSize.FONTSIZE_20);
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
    public List<Renderable> getSubRenderables() {
        return font.getDrawables(text, position, color, zIndex, renderScene);
    }

    @Override
    public boolean canRender() {
        throw new UnsupportedOperationException("Should not be called");
    }

    @Override
    public void update(double dt) {

    }

    public String getText() {
        return text;
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

}
