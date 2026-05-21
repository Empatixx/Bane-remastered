package cz.empatix.newphysics.service;

import cz.empatix.core.CollisionHandler;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.dyn4j.dynamics.BodyFixture;
import org.dyn4j.world.BroadphaseCollisionData;
import org.dyn4j.world.ManifoldCollisionData;
import org.dyn4j.world.NarrowphaseCollisionData;
import org.dyn4j.world.listener.CollisionListener;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

/**
 * Fans out narrow-phase collisions to game-supplied {@link CollisionHandler}s.
 * The engine passes each body's user data; it never inspects what handlers do.
 */
public class CollisionHandlerDispatcher implements CollisionListener<PhysicsObject, BodyFixture> {
    private final List<CollisionHandler> handlers = new CopyOnWriteArrayList<>();

    public void add(final CollisionHandler handler) {
        handlers.add(handler);
    }

    @Override
    public boolean collision(BroadphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        return true;
    }

    @Override
    public boolean collision(NarrowphaseCollisionData<PhysicsObject, BodyFixture> collision) {
        if (!handlers.isEmpty()) {
            final Object a = collision.getBody1().getUserData();
            final Object b = collision.getBody2().getUserData();
            for (final CollisionHandler handler : handlers) {
                handler.onCollision(a, b);
            }
        }
        return true;
    }

    @Override
    public boolean collision(ManifoldCollisionData<PhysicsObject, BodyFixture> collision) {
        return true;
    }
}
