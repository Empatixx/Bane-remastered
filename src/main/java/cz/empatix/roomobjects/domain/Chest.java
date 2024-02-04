package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.Set;

public class Chest implements RoomObject, Interactable {
    private final static int TOTAL_DROP_ITEMS = 5;
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = 16 * SCALE;
    private final static int DIMENSION_HEIGHT = 16 * SCALE;
    private final static int ITEM_DROP_FORCE = 15_000;

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
        physicsBody = PhysicsObjectFactory.create(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        InteractionService.getInstance().addInteractable(this);
        this.chestType = chestType;
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getPhysicsTranslation()),
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
        return Camera.isInProjection(getPosition(), getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        animation.update();
        if (canRemove()){
            dropItems();
        }
    }

    private void dropItems(){
        final ItemFactory itemFactory = itemService.getItemFactory();
        for (int i = 0; i < TOTAL_DROP_ITEMS; i++) {
            final MovableItemDrop itemDrop = itemFactory.createMovableItemDrop();
            itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth()/2f + getWidth()/2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight()/2f + getHeight()/2f);
            itemDrop.setDespawnable(false);

            final float angle = (float)(Math.PI * 2 * (i+1)) / TOTAL_DROP_ITEMS;
            final float forceX = (float) Math.cos(angle) * ITEM_DROP_FORCE;
            final float forceY = (float) Math.sin(angle) * ITEM_DROP_FORCE;

            itemDrop.applyForce(forceX, forceY);
            itemService.add(itemDrop);
        }
        switch (chestType){
            case WEAPON -> {
                final ItemDrop itemDrop = itemFactory.createWeaponDrop();
                itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth()/2f + getWidth()/2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight()/2f + getHeight()/2f);
                itemService.add(itemDrop);
            }
            case ARTEFACT -> {
                final ItemDrop itemDrop = itemFactory.createArtefactDrop();
                itemDrop.setPosition(physicsBody.getPhysicsTranslationX() - itemDrop.getWidth()/2f + getWidth()/2f, physicsBody.getPhysicsTranslationY() - itemDrop.getHeight()/2f + getHeight()/2f);
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
    public void interact(final Set<Integer> keys) {
        opened = true;
        animation.setState(AnimationState.OPEN);
    }

    @Override
    public boolean canInteract() {
        return !opened;
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
