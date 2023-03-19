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
        //renderers.put(RenderData.RenderDataType.STATIC, new StaticRenderer());
    }

    public void render(final List<Drawable> drawable) {
        final Set<Drawable> allDrawables = findAllDrawables(drawable);
        allDrawables.stream()
                .sorted(Comparator.comparingInt(Drawable::getZIndex))
                .collect(Collectors.groupingBy(Drawable::getZIndex, LinkedHashMap::new, Collectors.toList()))
                .forEach(this::renderDrawablesByZIndex);
    }

    private Set<Drawable> findAllDrawables(List<Drawable> drawable) {
        final Set<Drawable> allDrawables = new HashSet<>(drawable);
        Queue<Drawable> drawableQueue = new LinkedList<>(drawable);

        while (!drawableQueue.isEmpty()) {
            final Drawable current = drawableQueue.poll();
            final List<Drawable> subDrawables = current.getSubDrawables();
            allDrawables.addAll(subDrawables);
            drawableQueue.addAll(subDrawables);
        }

        return allDrawables;
    }
    private void renderDrawablesByZIndex(int zIndex, List<Drawable> drawables) {
        drawables.stream()
                .filter(d -> d.renderData() != null)
                .collect(Collectors.groupingBy(d -> d.renderData().type()))
                .forEach((type, drawablesOfType) -> renderDrawablesByType(type, drawablesOfType, zIndex));
    }
    private void renderDrawablesByType(RenderData.RenderDataType type, List<Drawable> drawablesOfType, int zIndex) {
        var renderer = renderers.get(type);
        drawablesOfType.stream()
                .collect(Collectors.groupingBy(d -> d.renderData().shaderBinding()))
                .forEach((shaderBinding, drawablesOfShader) -> renderer.render(drawablesOfShader, shaderBinding, zIndex));
    }
}
