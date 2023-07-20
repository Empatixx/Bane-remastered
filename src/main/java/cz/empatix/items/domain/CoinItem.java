package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.interaction.domain.InteractionZone;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.function.Consumer;

public class CoinItem implements ItemDrop{
    private final static float SCALE = 0.75f;
    private final static int DIMENSION_WIDTH = (int)(33 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(33 * SCALE);
    private final InteractionZone interactionZone;
    private final Animation animation;
    private final int amount;
    private boolean pickedUp = false;
    public CoinItem(final Consumer<Integer> coinAdd){
        this.amount = 1;
        this.animation = Animation.builder()
                .setTotal(1,10)
                .addState(AnimationState.IDLE, 0, 10, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .texture(AssetsService.getTexture("ingame/banecoin_anim.tga"))
                .build();
        this.interactionZone = new InteractionZone(new Vector2f(), new Vector2i(DIMENSION_WIDTH,DIMENSION_HEIGHT), pressedKeys -> {
            if(!pickedUp){
                pickedUp = true;
                coinAdd.accept(amount);
            }
        });
        InteractionService.getInstance().addZone(interactionZone);
    }
    @Override
    public void update(float dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return pickedUp;
    }

    @Override
    public void setPosition(float x, float y) {
        interactionZone.position().set(x, y);
    }

    @Override
    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    @Override
    public void destroy() {
        InteractionService.getInstance().removeZone(interactionZone);
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(interactionZone.position()),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }
}
