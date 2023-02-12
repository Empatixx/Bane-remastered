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
                .filter(Drawable::shouldRender)
                .collect(Collectors.groupingBy(d -> d.renderData().type()))
                .forEach((type, drawables) -> {
                    final Renderer renderer = renderers.get(type);

                    drawables.stream()
                            .collect(Collectors.groupingBy(Drawable::getZIndex))
                            .entrySet()
                            .stream()
                            .sorted(Map.Entry.comparingByKey())
                            .forEach(entry -> renderer.batchRender(entry.getValue().stream()
                                    .map(Drawable::renderData)
                                    .collect(Collectors.toList())));
                });
    }
}
