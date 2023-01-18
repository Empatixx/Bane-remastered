package cz.empatix.assets.domain;

import org.lwjgl.opengl.GL30;

import java.util.List;

public record Metadata (
        int verticesVbo,
        List<List<Integer>> texturesVbos
){
    public void destroy() {
        texturesVbos.stream()
                .flatMap(List::stream)
                .forEach(GL30::glDeleteBuffers);
        GL30.glDeleteBuffers(verticesVbo);
    }
    public List<Integer> getTexturesVbos(int index) {
        return texturesVbos.get(index);
    }
}
