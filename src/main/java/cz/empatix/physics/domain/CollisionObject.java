package cz.empatix.physics.domain;

import cz.empatix.render.domain.InterpolatedPosition;
import org.dyn4j.dynamics.Body;
import org.dyn4j.geometry.Transform;

import java.util.function.Consumer;

public class CollisionObject extends Body {
    private final Consumer<CollisionObject> postPhysicsUpdate;
    public CollisionObject(final InterpolatedPosition position){
        this(body -> {
            final Transform transform = body.getTransform();
            position.setPosition((float)transform.getTranslationX(), (float)transform.getTranslationY());
        });
    }
    public CollisionObject(final Consumer<CollisionObject> postPhysicsUpdate) {
        this.postPhysicsUpdate = postPhysicsUpdate;
    }
    public CollisionObject(){
        this((Consumer<CollisionObject>) null);
    }

    public void setPosition(final float x, final float y) {
        this.translateToOrigin();
        this.translate(x,y);
    }

    public void postPhysicsUpdate() {
        if(postPhysicsUpdate != null){
            postPhysicsUpdate.accept(this);
        }
    }

    public void setVelocity(final float x,final float y) {
        this.setLinearVelocity(x,y);
    }
}
