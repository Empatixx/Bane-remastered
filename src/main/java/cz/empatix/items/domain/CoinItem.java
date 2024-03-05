package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.service.ItemDespawnService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.Set;
import java.util.function.Consumer;

public class CoinItem implements ItemDrop {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static float SCALE = 0.75f;
    private final static int DIMENSION_WIDTH = (int) (33 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (33 * SCALE);
    private final CommonInteractable interactable;
    private final ItemDespawnService itemDespawnService;
    private final Vector2f position;
    private final Animation animation;
    private final int amount;
    private boolean pickedUp = false;

    public CoinItem(final Consumer<Integer> coinAdd) {
        this.amount = 1;
        this.animation = Animation.builder()
                .setTotal(1, 10)
                .addState(AnimationState.IDLE, 0, 10, DIMENSION_WIDTH, DIMENSION_HEIGHT, 75)
                .texture(AssetsService.getTexture("ingame/banecoin_anim.tga"))
                .build();
        position = new Vector2f();
        interactable = new CommonInteractable(pressedKeys -> {
            pickedUp = true;
            coinAdd.accept(amount);
            AUDIO_SERVICE.play(SoundEffect.COIN);
        }, () -> !pickedUp, this::canRemove, position, getHeight(), getWidth());
        itemDespawnService = new ItemDespawnService();
    }

    @Override
    public void update(float dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return pickedUp || itemDespawnService.canDespawn();
    }

    @Override
    public void setPosition(float x, float y) {
        position.set(x, y);
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
        InteractionService.getInstance().removeInteractable(this);
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
        return 3;
    }

    @Override
    public boolean canRender() {
        if (!Camera.isInProjection(position, DIMENSION_WIDTH, DIMENSION_HEIGHT)) {
            return false;
        }
        if(itemDespawnService.canBlink()){
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
