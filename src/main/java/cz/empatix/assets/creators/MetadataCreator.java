package cz.empatix.assets.creators;

import cz.empatix.assets.domain.Metadata;
import cz.empatix.assets.domain.RawMetadata;
import cz.empatix.assets.loaders.MetadataLoader;
import cz.empatix.handlers.WindowHandler;
import cz.empatix.utils.BufferFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.nio.FloatBuffer;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.stream.Collectors;

import static org.lwjgl.opengl.GL15.*;

public class MetadataCreator implements AssetCreator<RawMetadata, Metadata> {
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

    @Override
    public Metadata get(final String name) {
        return metadata.get(MetadataLoader.PATH.resolve(name).toString());
    }

    @Override
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
                    final int vbo = createVerticesVBO(width, height);
                    return Map.entry(dimensions, vbo);
                })
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));

        data.entrySet().stream()
                .filter(entry -> !metadata.containsKey(entry.getKey()))
                .map(entry -> {
                    final RawMetadata rawMetadata = entry.getValue();
                    final int verticesVBO = verticesVBOs.get(Map.entry(rawMetadata.width(), rawMetadata.height()));
                    final List<List<Integer>> texturesVBOs = createTextureVBOs(rawMetadata);

                    return Map.entry(entry.getKey(), new Metadata(verticesVBO, texturesVBOs, rawMetadata.width(), rawMetadata.height()));
                })
                .forEach(entry -> metadata.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} Metadata vbos created in {} ms", metadata.size() - loaded, System.currentTimeMillis() - start);
    }


    private int createVerticesVBO(final int width, final int height) {
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

    private List<List<Integer>> createTextureVBOs(final RawMetadata metadata) {

        if (!metadata.spritesheet()) {
            if (metadata.noimage()) {
                LOGGER.debug("No image for metadata {}", metadata.name());
                return List.of();
            } else {
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
                return List.of(List.of(vboTextures));
            }
        } else {
            final List<List<Integer>> vboList = new ArrayList<>();
            for (int y = 0; y < metadata.totalRows(); y++) {
                final List<Integer> vboRow = new ArrayList<>();
                for (int x = 0; x < metadata.totalColumns(); x++) {
                    final float[] textures = {
                            (float) x / metadata.totalColumns(), (float) y / metadata.totalRows(),
                            (float) (x + 1) / metadata.totalColumns(), (float) y / metadata.totalRows(),
                            (float) (x + 1) / metadata.totalColumns(), (float) (y + 1) / metadata.totalRows(),
                            (float) x / metadata.totalColumns(), (float) (y + 1) / metadata.totalRows()
                    };
                    final FloatBuffer texturesBuffer = BufferFactory.convertFloatArray(textures);
                    final int vboTextures = glGenBuffers();

                    glBindBuffer(GL_ARRAY_BUFFER, vboTextures);
                    glBufferData(GL_ARRAY_BUFFER, texturesBuffer, GL_STREAM_DRAW);
                    glBindBuffer(GL_ARRAY_BUFFER, 0);

                    vboRow.add(vboTextures);
                }
                vboList.add(vboRow);
            }
            return vboList;
        }
    }

    @Override
    public void destroy() {
        metadata.values().forEach(Metadata::destroy);
    }
}
