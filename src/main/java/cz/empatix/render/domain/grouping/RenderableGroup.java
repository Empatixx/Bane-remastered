package cz.empatix.render.domain.grouping;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public interface RenderableGroup {
    int getZIndex();
    ShaderBinding getShaderBinding();
    RenderScene getRenderScene();
    List<DrawableData> getDrawableData();
}
