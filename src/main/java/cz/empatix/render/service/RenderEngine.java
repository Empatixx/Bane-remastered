package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.renders.DynamicRenderer;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.stream.Collectors;

public class RenderEngine {
    private final Framebuffer fbo;
    private final RenderService renderService;
    private final PostprocessService postprocessService;

    private int totalDrawableLastFrame = 10;

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
                .collect(Collectors.groupingBy(RenderableGroup::getRenderScene, () -> new EnumMap<>(RenderScene.class), Collectors.toList()));
        render(renderScenes, RenderScene.SCENE, (float) dt);
        render(renderScenes, RenderScene.HUD, (float) dt);
        render(renderScenes, RenderScene.OVERLAYS, (float) dt);

    }

    private void render(Map<RenderScene, List<RenderableGroup>> renderScenes,
                        final RenderScene scene,
                        final float dt) {
        DynamicRenderer.draws.set(0);
        fbo.bind();
        renderService.render(renderScenes.get(scene));
        fbo.unbind();
        postprocessService.process(fbo, scene, dt);
        fbo.flush();

        postprocessService.flushFramebuffers();
        //LoggerFactory.getLogger(RenderEngine.class).info("Rendered total draws {} for scene {}", DynamicRenderer.draws.get(), scene);
    }

    private List<Renderable> findAllDrawables(final List<Renderable> renderables) {
        final List<Renderable> allDrawables = new ArrayList<>(totalDrawableLastFrame);
        allDrawables.addAll(renderables);

        final Queue<Renderable> drawableQueue = new LinkedList<>(renderables);

        while (!drawableQueue.isEmpty()) {
            final Renderable current = drawableQueue.poll();
            final List<Renderable> subDrawables = current.getSubRenderables();
            if (subDrawables.isEmpty()){
                continue;
            }
            allDrawables.addAll(subDrawables);
            drawableQueue.addAll(subDrawables);
        }

        totalDrawableLastFrame = allDrawables.size();

        return allDrawables;
    }
}
