package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Matrix4f;
import org.joml.Vector3f;
import org.joml.Vector4f;

public class StatusBarBinding implements ShaderBinding {
    private final Matrix4f projection;
    private final float fillAmount;
    private final Vector4f barRect;
    private final Vector3f barColor;

    public StatusBarBinding(final Matrix4f projection,
                            final Vector3f barColor,
                            final float fillAmount,
                            final float normXOffset,
                            final float normYOffset,
                            final float normWidth,
                            final float normHeight) {
        this.fillAmount = fillAmount;
        this.projection = projection;
        this.barColor = barColor;
        this.barRect = new Vector4f(normXOffset, normYOffset, normWidth, normHeight);
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("statusBar");
        shader.bind();
        shader.setUniform1i("u_texture", 0);
        //shader.setUniform1f("u_fillAmount", fillAmount);
        shader.setUniform4f("u_barRect", barRect);
        shader.setUniform3f("u_barColor", barColor);
        shader.setUniform4x4f("projection", projection);
        return shader;
    }
}
