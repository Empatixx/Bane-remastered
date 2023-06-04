package cz.empatix.physics.service;

import cz.empatix.physics.domain.PhysicsBody;

public class CollisionListener {
    private final PhysicsBody body;

    public CollisionListener(final PhysicsBody body) {
        this.body = body;
    }

    public void onCollision(final PhysicsBody other, final float dt) {
        final int massThis = body.getMass();
        final int massOther = other.getMass();
        if(massThis == Integer.MAX_VALUE && massOther == Integer.MAX_VALUE){
            return;
        }
        if(massThis == Integer.MAX_VALUE){
            other.collision(this.body, dt);
            return;
        }
        if(massOther == Integer.MAX_VALUE){
            this.body.setVelocity(0, 0);
            return;
        }
    }

}
