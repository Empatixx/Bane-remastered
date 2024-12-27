package cz.empatix.render.service;

import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.renders.DynamicRenderer;
import cz.empatix.render.shaderbindings.ShaderBinding;

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
                .collect(Collectors.groupingBy(RenderableGroup::getShaderBinding,
                        LinkedHashMap::new,
                        Collectors.toUnmodifiableList()))
                .forEach(this::render);
    }

    private void render(final ShaderBinding shaderBinding,
                        final List<RenderableGroup> drawablesOfType) {
        final List<DrawableData> drawables = drawablesOfType
                .stream()
                .flatMap(renderableGroup -> renderableGroup.getDrawableData().stream())
                .toList();
        renderer.render(drawables, shaderBinding);
    }
}
