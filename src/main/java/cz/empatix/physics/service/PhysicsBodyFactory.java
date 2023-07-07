package cz.empatix.physics.service;

import com.google.common.base.Preconditions;
import cz.empatix.physics.domain.geometry.Geometry;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.events.consumers.CollisionConsumer;

public class PhysicsBodyFactory {
    public static PhysicsBody createDynamic(final Geometry geometry) {
        final PhysicsBody physicsBody = PhysicsBody.createDynamic(geometry);
        Preconditions.checkArgument(physicsBody.getMass() > 0, "Mass must be greater than 0");
        PhysicsService.getInstance().add(physicsBody);
        return physicsBody;
    }
    public static PhysicsBody createStatic(final Geometry geometry){
        final PhysicsBody physicsBody = PhysicsBody.createStatic(geometry);
        physicsBody.setRestitution(0);
        physicsBody.addEventConsumer(new CollisionConsumer());
        Preconditions.checkArgument(physicsBody.getMass() > 0, "Mass must be greater than 0");
        PhysicsService.getInstance().add(physicsBody);
        return physicsBody;
    }
    public static PhysicsBody createStaticWithoutCollision(final Geometry geometry){
        final PhysicsBody physicsBody = PhysicsBody.createStatic(geometry);
        physicsBody.setRestitution(0);
        Preconditions.checkArgument(physicsBody.getMass() > 0, "Mass must be greater than 0");
        PhysicsService.getInstance().add(physicsBody);
        return physicsBody;
    }
}
