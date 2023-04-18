package cz.empatix.postprocessing.domain;

public class ForwardLightning implements Postprocess{
    @Override
    public void apply(final float dt) {

    }

    @Override
    public boolean shouldApply() {
        return false;
    }
}
