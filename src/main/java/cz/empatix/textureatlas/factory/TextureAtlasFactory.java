package cz.empatix.textureatlas.factory;

import com.google.common.base.Preconditions;
import cz.empatix.assets.domain.RawImage;
import cz.empatix.textureatlas.domain.RawTextureAtlas;
import cz.empatix.textureatlas.domain.SortingMethod;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.textureatlas.service.TextureAtlasPacker;

import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class TextureAtlasFactory {
    private final TextureAtlasPacker packer;

    public TextureAtlasFactory() {
        packer = new TextureAtlasPacker();
    }

    public TextureAtlas createAtlas(final Map<String, RawImage> images) {
        final List<SortingMethod> sortingMethods = List.of(SortingMethod.values());
        final List<RawTextureAtlas> textureAtlases = sortingMethods
                .stream()
                .map(method -> createAtlas(images, method))
                .toList();
        final RawTextureAtlas smallest = textureAtlases
                .stream()
                .min(Comparator.comparingInt(atlas -> atlas.pixels().length))
                .orElseThrow(() -> new IllegalStateException("No texture atlas was created"));
        return TextureAtlas.from(smallest);
    }

    private Comparator<Map.Entry<String, RawImage>> sortByField(final SortingMethod method) {
        return Comparator.comparingInt(entry -> {
            final RawImage image = entry.getValue();
            return switch (method) {
                case AREA -> image.width() * image.height();
                case WIDTH -> image.width();
                case HEIGHT -> image.height();
            };
        });
    }

    private RawTextureAtlas createAtlas(final Map<String, RawImage> images,
                                        final SortingMethod method) {
        final Comparator<Map.Entry<String, RawImage>> comparator = sortByField(method);
        final LinkedHashMap<String, RawImage> sorted = Map.copyOf(images)
                .entrySet()
                .stream()
                .sorted(comparator)
                .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue, (a, b) -> a, LinkedHashMap::new));
        return packer.pack(sorted);
    }
}