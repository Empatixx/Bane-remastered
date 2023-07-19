package cz.empatix.items.domain;

import cz.empatix.interaction.domain.InteractionZone;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;

public class CoinItem implements ItemDrop{
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int)(32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(32 * SCALE);
    private final InteractionZone interactionZone;
    private final Animation animation;
    private final int amount;
    private boolean pickedUp = false;
    public CoinItem(){
        this.amount = 1;
        this.animation = Animation.builder()
                .setTotal(1,1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture("ingame/coin.tga")
                .build();
    }
    @Override
    public void update(float dt) {

    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public void setPosition(float x, float y) {

    }

    @Override
    public int getWidth() {
        return 0;
    }

    @Override
    public int getHeight() {
        return 0;
    }

    @Override
    public void destroy() {

    }

    @Override
    public RenderData renderData() {
        return null;
    }

    @Override
    public RenderScene renderScene() {
        return null;
    }

    @Override
    public int getZIndex() {
        return 0;
    }

    @Override
    public boolean canRender() {
        return false;
    }
}
