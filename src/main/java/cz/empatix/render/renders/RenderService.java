package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

import java.util.*;
import java.util.stream.Collectors;

public class RenderService {
    private final Map<RenderData.RenderDataType, Renderer> renderers;

    public RenderService() {
        renderers = new HashMap<>();
        renderers.put(RenderData.RenderDataType.DYNAMIC, new DynamicRenderer());
        renderers.put(RenderData.RenderDataType.STATIC, new StaticRenderer());
    }

    public void render(final List<Drawable> drawable) {
        final List<Drawable> allDrawables = new ArrayList<>(drawable);

        final List<Drawable> foundSubDrawables = new ArrayList<>();
        final List<Drawable> searchSubDrawables = new ArrayList<>(drawable);
        do {
            foundSubDrawables.clear();
            searchSubDrawables.forEach(d -> foundSubDrawables.addAll(d.getSubDrawables()));
            allDrawables.addAll(foundSubDrawables);
            searchSubDrawables.clear();
            searchSubDrawables.addAll(foundSubDrawables);
        } while (!searchSubDrawables.isEmpty());
        allDrawables.stream()
                .sorted(Comparator.comparingInt(Drawable::getZIndex))
                .collect(Collectors.groupingBy(Drawable::getZIndex, LinkedHashMap::new, Collectors.toList()))
                .forEach((depth, drawables) -> {
                    drawables.stream()
                            .filter(d -> d.renderData() != null)
                            .collect(Collectors.groupingBy(d -> d.renderData().type()))
                            .forEach((type, drawablesOfType) -> {
                                final Renderer renderer = renderers.get(type);
                                drawablesOfType.stream()
                                        .collect(Collectors.groupingBy(d -> d.renderData().shaderBinding()))
                                        .forEach((shaderBinding, drawablesOfShader) -> {
                                            renderer.render(drawablesOfShader, shaderBinding, depth);
                                        });
                            });
                });
    }
}
