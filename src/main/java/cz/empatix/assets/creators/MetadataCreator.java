package cz.empatix.assets.creators;

import cz.empatix.assets.builders.VboBuilder;
import cz.empatix.assets.domain.metadata.Metadata;
import cz.empatix.assets.domain.metadata.RawMetadata;
import cz.empatix.assets.loaders.MetadataLoader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.stream.Collectors;

public class MetadataCreator implements AssetCreator {
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
                    final int texturesVBO = VboBuilder.builder()
                            .defaultNormalizedBuffer()
                            .build();

                    return Map.entry(entry.getKey(), new Metadata(rawMetadata, verticesVBO, texturesVBO));
                })
                .forEach(entry -> metadata.put(entry.getKey(), entry.getValue()));
        LOGGER.info("Total {} Metadata vbos created in {} ms", metadata.size() - loaded, System.currentTimeMillis() - start);
    }


    private int getVerticesVbo(final int width, final int height) {
        return VboBuilder.builder()
                .buffer(new float[]{
                        0, 0,
                        width, 0,
                        width, height,
                        0, height
                })
                .build();
    }

    @Override
    public void destroy() {
        metadata.values().forEach(Metadata::destroyVao);
        metadata.values().forEach(Metadata::destroyVbo);
    }
}
