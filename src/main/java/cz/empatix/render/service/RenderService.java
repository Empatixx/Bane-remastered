package cz.empatix.render.service;

import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.renders.DynamicRenderer;

import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.stream.Collectors;

public class RenderService {
    private final DynamicRenderer renderer = new DynamicRenderer();

    public void render(final RenderableGroup drawableData) {
        render(List.of(drawableData));
    }

    public void render(final List<RenderableGroup> renderableGroups) {
        if (renderableGroups == null || renderableGroups.isEmpty()) {
            return;
        }
        renderableGroups
                .stream()
                .sorted(Comparator.comparing(RenderableGroup::getZIndex))
                .collect(Collectors.groupingBy(
                        RenderableGroup::getZIndex,
                        LinkedHashMap::new,
                        Collectors.toList()))
                .forEach(this::renderDataByZIndex);
    }

    private void renderDataByZIndex(final int zIndex,
                                    final List<RenderableGroup> drawableData) {
        drawableData
                .stream()
                .collect(Collectors.groupingBy(RenderableGroup::getShaderBinding))
                .forEach((shaderBinding, drawablesOfType) -> {
                    final List<DrawableData> drawables = drawablesOfType
                            .stream()
                            .flatMap(renderableGroup -> renderableGroup.getDrawableData().stream())
                            .toList();
                    renderer.render(drawables, shaderBinding, zIndex);
                });
    }
}
