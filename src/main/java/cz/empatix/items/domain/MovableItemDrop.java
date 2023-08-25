package cz.empatix.items.domain;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import org.joml.Vector2f;

import java.util.Set;

public class MovableItemDrop implements ItemDrop{
    private final ItemDrop itemDrop;
    private final PhysicsBody physicsBody;
    public MovableItemDrop(final ItemDrop itemDrop){
        this.itemDrop = itemDrop;
        this.physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from(1, 1));
        this.physicsBody.setTranslation(itemDrop.getPosition());
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
        return physicsBody.getTranslation();
    }

    @Override
    public void update(float dt) {
        physicsBody.update(dt);
        itemDrop.setPosition(physicsBody.getTranslation().x,physicsBody.getTranslation().y);
    }

    @Override
    public boolean canRemove() {
        return itemDrop.canRemove();
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
}
