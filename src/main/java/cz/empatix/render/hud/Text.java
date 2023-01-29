package cz.empatix.render.hud;


import cz.empatix.assets.creators.FontCreator;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.TextRenderData;
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
    public TextRenderData renderData() {
        return new TextRenderData(
                font,
                text,
                shader,
                this::prepareShader
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(position.x, position.y, 0f));
        shader.setUniform("color", color);
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
