package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.ShaderProgram;
import org.joml.Matrix4f;
import org.joml.Vector3f;
import org.joml.Vector4f;

public record StatusBarBinding(
        float fillAmount,
        float delayedFillAmount,
        Vector4f barRect,
        Vector3f barColor
) implements ShaderProgramBinding {

    public StatusBarBinding(
            final Vector3f barColor,
                            final float fillAmount,
                            final float delayedFillAmount,
                            final float normXOffset,
                            final float normYOffset,
                            final float normWidth,
                            final float normHeight) {
        this(fillAmount, delayedFillAmount, new Vector4f(normXOffset, normYOffset, normWidth, normHeight), barColor);
    }

    @Override
    public ShaderProgram bind() {
        final ShaderProgram shader = AssetsService.getShader("statusBar");
        shader.bind();
        shader.setUniform1i("u_texture", 0);
        shader.setUniform1f("u_fillAmount", fillAmount);
        shader.setUniform1f("u_delayedFillAmount", delayedFillAmount);
        shader.setUniform4f("u_barRect", barRect);
        shader.setUniform3f("u_barColor", barColor);
        return shader;
    }
}
