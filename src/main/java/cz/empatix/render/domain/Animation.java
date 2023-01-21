package cz.empatix.render.domain;

import java.util.List;

public class Animation {
    private final List<List<Integer>> texturesVbos;
    private int currentState;
    private long lastAnimation;
    private long delayBetweenAnimations;
    private int currentTexture;

    public Animation(final List<List<Integer>> texturesVbos) {
        this.texturesVbos = texturesVbos;
        delayBetweenAnimations = 100;
        setAnimationState(0);
    }

    public void setAnimationState(final int state) {
        currentTexture = 0;
        currentState = state;
        lastAnimation = System.currentTimeMillis();
    }

    public void update() {
        if(delayBetweenAnimations == -1){
            return;
        }
        if (System.currentTimeMillis() - lastAnimation > delayBetweenAnimations) {
            lastAnimation = System.currentTimeMillis();
            currentTexture++;
            if (currentTexture >= texturesVbos.get(currentState).size()) {
                currentTexture = 0;
            }
        }
    }

    public int getCurrentTextureVbo() {
        return texturesVbos.get(currentState).get(currentTexture);
    }

    public void setDelayBetweenAnimations(long delayBetweenAnimations) {
        this.delayBetweenAnimations = delayBetweenAnimations;
    }

}
