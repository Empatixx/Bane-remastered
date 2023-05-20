package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.*;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.grouping.RenderableGroup;
import cz.empatix.render.domain.grouping.SingleRenderableGroup;

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

    public void render(final List<Renderable> renderables,
                       final double dt) {
        final Map<RenderScene, List<RenderableGroup>> renderScenes = findAllDrawables(renderables)
                .stream()
                .map(Renderable::getRenderableGroup)
                .collect(Collectors.groupingBy(RenderableGroup::getRenderScene));
        Camera.update((float) dt);

        sceneFbo.bind();
        renderService.render(renderScenes.get(RenderScene.SCENE));
        sceneFbo.unbind();

        hudFbo.bind();
        renderService.render(renderScenes.get(RenderScene.HUD));
        hudFbo.unbind();

        postprocessService.process(sceneFbo, RenderScene.SCENE, (float) dt);
        postprocessService.process(hudFbo, RenderScene.HUD, (float) dt);

        postprocessService.flushFramebuffers();
        sceneFbo.flush();
        hudFbo.flush();
    }

    private List<Renderable> findAllDrawables(final List<Renderable> renderables) {
        final List<Renderable> allDrawables = new ArrayList<>(renderables);
        final Queue<Renderable> drawableQueue = new LinkedList<>(renderables);

        while (!drawableQueue.isEmpty()) {
            final Renderable current = drawableQueue.poll();
            final List<Renderable> subDrawables = current.getSubRenderables();
            allDrawables.addAll(subDrawables);
            drawableQueue.addAll(subDrawables);
        }

        return allDrawables;
    }
}
