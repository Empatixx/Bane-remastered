package cz.empatix.common.assets.builders;

import com.google.common.base.Preconditions;
import cz.empatix.common.utils.BufferFactory;

import java.nio.FloatBuffer;

import static org.lwjgl.opengl.GL15.*;

public class VboBuilder {
    private FloatBuffer buffer;
    private boolean dynamic;
    private boolean streaming;

    public static VboBuilder builder(){
        return new VboBuilder();
    }
    public VboBuilder buffer(final FloatBuffer buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = buffer;
        return this;
    }
    public VboBuilder buffer(final float[] buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = BufferFactory.convertFloatArray(buffer);
        return this;
    }
    public VboBuilder defaultNormalizedBuffer(){
        final float[] vbo = {
                0, 0,
                1, 0,
                1, 1,
                0, 1
        };
        return buffer(vbo);
    }
    public VboBuilder glDefaultNormalizedBuffer(){
        final float[] vbo = {
                0, 1,
                1, 1,
                1, 0,
                0, 0
        };
        return buffer(vbo);
    }
    public VboBuilder dynamic(){
        Preconditions.checkState(!streaming, "Cannot be both dynamic and streaming");
        this.dynamic = true;
        return this;
    }
    public VboBuilder streaming(){
        Preconditions.checkState(!dynamic, "Cannot be both dynamic and streaming");
        this.streaming = true;
        return this;
    }
    public int build(){
        Preconditions.checkNotNull(buffer, "Buffer must be set");
        final int vbo = glGenBuffers();
        glBindBuffer(GL_ARRAY_BUFFER, vbo);
        if (dynamic){
            glBufferData(GL_ARRAY_BUFFER, buffer, GL_DYNAMIC_DRAW);
        } else if (streaming) {
            glBufferData(GL_ARRAY_BUFFER, buffer, GL_STREAM_DRAW);
        } else {
            glBufferData(GL_ARRAY_BUFFER, buffer, GL_STATIC_DRAW);
        }
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        return vbo;
    }
}
