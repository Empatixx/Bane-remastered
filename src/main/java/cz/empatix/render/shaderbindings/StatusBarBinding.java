package cz.empatix.render.shaderbindings;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.render.domain.Shader;
import org.joml.Vector4f;

public class StatusBarBinding implements ShaderBinding {
    private final float fillAmount;
    private final Vector4f barRect;

    public StatusBarBinding(final float fillAmount,
                            final float normXOffset,
                            final float normYOffset,
                            final float normWidth,
                            final float normHeight) {
        this.fillAmount = fillAmount;
        this.barRect = new Vector4f(normXOffset, normYOffset, normWidth, normHeight);
    }

    @Override
    public Shader apply() {
        final Shader shader = AssetsService.getShader("statusBar");
        shader.setUniform1i("u_texture", 0);
        shader.setUniform1f("u_fillAmount", fillAmount);
        shader.setUniform4f("u_barRect", barRect);
        return shader;
    }
}
