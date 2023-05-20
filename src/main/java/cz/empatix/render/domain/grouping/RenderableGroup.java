package cz.empatix.render.domain.grouping;

import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderBinding;

public interface RenderableGroup {
    int getZIndex();
    ShaderBinding getShaderBinding();
    RenderScene getRenderScene();
}
