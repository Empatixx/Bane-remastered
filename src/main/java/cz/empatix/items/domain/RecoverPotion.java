package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.service.FloatingEffectService;
import cz.empatix.items.service.ItemDespawnService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;

import java.util.Set;
import java.util.function.Consumer;
import java.util.function.Supplier;

public class RecoverPotion implements ItemDrop {
    private final static float SCALE = 2.5f;
    private final static float LARGE_SCALE = 2f;
    private final static int DIMENSION_WIDTH = (int) (24 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (24 * SCALE);
    private final static int LARGE_DIMENSION_WIDTH = (int) (32 * LARGE_SCALE);
    private final static int LARGE_DIMENSION_HEIGHT = (int) (32 * LARGE_SCALE);
    private final RecoverType type;
    private final RecoverSize size;
    private final CommonInteractable interactable;
    private final ItemDespawnService itemDespawnService;
    private final Animation animation;
    private final Vector2f position;
    private final int amount;
    private boolean pickedUp = false;
    private final FloatingEffectService floatingEffectService;

    public RecoverPotion(final RecoverSize size,
                         final RecoverType type,
                         final Supplier<Boolean> hasFull,
                         final Consumer<Integer> add) {
        this.type = type;
        this.size = size;
        this.amount = getDefaultAmount();
        animation = getAnimation(size);
        animation.setState(getAnimationState());
        position = new Vector2f();
        interactable = new CommonInteractable(pressedKeys -> {
            add.accept(amount);
            pickedUp = true;
            AUDIO_SERVICE.play(SoundEffect.PICKUP);
        }, () -> !pickedUp && !hasFull.get(), this::canRemove, position, getHeight(), getWidth());
        itemDespawnService = new ItemDespawnService();
        floatingEffectService = new FloatingEffectService();
    }

    private Animation getAnimation(RecoverSize size) {
        return switch (size) {
            case SMALL -> Animation.builder()
                    .setTotal(2, 4)
                    .addState(AnimationState.TYPE0, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                    .addState(AnimationState.TYPE1, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                    .texture(AssetsService.getTexture(getTexture()))
                    .build();
            case LARGE -> Animation.builder()
                    .setTotal(2, 1)
                    .addState(AnimationState.TYPE0, 0, 1, LARGE_DIMENSION_WIDTH, LARGE_DIMENSION_HEIGHT, -1)
                    .addState(AnimationState.TYPE1, 1, 1, LARGE_DIMENSION_WIDTH, LARGE_DIMENSION_HEIGHT, -1)
                    .texture(AssetsService.getTexture(getTexture()))
                    .build();
            default -> throw new IllegalStateException("Unexpected value: " + size);
        };
    }

    private AnimationState getAnimationState() {
        return switch (type) {
            case HEALTH -> AnimationState.TYPE0;
            case ARMOR -> AnimationState.TYPE1;
        };
    }

    private String getTexture() {
        return switch (size) {
            case SMALL -> "ingame/healing_pot.tga";
            case LARGE -> "ingame/large_healing_pot.tga";
        };
    }

    private int getDefaultAmount() {
        return switch (size) {
            case SMALL -> 1;
            case LARGE -> 4;
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
        animation.update();
        floatingEffectService.update(dt);
    }

    @Override
    public boolean canRemove() {
        return pickedUp || itemDespawnService.canDespawn();
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
    public boolean canInteract(final Set<Integer> keys) {
        return interactable.canInteract(keys);
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public int getWidth() {
        return switch (size) {
            case SMALL -> DIMENSION_WIDTH;
            case LARGE -> LARGE_DIMENSION_WIDTH;
        };
    }

    @Override
    public int getHeight() {
        return switch (size) {
            case SMALL -> DIMENSION_HEIGHT;
            case LARGE -> LARGE_DIMENSION_HEIGHT;
        };
    }

    @Override
    public void destroy() {
        InteractionService.getInstance().removeInteractable(this);
    }

    @Override
    public RenderData renderData() {
        final Vector2f floatingPosition = new Vector2f(position).add(0, floatingEffectService.getOffsetY());
        return new RenderData(
                animation.getVertices(floatingPosition, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        if (!Camera.getInstance().isInProjection(position, DIMENSION_WIDTH, DIMENSION_HEIGHT)) {
            return false;
        }
        if (itemDespawnService.canBlink()) {
            return System.currentTimeMillis() / 100 % 2 == 0;
        }
        return true;
    }

    @Override
    public void setDespawnable(final boolean despawn) {
        itemDespawnService.setDespawnTime(despawn ? ItemDespawnService.DEFAULT_ITEM_DESPAWN_TIME : Long.MAX_VALUE);
    }

    @Override
    public boolean isPickedUp() {
        return pickedUp;
    }
}
