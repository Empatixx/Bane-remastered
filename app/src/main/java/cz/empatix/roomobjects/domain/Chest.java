package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.sound.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;

public class Chest implements RoomObject {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final static int TOTAL_DROP_ITEMS = 5;
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 16 * SCALE;
    private final static int DIMENSION_HEIGHT = 16 * SCALE;
    private final static int ITEM_DROP_FORCE = 20_000;
    private static final int CHEST_PUSH_FORCE = 250_000_000;
    private final Animation animation;
    private final PhysicsObject physicsBody;
    private final ItemService itemService;
    private boolean opened = false;
    private final ChestType chestType;

    public Chest(final ItemService itemService,
                 final ChestType chestType) {
        this.itemService = itemService;
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.OPEN, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .texture(AssetsService.getTexture("ingame/chest.tga"))
                .build();
        physicsBody = PhysicsObjectFactory.createInterpolated(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        physicsBody.setType(CollisionType.ROOM_OBJECT);
        physicsBody.onCollision(other -> {
            if (opened){
                return;
            }
            if (other.getType().equals(CollisionType.PLAYER)) {
                opened = true;
                animation.setState(AnimationState.OPEN);
                pushBy(other);
            }
        });
        this.chestType = chestType;
    }
    private void pushBy(final PhysicsObject other){
        final Vector2f velocity = physicsBody.getVelocity();
        final Vector2f otherVelocity = other.getVelocity();

        Vector2f relativeVelocity = otherVelocity.sub(velocity);

        Vector2f directionNormalized = relativeVelocity.normalize();

        float angle = (float) Math.atan2(directionNormalized.y, directionNormalized.x);

        final Vector2f force = new Vector2f((float) Math.cos(angle) * CHEST_PUSH_FORCE, (float) Math.sin(angle) * CHEST_PUSH_FORCE);

        physicsBody.applyForce(force.x(), force.y());
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getPhysicsTranslation(), getZIndex()),
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
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(getPosition(), getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        animation.update();
        if (canRemove()) {
            dropItems();
        }
    }

    private void dropItems() {
        AUDIO_SERVICE.play(SoundEffect.OPEN_CHEST);
        final ItemFactory itemFactory = itemService.getItemFactory();
        for (int i = 0; i < TOTAL_DROP_ITEMS; i++) {
            final MovableItemDrop itemDrop = itemFactory.createMovableItemDrop();
            itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth() / 2f + getWidth() / 2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight() / 2f + getHeight() / 2f);
            itemDrop.setDespawnable(false);

            final float angle = (float) (Math.PI * 2 * (i + 1)) / TOTAL_DROP_ITEMS;
            final float forceX = (float) Math.cos(angle) * ITEM_DROP_FORCE;
            final float forceY = (float) Math.sin(angle) * ITEM_DROP_FORCE;

            itemDrop.applyForce(forceX, forceY);
            itemService.add(itemDrop);
        }
        switch (chestType) {
            case WEAPON -> {
                final ItemDrop itemDrop = itemFactory.createWeaponDrop();
                itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth() / 2f + getWidth() / 2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight() / 2f + getHeight() / 2f);
                itemService.add(itemDrop);
            }
            case ARTEFACT -> {
                final ItemDrop itemDrop = itemFactory.createArtefactDrop();
                itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth() / 2f + getWidth() / 2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight() / 2f + getHeight() / 2f);
                itemService.add(itemDrop);
            }
        }
    }

    @Override
    public boolean canRemove() {
        return opened && animation.hasPlayedOnce();
    }


    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }

    @Override
    public Vector2f getPosition() {
        return physicsBody.getPhysicsTranslation();
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
        physicsBody.destroy();
    }
}
