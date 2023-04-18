package cz.empatix.postprocessing.domain;

public class MotionBlur implements Postprocess{
    @Override
    public void apply(final float dt) {

    }

    @Override
    public boolean shouldApply() {
        return false;
    }
}
