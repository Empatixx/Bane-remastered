package cz.empatix.textureatlas.service;

import de.articdive.jnoise.core.api.functions.Interpolation;
import de.articdive.jnoise.generators.noise_parameters.fade_functions.FadeFunction;
import de.articdive.jnoise.modules.octavation.fractal_functions.FractalFunction;
import de.articdive.jnoise.pipeline.JNoise;
import org.lwjgl.BufferUtils;

import java.nio.ByteBuffer;

public class PerlinNoiseGenerator {
    private static final int SEED = 3302;
    private final JNoise noise;

    public PerlinNoiseGenerator() {
        noise = JNoise.newBuilder()
                //.white(SEED)
                .perlin(SEED, Interpolation.LINEAR, FadeFunction.CUBIC_POLY)
                .clamp(0,1)
                .scale(500)
                .octavate(3, 0.5, 1.2, FractalFunction.FBM,false)
                .build();
    }

    public ByteBuffer generateBuffer(final int width, final int height) {
        final ByteBuffer byteBuffer = BufferUtils.createByteBuffer(width * height * 4);
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                double nx = (double) x / width;
                double ny = (double) y / height;

                double noiseValue = noise.evaluateNoise(nx, ny);
                byteBuffer.put((byte)(255 * noiseValue));
                byteBuffer.put((byte)0);
                byteBuffer.put((byte)0);
                byteBuffer.put((byte)255);
            }
        }
        byteBuffer.flip();
        return byteBuffer;
    }

}
