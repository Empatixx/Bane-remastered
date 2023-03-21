package cz.empatix.assets.builders;

import com.google.common.base.Preconditions;
import cz.empatix.common.utils.BufferFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.Buffer;
import java.nio.FloatBuffer;
import java.nio.IntBuffer;

import static org.lwjgl.opengl.GL15.*;

public class VboBuilder{
    private final Logger LOGGER = LoggerFactory.getLogger(VboBuilder.class);
    private BuildParam bufferType;
    private BuildParam usage;
    private Buffer buffer;
    public VboBuilder(){
        usage = BuildParam.STATIC;
    }
    public static VboBuilder builder(){
        return new VboBuilder();
    }
    public VboBuilder buffer(final FloatBuffer buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = buffer;
        bufferType = BuildParam.FLOAT_BUFFER;
        return this;
    }
    public VboBuilder buffer(final IntBuffer buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        this.buffer = buffer;
        bufferType = BuildParam.INT_BUFFER;
        return this;
    }
    public VboBuilder buffer(final float[] buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        return buffer(BufferFactory.convertFloatArray(buffer));
    }
    public VboBuilder buffer(final int[] buffer){
        Preconditions.checkNotNull(buffer, "Buffer must not be null");
        return buffer(BufferFactory.convertIntArray(buffer));
    }
    public VboBuilder dynamic(){
        usage = BuildParam.DYNAMIC;
        return this;
    }
    public VboBuilder streaming(){
        usage = BuildParam.STREAMING;
        return this;
    }
    public int build(){
        Preconditions.checkNotNull(buffer, "Buffer must be set");
        final int vbo = glGenBuffers();
        try {
            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            final int usage = switch(this.usage){
                case DYNAMIC -> GL_DYNAMIC_DRAW;
                case STREAMING -> GL_STREAM_DRAW;
                case STATIC -> GL_STATIC_DRAW;
                default -> throw new IllegalStateException("Unexpected value: " + this.usage);
            };
            switch(bufferType){
                case FLOAT_BUFFER -> glBufferData(GL_ARRAY_BUFFER, (FloatBuffer) buffer, usage);
                case INT_BUFFER -> glBufferData(GL_ARRAY_BUFFER, (IntBuffer) buffer, usage);
                default -> throw new IllegalStateException("Unexpected value: " + bufferType);
            }
            glBindBuffer(GL_ARRAY_BUFFER, 0);
            return vbo;
        } catch (Exception e){
            LOGGER.error("Failed to build VBO, usage {}, buffer type {}", usage, bufferType, e);
            glDeleteBuffers(vbo);
            throw e;
        } finally {
            LOGGER.info("VBO built successfully with id {}, usage {}, buffer type {}", vbo, usage, bufferType);
        }
    }
    private enum BuildParam{
        STREAMING,
        DYNAMIC,
        STATIC,
        FLOAT_BUFFER,
        INT_BUFFER,
    }
}
