package cz.empatix.render.domain.grouping;

import cz.empatix.render.shaderbindings.ShaderBinding;

public interface DrawableGroup {
    int getZIndex();
    ShaderBinding getShaderBinding();
}
