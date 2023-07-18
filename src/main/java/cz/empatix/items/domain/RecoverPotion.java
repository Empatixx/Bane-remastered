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
import java.util.function.Supplier;

public class RecoverPotion implements ItemDrop{
    private final static float SCALE = 1f;
    private final static int DIMENSION_WIDTH = (int)(32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(32 * SCALE);
    private static final int DEFAULT_AMOUNT = 2;
    private final RecoverType type;
    private final InteractionZone interactionZone;
    private final Animation animation;
    private final int amount;
    private boolean pickedUp = false;

    public RecoverPotion(final RecoverType type,
                         final Supplier<Boolean> hasFull,
                         final Consumer<Integer> add) {
        this.type = type;
        amount = DEFAULT_AMOUNT;
        this.animation = Animation.builder()
                .setTotal(1,1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/healing_pot.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        this.interactionZone = new InteractionZone(new Vector2f(), new Vector2i(DIMENSION_WIDTH, DIMENSION_HEIGHT), pressedKeys -> {
            if(!hasFull.get()){
                add.accept(amount);
                pickedUp = true;
            }
        });
        InteractionService.getInstance().addZone(interactionZone);
    }

    public RecoverType getType() {
        return type;
    }

    public int getAmount() {
        return amount;
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
