package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.domain.GunType;
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

import java.util.concurrent.ThreadLocalRandom;
import java.util.function.BiConsumer;
import java.util.function.Function;

public class AmmoItem implements ItemDrop{
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int)(32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(32 * SCALE);
    private final InteractionZone interactionZone;
    private final GunType type;
    private final Animation animation;
    private final float percentAmount;
    private boolean pickedUp = false;

    public AmmoItem(final Function<GunType, Boolean> canPickUp,
                    final BiConsumer<GunType, Float> pickUp){
        this.type = GunType.values()[ThreadLocalRandom.current().nextInt(GunType.values().length)];
        this.percentAmount = ThreadLocalRandom.current().nextFloat(0,0.05f);
        this.animation = Animation.builder()
                .setTotal(1,1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/ammobox.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        this.interactionZone = new InteractionZone(new Vector2f(), new Vector2i(DIMENSION_WIDTH, DIMENSION_HEIGHT), pressedKeys -> {
            if(!pickedUp && canPickUp.apply(type)){
                pickUp.accept(type, percentAmount);
                pickedUp = true;
            }
        });
        InteractionService.getInstance().addZone(interactionZone);
    }

    @Override
    public void update(float dt) {

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
