package cz.empatix.render.domain;

import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.List;

public record RenderData (
    List<Vertex> vertices,
    int textureId,
    ShaderBinding shaderBinding
){
}
