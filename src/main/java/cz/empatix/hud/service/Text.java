package cz.empatix.hud.service;


import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.List;

public class Text implements HUDElement {
    private final String text;
    private final Font font;
    private final Vector2f position;
    private final Vector3f color;
    private final int zIndex;

    public Text(final String text, final String font, final Vector2f position, final int prevZIndex) {
        this.text = text;
        this.font = AssetsService.getFont(font, FontSize.FONTSIZE_20);
        this.position = position;
        zIndex = prevZIndex + 1;
        color = new Vector3f(1f,1f,1f);
    }
    public void setCenter(final float xMin, final float xMax){
        position.x = (xMin + xMax) / 2 - font.getWidth(text) / 2;
    }
    public void setColorRGB(final float r, final float g, final float b){
        color.set(r/255f, g/255f, b/255f);
    }

    @Override
    public RenderData renderData() {
        throw new UnsupportedOperationException("Should not be called");
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return font.getDrawables(text, position, color, zIndex);
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
}
