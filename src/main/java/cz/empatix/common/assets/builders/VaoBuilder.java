package cz.empatix.common.assets.builders;

import org.lwjgl.opengl.GL30;

import java.util.HashMap;
import java.util.Map;

import static org.lwjgl.opengl.GL11.GL_FLOAT;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;

public class VaoBuilder {
    private Map<Integer, Integer> vboMap;

    public VaoBuilder() {
        vboMap = new HashMap<>();
    }

    public static VaoBuilder builder() {
        return new VaoBuilder();
    }

    public VaoBuilder addVbo(int index, int vboId) {
        vboMap.put(index, vboId);
        return this;
    }

    public int build() {
        final int vao = GL30.glGenVertexArrays();
        GL30.glBindVertexArray(vao);
        for (Map.Entry<Integer, Integer> entry : vboMap.entrySet()) {
            GL30.glBindBuffer(GL_ARRAY_BUFFER, entry.getValue());
            GL30.glVertexAttribPointer(entry.getKey(), 2, GL_FLOAT, false, 0, 0);
            GL30.glEnableVertexAttribArray(entry.getKey());
        }
        GL30.glBindVertexArray(0);
        return vao;
    }
}
