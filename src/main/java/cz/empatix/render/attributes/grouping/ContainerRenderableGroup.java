package cz.empatix.render.attributes.grouping;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public class ContainerRenderableGroup implements RenderableGroup {
    public static final ContainerRenderableGroup INSTANCE = new ContainerRenderableGroup();

    @Override
    public boolean isRenderable() {
        return false;
    }

    @Override
    public int getZIndex() {
        return 0;
    }

    @Override
    public ShaderBinding getShaderBinding() {
        return null;
    }

    @Override
    public RenderScene getRenderScene() {
        return RenderScene.HUD;
    }

    @Override
    public List<DrawableData> getDrawableData() {
        return List.of();
    }
}
