package cz.empatix.physics.service;

import cz.empatix.physics.domain.CollisionResult;
import cz.empatix.physics.domain.geometry.Circle;
import cz.empatix.physics.domain.geometry.Geometry;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.opengl.GL30;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;

public class PhysicsEngine {
    private final static boolean RENDER_COLLISIONS = true;
    private final static Logger LOGGER = LoggerFactory.getLogger(PhysicsEngine.class);
    private static List<PhysicsBody> physicsBodies;
    private final CollisionResolver collisionResolver;
    private final CollisionHandler collisionHandler;
    private final BodyUpdater bodyUpdater;

    public PhysicsEngine() {
        physicsBodies = new ArrayList<>();
        collisionResolver = new CollisionResolver();
        collisionHandler = new CollisionHandler();
        bodyUpdater = new BodyUpdater();
    }

    public static void add(final PhysicsBody physicsBody) {
        physicsBodies.add(physicsBody);
    }

    public void fixedUpdate(final double dt) {
        physicsBodies.forEach(body -> bodyUpdater.update(body, (float) dt));
        for (int i = 0; i < physicsBodies.size(); i++) {

            final PhysicsBody bodyA = physicsBodies.get(i);

            for (int j = i + 1; j < physicsBodies.size(); j++) {
                final PhysicsBody bodyB = physicsBodies.get(j);

                if (bodyA.isImmmovable() && bodyB.isImmmovable()) {
                    continue;
                }

                final CollisionResult result = collisionResolver.resolve(bodyA, bodyB);

                if (result != null) {
                    final Vector2f normal = result.normal();

                    final Vector2f translateA = new Vector2f(normal).mul(-result.depth() / 2);
                    final Vector2f translateB = new Vector2f(normal).mul(result.depth() / 2);

                    if (!bodyA.isImmmovable()) {
                        bodyA.translate(translateA);
                    }
                    if (!bodyB.isImmmovable()) {
                        bodyB.translate(translateB);
                    }
                    collisionHandler.handle(bodyA, bodyB, result);
                }
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
