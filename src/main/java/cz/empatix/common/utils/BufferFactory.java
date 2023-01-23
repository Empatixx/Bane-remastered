package cz.empatix.common.utils;

import org.joml.Matrix4f;
import org.lwjgl.BufferUtils;

import java.nio.ByteBuffer;
import java.nio.FloatBuffer;

public class BufferFactory {
    public static FloatBuffer convertMatrix4f(final Matrix4f matrix4f){
        final FloatBuffer buffer = BufferUtils.createFloatBuffer(16);
        matrix4f.get(buffer);
        return buffer;
    }
    public static FloatBuffer convertFloatArray(final float[] array){
        final FloatBuffer buffer = BufferUtils.createFloatBuffer(array.length);
        buffer.put(array);
        buffer.flip();
        return buffer;
    }
    public static ByteBuffer convertByteArray(final byte[] array){
        final ByteBuffer buffer = BufferUtils.createByteBuffer(array.length);
        buffer.put(array);
        buffer.flip();
        return buffer;
    }
}
