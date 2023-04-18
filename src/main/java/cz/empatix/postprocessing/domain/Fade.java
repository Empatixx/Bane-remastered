package cz.empatix.postprocessing.domain;

public class Fade implements Postprocess{
    private final static float MAX_TRANSPARENCY = 1f;
    private final static float MIN_TRANSPARENCY = 0f;
    private final static float DEFAULT_TWEEN = 0.01f;
    private static float transparency = MIN_TRANSPARENCY;
    private static float tween = DEFAULT_TWEEN;
    private static boolean incremental = true;
    @Override
    public void apply(final float dt) {
        transparency += incremental ? transparency * tween : -transparency * tween;
        fixBounds();
    }

    @Override
    public boolean shouldApply() {
        return false;
    }

    private void fixBounds(){
        if(transparency > MAX_TRANSPARENCY){
            transparency = MAX_TRANSPARENCY;
        } else if(transparency < MIN_TRANSPARENCY){
            transparency = MIN_TRANSPARENCY;
        }
    }

    public void startTransition(){
        transparency = MAX_TRANSPARENCY;
    }
}
