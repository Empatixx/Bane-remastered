package cz.empatix.postprocessing.service;

import cz.empatix.postprocessing.domain.*;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.RenderService;

import java.util.ArrayList;
import java.util.List;

import static org.lwjgl.opengl.GL11.*;

public class PostprocessService {
    private final RenderService renderService;
    private final List<Postprocess> postprocesses;
    private final Postprocess fallbackPostprocess;
    private final Framebuffer fbo1 = Framebuffer.createRgba();
    private final Framebuffer fbo2 = Framebuffer.createRgba();

    public PostprocessService(final RenderService renderService) {
        this.renderService = renderService;
        this.postprocesses = List.of(
                new ForwardLightning(),
                new GaussianBlur(),
                new Fade(),
                new RedVignette(),
                new PoisonVignette()
        );
        this.fallbackPostprocess = new NoPostprocessing();
    }

    public void process(final Framebuffer sceneFbo, final RenderScene scene, final float dt) {
        glDisable(GL_DEPTH_TEST);

        final List<Postprocess> applicablePostprocesses = postprocesses.stream()
                .filter(Postprocess::canRender)
                .filter(postprocess -> postprocess.accepts(scene))
                .toList();

        if (applicablePostprocesses.isEmpty()) {
            applyFallback(sceneFbo);
            return;
        }

        Framebuffer currentInputFbo = sceneFbo;

        for (int i = 0; i < applicablePostprocesses.size() - 1; i++) {
            final Postprocess postprocess = applicablePostprocesses.get(i);
            final Framebuffer currentOutputFbo = (i % 2 == 0) ? fbo1 : fbo2;

            executePostprocess(postprocess, currentInputFbo, currentOutputFbo, dt);

            currentInputFbo = currentOutputFbo;
        }

        final Postprocess lastPostprocess = applicablePostprocesses.get(applicablePostprocesses.size() - 1);
        executePostprocess(lastPostprocess, currentInputFbo, dt);

        glEnable(GL_DEPTH_TEST);
    }

    private void applyFallback(final Framebuffer sceneFbo) {
        fallbackPostprocess.setInput(sceneFbo);
        renderService.render(fallbackPostprocess.getRenderableGroup());
        glEnable(GL_DEPTH_TEST);
    }

    private void executePostprocess(final Postprocess postprocess,
                                    final Framebuffer inputFbo,
                                    final Framebuffer outputFbo,
                                    final float dt) {
        postprocess.setInput(inputFbo);

        outputFbo.bind();
        postprocess.apply(dt);
        renderService.render(postprocess.getRenderableGroup());
        outputFbo.unbind();
    }
    private void executePostprocess(final Postprocess postprocess,
                                    final Framebuffer inputFbo,
                                    final float dt) {
        postprocess.setInput(inputFbo);
        postprocess.apply(dt);
        renderService.render(postprocess.getRenderableGroup());
    }
}
