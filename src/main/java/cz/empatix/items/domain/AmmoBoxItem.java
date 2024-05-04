package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.service.FloatingEffectService;
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
import java.util.function.*;

public class AmmoBoxItem implements ItemDrop {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static float SCALE = 1.5f;
    private final static int UNIVERSAL_DIMENSION_WIDTH = (int) (32 * SCALE);
    private final static int UNIVERSAL_DIMENSION_HEIGHT = (int) (32 * SCALE);
    private final static int OTHER_DIMENSION_WIDTH = (int) (24 * SCALE);
    private final static int OTHER_DIMENSION_HEIGHT = (int) (24 * SCALE);
    private final CommonInteractable interactable;
    private final ItemDespawnService itemDespawnService;
    private final Animation animation;
    private final Vector2f position;
    private final float percentAmount;
    private boolean pickedUp = false;
    private final AmmoBoxType ammoBoxType;
    private final int width;
    private final int height;
    private final FloatingEffectService floatingEffectService;
    public AmmoBoxItem(final AmmoBoxType ammoBoxType,
                       final Predicate<AmmoBoxType> canPickUp,
                       final BiConsumer<Float, AmmoBoxType> pickUp) {
        this.ammoBoxType = ammoBoxType;
        this.percentAmount = getPercentAmount(ammoBoxType);
        this.width = switch (ammoBoxType) {
            case PISTOL, SUBMACHINE, SHOTGUN -> OTHER_DIMENSION_WIDTH;
            case UNIVERSAL -> UNIVERSAL_DIMENSION_WIDTH;
        };
        this.height = switch (ammoBoxType) {
            case PISTOL, SUBMACHINE, SHOTGUN -> OTHER_DIMENSION_HEIGHT;
            case UNIVERSAL -> UNIVERSAL_DIMENSION_HEIGHT;
        };
        this.animation = getAnimation(ammoBoxType);
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        interactable = new CommonInteractable(pressedKeys -> {
            pickUp.accept(percentAmount, ammoBoxType);
            pickedUp = true;
            AUDIO_SERVICE.play(SoundEffect.PICKUP);
        }, () -> !pickedUp && canPickUp.test(ammoBoxType), this::canRemove, position, getHeight(), getWidth());
        itemDespawnService = new ItemDespawnService();
        floatingEffectService = new FloatingEffectService();
    }
    private Animation getAnimation(final AmmoBoxType ammoBoxType) {
        return switch (ammoBoxType) {
            case PISTOL, SUBMACHINE, SHOTGUN -> Animation.builder()
                    .setTotal(1, 8)
                    .addState(AnimationState.IDLE, 0, 8, width, height, 100)
                    .texture(AssetsService.getTexture(getTexturePath(ammoBoxType)))
                    .build();
            case UNIVERSAL -> Animation.builder()
                    .setTotal(1, 1)
                    .addState(AnimationState.IDLE, 0, 1, width, height, -1)
                    .texture(AssetsService.getTexture(getTexturePath(ammoBoxType)))
                    .build();
        };
    }
    private String getTexturePath(final AmmoBoxType ammoBoxType) {
        return switch (ammoBoxType) {
            case PISTOL -> "ingame/ammobox_pistol.tga";
            case SHOTGUN -> "ingame/ammobox_shotgun.tga";
            case SUBMACHINE -> "ingame/ammobox_pistol.tga";
            case UNIVERSAL -> "ingame/ammobox_universal.tga";
            default -> throw new IllegalArgumentException("Unknown ammo box type: " + ammoBoxType);
        };
    }
    private float getPercentAmount(final AmmoBoxType ammoBoxType) {
        return switch (ammoBoxType) {
            case PISTOL -> 0.10f;
            case SHOTGUN -> 0.25f;
            case SUBMACHINE -> 0.10f;
            case UNIVERSAL -> 0.5f;
            default -> throw new IllegalArgumentException("Unknown ammo box type: " + ammoBoxType);
        };
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
    public boolean isPickedUp() {
        return pickedUp;
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
        return width;
    }

    @Override
    public int getHeight() {
        return height;
    }

    @Override
    public void destroy() {
        InteractionService.getInstance().removeInteractable(this);
    }

    @Override
    public RenderData renderData() {
        final Vector2f floatingPosition = new Vector2f(position).add(0, floatingEffectService.getOffsetY());
        return new RenderData(
                animation.getVertices(floatingPosition),
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
        if (!Camera.isInProjection(position, width, height)) {
            return false;
        }
        if (itemDespawnService.canBlink()) {
            return System.currentTimeMillis() / 100 % 2 == 0;
        }
        return true;
    }
}
