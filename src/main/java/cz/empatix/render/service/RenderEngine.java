package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.DepthColorFramebuffer;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.renders.DynamicRenderer;

import java.util.*;
import java.util.stream.Collectors;

public class RenderEngine {
    private final Framebuffer mainFramebuffer;
    private final RenderService renderService;
    private final PostprocessService postprocessService;
    private int totalDrawableLastFrame = 10;

    public RenderEngine() {
        this.mainFramebuffer = new DepthColorFramebuffer(4);
        Camera.getInstance().addObserver(mainFramebuffer);
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
        render(renderScenes.getOrDefault(RenderScene.SCENE, List.of()), RenderScene.SCENE, (float) dt);
        render(renderScenes.getOrDefault(RenderScene.HUD, List.of()), RenderScene.HUD, (float) dt);
        render(renderScenes.getOrDefault(RenderScene.OVERLAYS, List.of()), RenderScene.OVERLAYS, (float) dt);
    }

    private void render(List<RenderableGroup> renderables,
                        final RenderScene scene,
                        final float dt) {
        if (renderables.isEmpty()) {
            return;
        }
        mainFramebuffer.bind();
        mainFramebuffer.flush();
        renderService.render(renderables);
        mainFramebuffer.unbind();
        postprocessService.process(mainFramebuffer, scene, dt);
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
