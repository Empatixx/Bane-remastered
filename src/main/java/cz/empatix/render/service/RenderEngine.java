package cz.empatix.render.service;

import cz.empatix.postprocessing.service.PostprocessService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.renders.RenderService;

import java.util.List;

public class RenderEngine {
    private final Framebuffer sceneFbo;
    private final Framebuffer hudFbo;
    private final RenderService renderService;
    private final PostprocessService postprocessService;

    public RenderEngine() {
        this.sceneFbo = Framebuffer.create();
        this.hudFbo = Framebuffer.create();
        this.renderService = new RenderService();
        this.postprocessService = new PostprocessService(renderService);
    }

    public void render(final List<Drawable> drawables,
                       final double dt) {
        Camera.update((float) dt);

        final List<Drawable> hudDrawables = drawables.stream()
                .filter(drawable -> drawable.targetScene() == RenderScene.HUD)
                .toList();
        final List<Drawable> sceneDrawables = drawables.stream()
                .filter(drawable -> drawable.targetScene() == RenderScene.SCENE)
                .toList();

        sceneFbo.bind();
        renderService.render(sceneDrawables);
        sceneFbo.unbind();

        postprocessService.process(sceneFbo, (float) dt);

        //hudFbo.bind();
        renderService.render(hudDrawables);
        //hudFbo.unbind();

        postprocessService.flushFramebuffers();
        sceneFbo.flush();
        hudFbo.flush();
    }
}
