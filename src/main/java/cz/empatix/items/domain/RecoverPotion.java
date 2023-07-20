package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class RecoverPotion implements ItemDrop {
    private final static float SCALE = 2.5f;
    private final static int DIMENSION_WIDTH = (int) (16 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (16 * SCALE);
    private final RecoverType type;
    private final CommonInteractable interactable;
    private final Animation animation;
    private final Vector2f position;
    private final int amount;
    private boolean pickedUp = false;

    public RecoverPotion(final RecoverType type,
                         final Supplier<Boolean> hasFull,
                         final Consumer<Integer> add) {
        this.type = type;
        this.amount = getDefaultAmount();
        this.animation = Animation.builder()
                .setTotal(2, 1)
                .addState(AnimationState.TYPE0, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.TYPE1, 1, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/healing_pot.tga"))
                .build();
        animation.setState(getAnimationState());
        position = new Vector2f();
        interactable = new CommonInteractable(pressedKeys -> {
            add.accept(amount);
            pickedUp = true;
        },() -> !pickedUp && !hasFull.get(), this::canRemove, position, getHeight(), getWidth());
    }

    private AnimationState getAnimationState() {
        return switch (type) {
            case HEALTH -> AnimationState.TYPE0;
            case ARMOR -> AnimationState.TYPE1;
        };
    }

    private int getDefaultAmount() {
        return switch (type) {
            case HEALTH -> 2;
            case ARMOR -> 1;
        };
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
        interactable.setPosition(x, y);
    }

    @Override
    public void interact(final Set<Integer> keys) {
        interactable.interact(keys);
    }

    @Override
    public boolean canInteract() {
        return interactable.canInteract();
    }

    @Override
    public Vector2f getPosition() {
        return position;
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
    }

    @Override
    public ItemRarity getRarity() {
        return ItemRarity.UNCOMMON;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
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
