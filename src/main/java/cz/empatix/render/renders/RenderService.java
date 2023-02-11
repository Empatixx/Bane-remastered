package cz.empatix.render.renders;

import com.google.common.base.Preconditions;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.List;
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
                        .toList())
                .forEach(drawableBatch -> {
                    Preconditions.checkArgument(drawableBatch.stream()
                            .map(RenderData::type)
                            .collect(Collectors.toSet()).size() == 1, "Batch contains different types of render data");
                    renderers.stream()
                            .filter(renderer -> renderer.accepts(drawableBatch.get(0)))
                            .forEach(renderer -> renderer.batchRender(drawableBatch));
                });
    }
}
