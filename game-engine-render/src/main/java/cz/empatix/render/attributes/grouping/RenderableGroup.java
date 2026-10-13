package cz.empatix.render.attributes.grouping;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderProgramBinding;

import java.util.List;

public interface RenderableGroup {
    int getZIndex();
    ShaderProgramBinding getShaderBinding();
    RenderScene getRenderScene();
    List<DrawableData> getDrawableData();
    boolean isRenderable();
}
