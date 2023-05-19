package cz.empatix.render.attributes;

import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public interface DrawableBatch extends Renderable {
    List<Drawable> getDrawables();
    RenderScene renderScene();
    ShaderBinding shaderBinding();
    int getZIndex();
}
