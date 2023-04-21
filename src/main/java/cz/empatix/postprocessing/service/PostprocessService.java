package cz.empatix.postprocessing.service;

import cz.empatix.postprocessing.domain.Fade;
import cz.empatix.postprocessing.domain.ForwardLightning;
import cz.empatix.postprocessing.domain.MotionBlur;
import cz.empatix.postprocessing.domain.Postprocess;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.RenderData;

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

    public List<Drawable> postProcessing(final float dt){
        final List<Drawable> renderData = new ArrayList<>();
        for (Postprocess postprocess : postprocesses) {
            if(postprocess.shouldApply()){
                postprocess.apply(dt);
                renderData.add(postprocess);
            }
        }
        return renderData;
    }
}
