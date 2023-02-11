package cz.empatix.render.hud;


import cz.empatix.common.assets.creators.FontCreator;
import cz.empatix.common.assets.domain.font.Font;
import cz.empatix.common.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Shader;
import org.joml.Vector2f;
import org.joml.Vector3f;

public class Text implements HUDElement {
    private String text;
    private final Shader shader;
    private final Font font;
    private final Vector2f position;
    private final Vector3f color;

    private final int zIndex;

    public Text(final String text, final String font, final Vector2f position, final int prevZIndex) {
        this.text = text;
        this.font = AssetsService.getFont(font, FontCreator.FONT_SIZE_24);
        this.shader = AssetsService.getShader("text");
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
        return null;
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform1i("sampler", 0);
        shader.setUniform4x4f("projection", Camera.getProjection()
                .translate(position.x, position.y, 0f));
        shader.setUniform3f("color", color);
    }

    @Override
    public boolean shouldRender() {
        return true;
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
