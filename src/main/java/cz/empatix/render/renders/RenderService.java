package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;

import java.util.*;
import java.util.stream.Collectors;

public class RenderService {
    private final List<Renderer> renderers;

    public RenderService() {
        renderers = new ArrayList<>();
        renderers.add(new TextRenderer());
        renderers.add(new StaticRenderer());
        renderers.add(new DynamicRenderer());
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
                .sorted(Comparator.comparing(Drawable::getZIndex)
                        .thenComparing(a -> a.getClass().getSimpleName()))
                .collect(Collectors.groupingBy(Object::getClass, LinkedHashMap::new, Collectors.toList()))
                .values()
                .stream()
                .map(batchDrawable -> batchDrawable.stream()
                        .map(Drawable::renderData)
                        .filter(Objects::nonNull)
                        .toList())
                .filter(drawableBatch -> !drawableBatch.isEmpty())
                .forEach(drawableBatch -> {
                    /*Preconditions.checkArgument(drawableBatch.stream()
                            .map(RenderData::type)
                            .collect(Collectors.toSet()).size() == 1, "Batch contains different types of render data");*/
                    renderers.stream()
                            .filter(renderer -> renderer.accepts(drawableBatch.get(0)))
                            .forEach(renderer -> renderer.batchRender(drawableBatch));
                });
    }
}
