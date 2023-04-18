package cz.empatix.postprocessing.domain;

public interface Postprocess {
    void apply(final float dt);
    boolean shouldApply();
}
