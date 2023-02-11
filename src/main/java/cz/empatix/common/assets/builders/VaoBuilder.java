package cz.empatix.common.assets.builders;

import org.lwjgl.opengl.GL30;

import java.util.HashMap;
import java.util.Map;

import static org.lwjgl.opengl.GL11.GL_FLOAT;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;

public class VaoBuilder {
    private final Map<Integer, Integer> vboMap;
    private final Map<Integer, Integer> vboSizeMap;

    public VaoBuilder() {
        vboMap = new HashMap<>();
        vboSizeMap = new HashMap<>();
    }

    public static VaoBuilder builder() {
        return new VaoBuilder();
    }

    public VaoBuilder addVbo(int index, int vboId) {
        return addVbo(index, vboId, 2);
    }
    public VaoBuilder addVbo(int index, int vboId, int size) {
        vboMap.put(index, vboId);
        vboSizeMap.put(index, size);
        return this;
    }

    public int build() {
        final int vao = GL30.glGenVertexArrays();
        GL30.glBindVertexArray(vao);
        for (Map.Entry<Integer, Integer> entry : vboMap.entrySet()) {
            final int vbo = entry.getValue();
            final int index = entry.getKey();
            GL30.glBindBuffer(GL_ARRAY_BUFFER, vbo);
            GL30.glVertexAttribPointer(entry.getKey(), vboSizeMap.get(index), GL_FLOAT, false, 0, 0);
            GL30.glEnableVertexAttribArray(entry.getKey());
        }
        GL30.glBindBuffer(GL_ARRAY_BUFFER, 0);
        GL30.glBindVertexArray(0);
        return vao;
    }
}
