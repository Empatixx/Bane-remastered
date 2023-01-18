package cz.empatix.assets.loaders;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.ObjectReader;
import cz.empatix.assets.creators.AssetCreator;
import cz.empatix.assets.creators.MetadataCreator;
import cz.empatix.assets.domain.Metadata;
import cz.empatix.assets.domain.RawMetadata;
import cz.empatix.utils.LinuxPathStringConvertor;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.Stream;

public class MetadataLoader implements AssetLoader<RawMetadata> {
    private final static Logger LOGGER = LoggerFactory.getLogger(MetadataLoader.class);
    public final static Path PATH = Path.of("./assets/metadata/");
    private final static Path[] PRELOAD_METADATA = {
            PATH.resolve("background-overlay.json")
    };
    private final MetadataCreator metadataCreator;
    private final Map<String, RawMetadata> metadata;
    private final ObjectReader reader;
    public MetadataLoader() {
        this.metadata = new HashMap<>();
        this.reader = new ObjectMapper().readerFor(RawMetadata.class);
        this.metadataCreator = new MetadataCreator();
    }

    @Override
    public int load() {
        try (final Stream<Path> paths = Files.walk(PATH)) {
            paths.filter(Files::isRegularFile)
                    .filter(p -> p.toString().endsWith(".tga") || p.toString().endsWith(".png"))
                    .parallel()
                    .filter(path -> !metadata.containsKey(path.toString()))
                    .map(path -> Map.entry(path.toString(), decodeMetadata(path)))
                    .forEach(entry -> this.metadata.put(entry.getKey(), entry.getValue()));
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        return metadata.size();
    }

    @Override
    public int preload() {
        Arrays.stream(PRELOAD_METADATA)
                .map(path -> Map.entry(path.toString(), decodeMetadata(path)))
                .forEach(entry -> this.metadata.put(entry.getKey(), entry.getValue()));
        return PRELOAD_METADATA.length;
    }

    @Override
    public void submit() {
        metadataCreator.add(metadata);
    }

    private RawMetadata decodeMetadata(final Path path) {
        try {
            return reader.readValue(path.toFile());
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    @Override
    public MetadataCreator getCreator() {
        return metadataCreator;
    }

    @Override
    public MetadataLoader call() throws Exception {
        final long start = System.currentTimeMillis();
        final int loaded = load();
        LOGGER.info("Loaded {} metadata in {} ms", loaded, System.currentTimeMillis() - start);
        metadataCreator.ready();
        return this;
    }
}
