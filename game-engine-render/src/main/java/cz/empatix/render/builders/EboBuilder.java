package cz.empatix.render.builders;

import com.google.common.base.Preconditions;
import cz.empatix.render.util.BufferFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.IntBuffer;

import static org.lwjgl.opengl.GL15.*;

public class EboBuilder {
    private static final Logger LOGGER = LoggerFactory.getLogger(EboBuilder.class);
    private IntBuffer buffer;

    public static EboBuilder builder() {
        return new EboBuilder();
    }

    public EboBuilder buffer(final IntBuffer buffer) {
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = buffer;
        return this;
    }

    public EboBuilder buffer(final int[] buffer) {
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        return buffer(BufferFactory.convertIntArray(buffer));
    }

    public int build() {
        Preconditions.checkNotNull(buffer, "Buffer must be set");
        final int ebo = glGenBuffers();
        try {
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
            glBufferData(GL_ELEMENT_ARRAY_BUFFER, buffer, GL_STATIC_DRAW);
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
            return ebo;
        } catch (Exception e) {
            LOGGER.error("Failed to build EBO", e);
            glDeleteBuffers(ebo);
            throw e;
        }
    }
}
