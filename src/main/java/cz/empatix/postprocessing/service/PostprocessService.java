package cz.empatix.postprocessing.service;

import cz.empatix.postprocessing.domain.*;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.DrawableData;
import cz.empatix.render.domain.Framebuffer;
import cz.empatix.render.renders.RenderService;

import java.util.ArrayList;
import java.util.List;

public class PostprocessService {
    private final RenderService renderService;
    private final List<Postprocess> postprocesses;
    private final Postprocess fallbackPostprocess;

    private final Framebuffer fbo1 = Framebuffer.createRgb();
    private final Framebuffer fbo2 = Framebuffer.createRgb();

    public PostprocessService(final RenderService renderService) {
        postprocesses = new ArrayList<>();
        postprocesses.add(new Fade());
        postprocesses.add(new ForwardLightning());
        postprocesses.add(new GaussianBlur());

        fallbackPostprocess = new NoPostprocessing();
        this.renderService = renderService;
    }

    public void process(final Framebuffer sceneFbo,
                        final RenderScene scene,
                        final float dt) {
        final List<Postprocess> postprocesses = this.postprocesses.stream()
                .filter(Postprocess::canRender)
                .filter(p -> p.accepts(scene))
                .toList();

        if(postprocesses.isEmpty()){
            fallbackPostprocess.setInput(sceneFbo);
            renderService.render(DrawableData.from(fallbackPostprocess));
            return;
        }

        Framebuffer inputFBO = sceneFbo;
        Framebuffer outputFBO;

        for (int i = 0; i < postprocesses.size(); i++) {
            final Postprocess postprocess = postprocesses.get(i);
            outputFBO = getFramebuffer(postprocesses, i);
            postprocess.setInput(inputFBO);

            if (outputFBO != null) {
                outputFBO.bind();
            }

            postprocess.apply(dt);
            renderService.render(DrawableData.from(postprocess));

            if (outputFBO != null) {
                outputFBO.unbind();
            }

            inputFBO = outputFBO;
        }
    }

    private Framebuffer getFramebuffer(final List<Postprocess> postprocesses,
                                       final int i) {
        if (i == postprocesses.size() - 1) {
            return null;
        } else {
            return i % 2 == 0 ? fbo1 : fbo2;
        }
    }
    public void flushFramebuffers(){
        fbo1.flush();
        fbo2.flush();
    }

}
