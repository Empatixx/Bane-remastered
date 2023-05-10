package cz.empatix.render.domain;

import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public record RenderData (
    List<Vertex> vertices,
    int textureId,
    ShaderBinding shaderBinding,
    RenderDataType type,
    RenderScene target
){
    public RenderData(final List<Vertex> vertices,
                      final int textureId,
                      final ShaderBinding shaderBinding,
                      final RenderDataType type){
        this(vertices, textureId, shaderBinding, type, RenderScene.SCENE);
    }
    public final static int NO_TEXTURE = -1;

}
