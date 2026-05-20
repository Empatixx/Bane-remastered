package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.ColorFramebufferAttachment;
import cz.empatix.render.domain.DepthStencilFramebufferAttachment;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.attributes.grouping.RenderableGroup;
import cz.empatix.render.renders.DynamicRenderer;

import cz.empatix.render.domain.RenderableState;

import java.util.*;
import java.util.stream.Collectors;

public class RenderEngine {
    private final Framebuffer mainFramebuffer;
    private final RenderService renderService;
    private final PostprocessService postprocessService;

    private final List<Renderable> allDrawables = new ArrayList<>(256);
    private final ArrayDeque<Renderable> drawableQueue = new ArrayDeque<>(128);
    private final RenderableState subRenderableState = new RenderableState(64);

    public RenderEngine() {
        this.mainFramebuffer = new Framebuffer();
        this.mainFramebuffer.attach(new ColorFramebufferAttachment(4));
        this.mainFramebuffer.attach(new DepthStencilFramebufferAttachment());
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

        renderService.beginRendering();
        render(renderScenes, RenderScene.SCENE, (float) dt);
        render(renderScenes, RenderScene.HUD, (float) dt);
        render(renderScenes, RenderScene.OVERLAYS, (float) dt);
        renderService.endRendering();

        mainFramebuffer.flush();
        postprocessService.flushFramebuffers();
    }

    private void render(final Map<RenderScene, List<RenderableGroup>> renderScenes,
                        final RenderScene scene,
                        final float dt) {
        final List<RenderableGroup> renderables = renderScenes.get(scene);
        if (renderables == null || renderables.isEmpty()) {
            return;
        }
        mainFramebuffer.bind();
        mainFramebuffer.clear();
        renderService.render(renderables);
        mainFramebuffer.unbind();
        postprocessService.process(mainFramebuffer, scene, dt);
    }

    private List<Renderable> findAllDrawables(final List<Renderable> renderables) {
        allDrawables.clear();
        drawableQueue.clear();

        allDrawables.addAll(renderables);
        drawableQueue.addAll(renderables);

        while (!drawableQueue.isEmpty()) {
            final Renderable current = drawableQueue.poll();
            subRenderableState.clear();
            current.collectSubRenderables(subRenderableState);
            List<Renderable> subs = subRenderableState.getList();
            if (!subs.isEmpty()) {
                allDrawables.addAll(subs);
                drawableQueue.addAll(subs);
            }
        }

        return allDrawables;
    }
}
