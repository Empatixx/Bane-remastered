package cz.empatix.render.builders;

import org.lwjgl.opengl.GL30;

import java.util.HashMap;
import java.util.Map;

import static org.lwjgl.opengl.GL11.GL_FLOAT;
import static org.lwjgl.opengl.GL15.GL_ARRAY_BUFFER;

public class VaoBuilder {
    private final Map<Integer, Integer> vboMap;
    private final Map<Integer, Integer> vboSizeMap;
    private final Map<Integer, Integer> vboStrideMap;
    private final Map<Integer, Integer> vboOffsetMap;

    public VaoBuilder() {
        vboMap = new HashMap<>();
        vboSizeMap = new HashMap<>();
        vboStrideMap = new HashMap<>();
        vboOffsetMap = new HashMap<>();
    }

    public static VaoBuilder builder() {
        return new VaoBuilder();
    }

    public VaoBuilder addVbo(final int index,
                             final int vboId,
                             final int size,
                             final int stride,
                             final int offset) {
        vboMap.put(index, vboId);
        vboSizeMap.put(index, size);
        vboStrideMap.put(index, stride);
        vboOffsetMap.put(index, offset);
        return this;
    }

    public int build() {
        final int vao = GL30.glGenVertexArrays();
        GL30.glBindVertexArray(vao);
        for (Map.Entry<Integer, Integer> entry : vboMap.entrySet()) {
            final int vbo = entry.getValue();
            final int index = entry.getKey();
            GL30.glBindBuffer(GL_ARRAY_BUFFER, vbo);
            GL30.glVertexAttribPointer(entry.getKey(), vboSizeMap.get(index), GL_FLOAT, false, vboStrideMap.get(index), (long) vboOffsetMap.get(index));
            GL30.glEnableVertexAttribArray(entry.getKey());
        }
        GL30.glBindBuffer(GL_ARRAY_BUFFER, 0);
        GL30.glBindVertexArray(0);
        return vao;
    }
}
