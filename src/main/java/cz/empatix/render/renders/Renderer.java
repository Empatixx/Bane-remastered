package cz.empatix.render.renders;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public interface Renderer {
    int TOTAL_VERTICES_PER_QUAD = 4;
    int TOTAL_ELEMENT_VERTICES_PER_QUAD = 6;
    void render(final List<DrawableData> drawables, final ShaderBinding shaderBinding, final int zDepth);
}
