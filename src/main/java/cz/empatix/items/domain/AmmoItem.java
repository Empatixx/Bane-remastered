package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.domain.GunType;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.items.service.ItemDespawnService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.BiConsumer;
import java.util.function.Function;

public class AmmoItem implements ItemDrop {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int) (32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (32 * SCALE);
    private final CommonInteractable interactable;
    private final ItemDespawnService itemDespawnService;
    private final GunType type;
    private final Animation animation;
    private final Vector2f position;
    private final float percentAmount;
    private boolean pickedUp = false;

    public AmmoItem(final Function<GunType, Boolean> canPickUp,
                    final BiConsumer<GunType, Float> pickUp) {
        this.type = GunType.values()[RANDOM.nextInt(GunType.values().length)];
        this.percentAmount = RANDOM.nextFloat(0, 0.05f);
        this.animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/ammobox.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        interactable = new CommonInteractable(pressedKeys -> {
            pickUp.accept(type, percentAmount);
            pickedUp = true;
            AUDIO_SERVICE.play(SoundEffect.PICKUP);
        }, () -> !pickedUp && canPickUp.apply(type), this::canRemove, position, getHeight(), getWidth());
        itemDespawnService = new ItemDespawnService();
    }

    @Override
    public void update(float dt) {

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
    public void setDespawnable(final boolean despawn) {
        itemDespawnService.setDespawnTime(despawn ? ItemDespawnService.DEFAULT_ITEM_DESPAWN_TIME : Long.MAX_VALUE);
    }

    @Override
    public void interact(Set<Integer> keys) {

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
        if(itemDespawnService.canBlink()){
            return System.currentTimeMillis() / 100 % 2 == 0;
        }
        return true;
    }
}
