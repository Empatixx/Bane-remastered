package cz.empatix.assets.creators;

import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.domain.metadata.RawMetadata;
import cz.empatix.assets.loaders.MetadataLoader;
import cz.empatix.common.utils.BufferFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.FloatBuffer;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Collectors;

import static org.lwjgl.opengl.GL15.*;

public class MetadataCreator implements AssetCreator{
    private static final Logger LOGGER = LoggerFactory.getLogger(MetadataCreator.class);
    private boolean ready;
    private final Map<String, Metadata> metadata;

    public MetadataCreator() {
        this.metadata = new HashMap<>();
    }

    @Override
    public boolean isReady() {
        return ready;
    }

    @Override
    public void ready() {
        this.ready = true;
    }

    public Metadata get(final String name) {
        return metadata.get(MetadataLoader.PATH.resolve(name).toString());
    }

    public void add(final Map<String, RawMetadata> data) {
        final long start = System.currentTimeMillis();
        final int loaded = metadata.size();

        final Map<Map.Entry<Integer, Integer>, Integer> verticesVBOs = data.entrySet().stream()
                .filter(entry -> !metadata.containsKey(entry.getKey()))
                .map(Map.Entry::getValue)
                .map(rawMetadata -> {
                    final int width = rawMetadata.width();
                    final int height = rawMetadata.height();

                    return Map.entry(width, height);
                })
                .collect(Collectors.toSet())
                .stream()
                .map(dimensions -> {
                    final int width = dimensions.getKey();
                    final int height = dimensions.getValue();
                    final int vbo = getVerticesVbo(width, height);
                    return Map.entry(dimensions, vbo);
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

        data.entrySet().stream()
                .filter(entry -> !metadata.containsKey(entry.getKey()))
                .map(entry -> {
                    final RawMetadata rawMetadata = entry.getValue();

                    final int verticesVBO = verticesVBOs.get(Map.entry(rawMetadata.width(), rawMetadata.height()));
                    final int texturesVBO = getTexturesVbo();

                    return Map.entry(entry.getKey(), new Metadata(rawMetadata, verticesVBO, texturesVBO));
                })
                .forEach(entry -> metadata.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} Metadata vbos created in {} ms", metadata.size() - loaded, System.currentTimeMillis() - start);
    }


    private int getVerticesVbo(final int width, final int height) {
        final int vboVertices = glGenBuffers();
        final float[] vertices = {
                0, 0,
                width , 0,
                width , height ,
                0, height
        };
        final FloatBuffer verticesBuffer = BufferFactory.convertFloatArray(vertices);

        glBindBuffer(GL_ARRAY_BUFFER, vboVertices);
        glBufferData(GL_ARRAY_BUFFER, verticesBuffer, GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        return vboVertices;
    }

    private int getTexturesVbo() {
        final float[] textures = {
                0, 0,
                1, 0,
                1, 1,
                0, 1
        };
        final FloatBuffer texturesBuffer = BufferFactory.convertFloatArray(textures);
        final int vboTextures = glGenBuffers();
        glBindBuffer(GL_ARRAY_BUFFER, vboTextures);
        glBufferData(GL_ARRAY_BUFFER, texturesBuffer, GL_STATIC_DRAW);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        return vboTextures;
    }

    @Override
    public void destroy() {
        metadata.values().forEach(Metadata::destroyVao);
        metadata.values().forEach(Metadata::destroyVbo);
    }
}
