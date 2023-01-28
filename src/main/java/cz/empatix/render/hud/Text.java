package cz.empatix.render.hud;


import cz.empatix.assets.creators.FontCreator;
import cz.empatix.assets.domain.font.Font;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.Camera;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.renderdata.TextRenderData;
import org.joml.Vector2f;

public class Text implements HUDElement {
    private String text;
    private final Shader shader;
    private final Font font;
    private final Vector2f position;

    public Text(final String text, final String font, final Vector2f position) {
        this.text = text;
        this.font = AssetsService.getFont(font, FontCreator.FONT_SIZE_16);
        this.shader = AssetsService.getShader("shader");
        this.position = position;
    }


    @Override
    public TextRenderData renderData() {
        return new TextRenderData(
                font.vao(),
                font.glyphs(),
                text,
                shader,
                this::prepareShader
        );
    }

    private void prepareShader(final Shader shader) {
        shader.setUniform("sampler", 0);
        shader.setUniform("projection", Camera.getProjection()
                .translate(position.x, position.y, 0)
        );
        shader.setUniform("totalColumns", 1);
        shader.setUniform("totalRows", 1);
        shader.setUniform("currentRow", 0);
        shader.setUniform("currentColumn", 0);
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
}
