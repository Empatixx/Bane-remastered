package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.shaderbindings.ShaderBinding;

import java.util.*;

public class RenderService {
    private final Map<RenderData.RenderDataType, Renderer> renderers;

    public RenderService() {
        final EnumMap<RenderData.RenderDataType, Renderer> map = new EnumMap<>(RenderData.RenderDataType.class) {{
            put(RenderData.RenderDataType.DYNAMIC, new DynamicRenderer());
            put(RenderData.RenderDataType.STATIC, new StaticRenderer());
        }};
        renderers = Collections.unmodifiableMap(map);
    }

    public void render(final Drawable drawable) {
        render(List.of(drawable));
    }

    public void render(final List<Drawable> drawable) {
        final List<DrawableData> allDrawables = findAllDrawables(drawable)
                .stream()
                .map(DrawableData::fromDrawable)
                .sorted(Comparator.comparingInt(DrawableData::zIndex))
                .toList();

        final LinkedHashMap<Integer, List<DrawableData>> dataByZIndex = new LinkedHashMap<>();
        for (final DrawableData d : allDrawables) {
            dataByZIndex.computeIfAbsent(d.zIndex(), k -> new ArrayList<>()).add(d);
        }

        for (final Map.Entry<Integer, List<DrawableData>> dataPerZ : dataByZIndex.entrySet()) {
            renderDataByZIndex(dataPerZ.getKey(), dataPerZ.getValue());
        }
    }

    private List<Drawable> findAllDrawables(List<Drawable> drawable) {
        final List<Drawable> allDrawables = new ArrayList<>(drawable);
        final Queue<Drawable> drawableQueue = new LinkedList<>(drawable);

        while (!drawableQueue.isEmpty()) {
            final Drawable current = drawableQueue.poll();
            final List<Drawable> subDrawables = current.getSubDrawables();
            allDrawables.addAll(subDrawables);
            drawableQueue.addAll(subDrawables);
        }

        return allDrawables;
    }

    private void renderDataByZIndex(int zIndex, List<DrawableData> drawables) {
        final Map<RenderData.RenderDataType, List<DrawableData>> dataByType = new EnumMap<>(RenderData.RenderDataType.class);

        for (final DrawableData d : drawables) {
            if (d.renderData() != null) {
                final RenderData.RenderDataType type = d.renderData().type();
                dataByType.computeIfAbsent(type, k -> new ArrayList<>()).add(d);
            }
        }

        for (Map.Entry<RenderData.RenderDataType, List<DrawableData>> typeEntry : dataByType.entrySet()) {
            renderDataByType(typeEntry.getKey(), typeEntry.getValue(), zIndex);
        }
    }

    private void renderDataByType(RenderData.RenderDataType type, List<DrawableData> drawablesOfType, int zIndex) {
        final Renderer renderer = renderers.get(type);
        final Map<ShaderBinding, List<DrawableData>> dataByShader = new HashMap<>();

        for (final DrawableData d : drawablesOfType) {
            final ShaderBinding shaderBinding = d.renderData().shaderBinding();
            dataByShader.computeIfAbsent(shaderBinding, k -> new ArrayList<>()).add(d);
        }

        for (final Map.Entry<ShaderBinding, List<DrawableData>> shaderEntry : dataByShader.entrySet()) {
            renderer.render(shaderEntry.getValue(), shaderEntry.getKey(), zIndex);
        }
    }
}
