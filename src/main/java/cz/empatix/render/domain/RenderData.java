package cz.empatix.render.domain;

import cz.empatix.render.shaderbindings.ShaderProgramBinding;

import java.util.List;

public record RenderData (
    List<Vertex> vertices,
    int textureId,
    ShaderProgramBinding shaderProgramBinding
){
}
