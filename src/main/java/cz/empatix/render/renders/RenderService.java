package cz.empatix.render.renders;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public class RenderService {
    private final List<Renderer> renderers;
    public RenderService(){
        renderers = new ArrayList<>();
        renderers.add(new TextRenderer());
        renderers.add(new CommonRenderer());
    }
    public void render(final List<Drawable> drawable){
        final List<Drawable> allDrawables = new ArrayList<>(drawable);

        final List<Drawable> foundSubDrawables = new ArrayList<>();
        final List<Drawable> searchSubDrawables = new ArrayList<>(drawable);
        do{
            foundSubDrawables.clear();
            searchSubDrawables.forEach(d -> foundSubDrawables.addAll(d.getSubDrawables()));
            allDrawables.addAll(foundSubDrawables);
            searchSubDrawables.clear();
            searchSubDrawables.addAll(foundSubDrawables);
        } while(!searchSubDrawables.isEmpty());

        allDrawables.stream()
                .filter(Drawable::shouldRender)
                .sorted(Comparator.comparing(a -> a.getClass().getName()))
                .forEach(this::render);
    }
    public void render(final Drawable  drawable){
        for(final Renderer renderer : renderers){
            if(renderer.accepts(drawable)){
                renderer.render(drawable);
                return;
            }
        }
    }
}
