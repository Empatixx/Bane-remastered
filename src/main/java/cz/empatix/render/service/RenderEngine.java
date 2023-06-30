package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.grouping.RenderableGroup;

import java.util.*;
import java.util.stream.Collectors;

public class RenderEngine {
    private final Framebuffer fbo;
    private final RenderService renderService;
    private final PostprocessService postprocessService;

    public RenderEngine() {
        this.fbo = Framebuffer.createRgba();
        this.renderService = new RenderService();
        this.postprocessService = new PostprocessService(renderService);
    }

    public void render(final List<Renderable> renderables,
                       final double dt) {
        final Map<RenderScene, List<RenderableGroup>> renderScenes = findAllDrawables(renderables)
                .stream()
                .map(Renderable::getRenderableGroup)
                .filter(RenderableGroup::isRenderable)
                .collect(Collectors.groupingBy(RenderableGroup::getRenderScene));
        Camera.update((float) dt);

        render(renderScenes, RenderScene.SCENE, (float) dt);
        render(renderScenes, RenderScene.HUD, (float) dt);
        render(renderScenes, RenderScene.OVERLAYS, (float) dt);

    }

    private void render(Map<RenderScene, List<RenderableGroup>> renderScenes,
                        final RenderScene scene,
                        final float dt) {
        fbo.bind();
        renderService.render(renderScenes.get(scene));
        fbo.unbind();
        postprocessService.process(fbo, scene, dt);
        fbo.flush();

        postprocessService.flushFramebuffers();
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
