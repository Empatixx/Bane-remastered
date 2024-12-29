package cz.empatix.postprocessing.service;

import cz.empatix.postprocessing.domain.*;
import cz.empatix.render.domain.ColorFramebuffer;
import cz.empatix.render.domain.DepthColorFramebuffer;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.service.RenderService;

import java.util.List;

import static org.lwjgl.opengl.GL11.*;

public class PostprocessService {
    private final RenderService renderService;
    private final List<Postprocess> postprocesses;
    private final Postprocess fallbackPostprocess;
    private final Framebuffer fbo1;
    private final Framebuffer fbo2;


    public PostprocessService(final RenderService renderService) {
        this.renderService = renderService;
        this.postprocesses = List.of(
                new GaussianBlur(),
                new Fade(),
                new RedVignette()
        );
        this.fallbackPostprocess = new NoPostprocessing();

        fbo1 = new ColorFramebuffer(4);
        fbo2 = new ColorFramebuffer(4);
        Camera.getInstance().addObserver(fbo1);
        Camera.getInstance().addObserver(fbo2);
    }

    public boolean hasNonePostProcessingForScene(final RenderScene scene) {
        return postprocesses.stream()
                .filter(Postprocess::canRender)
                .noneMatch(postprocess -> postprocess.accepts(scene));
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
    }

    private void executePostprocess(final Postprocess postprocess,
                                    final Framebuffer inputFbo,
                                    final float dt) {
        inputFbo.unbind();
        postprocess.setInput(inputFbo);
        postprocess.apply(dt);
        renderService.render(postprocess.getRenderableGroup());
    }
}
