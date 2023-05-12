package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.Framebuffer;

import java.util.*;
import java.util.stream.Collectors;

public class RenderEngine {
    private final Framebuffer sceneFbo;
    private final Framebuffer hudFbo;
    private final RenderService renderService;
    private final PostprocessService postprocessService;

    public RenderEngine() {
        this.sceneFbo = Framebuffer.createRgb();
        this.hudFbo = Framebuffer.createRgba();
        this.renderService = new RenderService();
        this.postprocessService = new PostprocessService(renderService);
    }

    public void render(final List<Drawable> drawables,
                       final double dt) {
        final Map<RenderScene, List<DrawableData>> drawableData = findAllDrawables(drawables)
                .stream()
                .map(DrawableData::from)
                .collect(Collectors.groupingBy(DrawableData::target));
        Camera.update((float) dt);

        sceneFbo.bind();
        renderService.render(drawableData.get(RenderScene.SCENE));
        sceneFbo.unbind();

        hudFbo.bind();
        renderService.render(drawableData.get(RenderScene.HUD));
        hudFbo.unbind();

        postprocessService.process(sceneFbo, RenderScene.SCENE, (float) dt);
        postprocessService.process(hudFbo, RenderScene.HUD, (float) dt);

        postprocessService.flushFramebuffers();
        sceneFbo.flush();
        hudFbo.flush();
    }

    private List<Drawable> findAllDrawables(final List<Drawable> drawable) {
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
}
