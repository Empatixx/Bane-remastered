package cz.empatix.items.domain;

import cz.empatix.interaction.service.InteractionService;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.dyn4j.geometry.Mass;
import org.joml.Vector2f;

import java.util.Set;

public class MovableItemDrop implements ItemDrop{
    private final ItemDrop itemDrop;
    private final InterpolatedPhysicsObject physicsBody;
    public MovableItemDrop(final ItemDrop itemDrop){
        this.itemDrop = itemDrop;
        this.physicsBody = PhysicsObjectFactory.createInterpolated(itemDrop.getWidth(), itemDrop.getHeight());
        this.physicsBody.setMass(new Mass(physicsBody.getMass().getCenter(), 1, 0));
        this.physicsBody.setType(CollisionType.ITEM);
        this.physicsBody.setUserData(this);
    }
    @Override
    public void interact(final Set<Integer> keys) {
        itemDrop.interact(keys);
    }

    @Override
    public boolean canInteract() {
        return itemDrop.canInteract();
    }

    @Override
    public Vector2f getPosition() {
        return physicsBody.getPhysicsTranslation();
    }

    @Override
    public void update(float dt) {
        itemDrop.setPosition(physicsBody.getTranslation().x(), physicsBody.getTranslation().y());
    }

    @Override
    public boolean canRemove() {
        return itemDrop.canRemove();
    }

    @Override
    public boolean isPickedUp() {
        return itemDrop.isPickedUp();
    }

    @Override
    public void setPosition(final float x,final float y) {
        physicsBody.setTranslation(x,y);
    }

    @Override
    public void setDespawnable(final boolean despawn) {
        itemDrop.setDespawnable(despawn);
    }

    @Override
    public int getWidth() {
        return itemDrop.getWidth();
    }

    @Override
    public int getHeight() {
        return itemDrop.getHeight();
    }

    @Override
    public void destroy() {
        itemDrop.destroy();
        physicsBody.destroy();
        InteractionService.getInstance().removeInteractable(this);
    }

    @Override
    public RenderData renderData() {
        return itemDrop.renderData();
    }

    @Override
    public RenderScene renderScene() {
        return itemDrop.renderScene();
    }

    @Override
    public int getZIndex() {
        return itemDrop.getZIndex();
    }

    @Override
    public boolean canRender() {
        return itemDrop.canRender();
    }
    public void applyForce(final float x, final float y) {
        physicsBody.applyForce(x,y);
    }
    @Override
    public DropSource getSource() {
        return itemDrop.getSource();
    }
}
