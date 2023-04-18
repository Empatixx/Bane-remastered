package cz.empatix.postprocessing.service;

import cz.empatix.postprocessing.domain.Fade;
import cz.empatix.postprocessing.domain.ForwardLightning;
import cz.empatix.postprocessing.domain.MotionBlur;
import cz.empatix.postprocessing.domain.Postprocess;

import java.util.ArrayList;
import java.util.List;

public class PostprocessService {
    private final List<Postprocess> postprocesses;

    public PostprocessService() {
        postprocesses = new ArrayList<>();
        postprocesses.add(new Fade());
        postprocesses.add(new MotionBlur());
        postprocesses.add(new ForwardLightning());
    }

    public void renderPostprocessing(final float dt) {
        postprocesses.stream()
                .filter(Postprocess::shouldApply)
                .forEach(p -> p.apply(dt));
    }
}
