package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Circle;
import cz.empatix.physics.domain.geometry.Geometry;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.events.domain.CollisionEvent;
import cz.empatix.physics.events.suppliers.CollisionSupplier;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class PhysicsEngine {
    private final static boolean RENDER_COLLISIONS = false;
    private final static Logger LOGGER = LoggerFactory.getLogger(PhysicsEngine.class);
    private static List<PhysicsBody> physicsBodies;
    private final CollisionSupplier collisionSupplier = new CollisionSupplier();

    public PhysicsEngine() {
        physicsBodies = new ArrayList<>();
    }

    public static void add(final PhysicsBody physicsBody) {
        physicsBodies.add(physicsBody);
    }

    public static void clear() {
        physicsBodies.clear();
    }

    public void fixedUpdate(final double dt) {
        physicsBodies.forEach(body -> {
            final Vector2f velocity = body.getVelocity();
            final Vector2f force = body.getForce();
            final float mass = body.getMass();
            final float linearDamping = body.getLinearDamping();

            velocity.x += (force.x / mass) * dt;
            velocity.y += (force.y / mass) * dt;

            body.translate(new Vector2f(velocity).mul((float) dt));

            force.zero();

            velocity.x *= linearDamping;
            velocity.y *= linearDamping;
        });
        for (int i = 0; i < physicsBodies.size(); i++) {

            final PhysicsBody bodyA = physicsBodies.get(i);

            for (int j = i + 1; j < physicsBodies.size(); j++) {
                final PhysicsBody bodyB = physicsBodies.get(j);

                if (bodyA.isImmmovable() && bodyB.isImmmovable()) {
                    continue;
                }
                final CollisionEvent event = collisionSupplier.supply(bodyA, bodyB);

                if(event == null){
                    continue;
                }

                bodyA.consumeEvents(List.of(event));
                bodyB.consumeEvents(List.of(event));

                bodyA.consumeEvents(bodyB.produceEvents(bodyA, bodyB));
                bodyB.consumeEvents(bodyA.produceEvents(bodyA, bodyB));
            }
        }
    }

    public void update(final double dt) {
        physicsBodies.forEach(body -> body.update((float) dt));
        physicsBodies.removeIf(PhysicsBody::canBeRemoved);
    }

    public void debugRender() {
        if(!RENDER_COLLISIONS){
            return;
        }
        physicsBodies.forEach(b -> {
            final Geometry g = b.getGeometry();
            if (g instanceof Rectangle r) {
                GL30.glBegin(GL30.GL_LINE_LOOP);
                GL30.glVertex2f(b.getTranslationX() - Camera.getX(), b.getTranslationY() - Camera.getY());
                GL30.glVertex2f(b.getTranslationX() + r.width() - Camera.getX(), b.getTranslationY() - Camera.getY());
                GL30.glVertex2f(b.getTranslationX() + r.width() - Camera.getX(), b.getTranslationY() + r.height() - Camera.getY());
                GL30.glVertex2f(b.getTranslationX() - Camera.getX(), b.getTranslationY() + r.height() - Camera.getY());
                GL30.glEnd();
            } else if (g instanceof Circle c) {
                GL30.glBegin(GL30.GL_LINE_LOOP);
                for (int i = 0; i < 360; i++) {
                    float degInRad = (float) Math.toRadians(i);
                    GL30.glVertex2f((float) (b.getTranslationX() + Math.cos(degInRad) * c.radius() - Camera.getX()),
                            (float) (b.getTranslationY() + Math.sin(degInRad) * c.radius() - Camera.getY()));
                }
                GL30.glEnd();
            }
        });
    }
}
