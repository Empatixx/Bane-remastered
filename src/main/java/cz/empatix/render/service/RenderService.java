package cz.empatix.render.service;

import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderDataType;
import cz.empatix.render.renders.DynamicRenderer;
import cz.empatix.render.renders.Renderer;
import cz.empatix.render.renders.StaticRenderer;

import java.util.*;
import java.util.stream.Collectors;

public class RenderService {
    private final Map<RenderDataType, Renderer> renderers;

    public RenderService() {
        final EnumMap<RenderDataType, Renderer> map = new EnumMap<>(RenderDataType.class) {{
            put(RenderDataType.DYNAMIC, new DynamicRenderer());
            put(RenderDataType.STATIC, new StaticRenderer());
        }};
        renderers = Collections.unmodifiableMap(map);
    }

    public void render(final DrawableData drawableData){
        render(List.of(drawableData));
    }
    public void render(final List<DrawableData> drawableData) {
        if(drawableData == null || drawableData.isEmpty()){
            return;
        }
        drawableData
                .stream()
                .filter(d -> Objects.nonNull(d.renderData()))
                .collect(Collectors.groupingBy(DrawableData::zIndex))
                .forEach(this::renderDataByZIndex);
    }

    private void renderDataByZIndex(final int zIndex,
                                    final List<DrawableData> drawableData) {
        drawableData
                .stream()
                .collect(Collectors.groupingBy(DrawableData::type))
                .forEach((renderDataType, drawablesOfType) -> renderDataByType(renderDataType, zIndex, drawablesOfType));
    }

    private void renderDataByType(final RenderDataType renderDataType,
                                  final int zIndex,
                                  final List<DrawableData> drawableData) {
        final Renderer renderer = renderers.get(renderDataType);
        drawableData
                .stream()
                .collect(Collectors.groupingBy(d -> d.renderData().shaderBinding()))
                .forEach((shaderBinding, drawablesOfType) -> {
                    renderer.render(drawablesOfType, shaderBinding, zIndex);
                });
    }
}
