package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
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
    public void render(final Drawable drawable){
        render(List.of(drawable));
    }
    public void render(final List<Drawable> drawable) {
        final List<Drawable> allDrawables = findAllDrawables(drawable);
        allDrawables.sort(Comparator.comparingInt(Drawable::getZIndex));

        final LinkedHashMap <Integer, List<Drawable>> drawablesByZIndex = new LinkedHashMap<>();
        for(final Drawable d : allDrawables){
            drawablesByZIndex.computeIfAbsent(d.getZIndex(), k -> new ArrayList<>()).add(d);
        }

        for(final Map.Entry<Integer, List<Drawable>> drawablesPerZ : drawablesByZIndex.entrySet()){
            renderDrawablesByZIndex(drawablesPerZ.getKey(), drawablesPerZ.getValue());
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
    private void renderDrawablesByZIndex(int zIndex, List<Drawable> drawables) {
        final Map<RenderData.RenderDataType, List<Drawable>> drawablesByType = new EnumMap<>(RenderData.RenderDataType.class);

        for (final Drawable d : drawables) {
            if (d.renderData() != null) {
                final RenderData.RenderDataType type = d.renderData().type();
                drawablesByType.computeIfAbsent(type, k -> new ArrayList<>()).add(d);
            }
        }

        for (Map.Entry<RenderData.RenderDataType, List<Drawable>> typeEntry : drawablesByType.entrySet()) {
            renderDrawablesByType(typeEntry.getKey(), typeEntry.getValue(), zIndex);
        }
    }
    private void renderDrawablesByType(RenderData.RenderDataType type, List<Drawable> drawablesOfType, int zIndex) {
        final Renderer renderer = renderers.get(type);
        final Map<ShaderBinding, List<Drawable>> drawablesByShader = new HashMap<>();

        for (final Drawable d : drawablesOfType) {
            final ShaderBinding shaderBinding = d.renderData().shaderBinding();
            drawablesByShader.computeIfAbsent(shaderBinding, k -> new ArrayList<>()).add(d);
        }

        for (final Map.Entry<ShaderBinding, List<Drawable>> shaderEntry : drawablesByShader.entrySet()) {
            renderer.render(shaderEntry.getValue(), shaderEntry.getKey(), zIndex);
        }
    }
}
