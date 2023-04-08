package cz.empatix.physics.service;

import com.google.common.base.Preconditions;
import cz.empatix.physics.domain.Geometry;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.transform.InterpolatedTransform;
import org.joml.Vector2f;

public class PhysicsBodyFactory {
    public static PhysicsBody createDynamic(final Geometry geometry) {
        final PhysicsBody physicsBody = new PhysicsBody(
                new CollisionListener(),
                geometry,
                new InterpolatedTransform(),
                new Vector2f(),
                new Vector2f(),
                geometry.getMass()
        );
        Preconditions.checkArgument(physicsBody.mass() > 0, "Mass must be greater than 0");
        PhysicsEngine.add(physicsBody);
        return physicsBody;
    }
    public static PhysicsBody createStatic(final Geometry geometry){
        final PhysicsBody physicsBody = new PhysicsBody(
                new CollisionListener(),
                geometry,
                new InterpolatedTransform(),
                new Vector2f(),
                new Vector2f(),
                geometry.getMass()
        );
        Preconditions.checkArgument(physicsBody.mass() > 0, "Mass must be greater than 0");
        PhysicsEngine.add(physicsBody);
        return physicsBody;
    }
}
