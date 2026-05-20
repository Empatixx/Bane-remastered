package cz.empatix.render.domain;

import cz.empatix.render.attributes.Renderable;

import java.util.ArrayList;
import java.util.List;

public class RenderableState {
    private final List<Renderable> renderables;

    public RenderableState(int initialCapacity) {
        renderables = new ArrayList<>(initialCapacity);
    }

    public RenderableState add(Renderable renderable) {
        renderables.add(renderable);
        return this;
    }

    public RenderableState addAll(List<? extends Renderable> list) {
        renderables.addAll(list);
        return this;
    }

    public void clear() {
        renderables.clear();
    }

    public List<Renderable> getList() {
        return renderables;
    }
}
