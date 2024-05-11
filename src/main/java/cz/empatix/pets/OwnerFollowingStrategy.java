package cz.empatix.pets;

import cz.empatix.enemies.targetting.AStarTargetingState;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.enemies.targetting.Targetable;
import org.joml.Vector2f;

public class OwnerFollowingStrategy implements FollowingStrategy{
    protected static final int DISTANCE_MIN = 550;
    protected static final int STOP_DISTANCE = 100;
    private AStarTargetting targetting;
    private Targetable followTarget;
    private final Pet self;
    public OwnerFollowingStrategy(Pet self) {
        targetting = new AStarTargetting(self::getPosition, self.getWidth(), self.getHeight());
        this.self = self;
    }
    @Override
    public void fixedUpdate(final float dt) {
        if (followTarget == null){
            return;
        }

        final Vector2f position = followTarget.getPosition();
        final Vector2f selfPosition = self.getPosition();
        final float distance = position.distance(selfPosition);
        if (distance > DISTANCE_MIN) {
            // teleport
            self.setPosition(position.x, position.y);
            performPathUpdate(dt);
            return;
        }

        final AStarTargetingState pathFound = performPathUpdate(dt);
        if (pathFound == AStarTargetingState.NOT_FOUND) {
            // teleport to target
            self.setPosition(followTarget.getX(), followTarget.getY());
            performPathUpdate(dt);
        }
    }

    @Override
    public boolean hasTarget() {
        return followTarget != null;
    }

    @Override
    public void follow(Targetable followTarget) {
        this.followTarget = followTarget;
        targetting.target(followTarget);
    }

    @Override
    public void update(final float dt) {
        targetting.update(dt);
    }


    protected AStarTargetingState performPathUpdate(final float dt) {
        final AStarTargetingState pathFound = targetting.updatePath();
        targetting.update(dt);
        return pathFound;
    }
    @Override

    public Vector2f getDirection() {
        final Vector2f position = followTarget.getPosition();
        final Vector2f selfPosition = self.getPosition();

        final Vector2f targetCenter = new Vector2f(position.x + followTarget.getWidth() / 2, position.y + followTarget.getHeight() / 2);
        final Vector2f selfCenter = new Vector2f(selfPosition.x + self.getWidth() / 2, selfPosition.y + self.getHeight() / 2);

        final float distance = targetCenter.distance(selfCenter);
        if (distance < STOP_DISTANCE) {
            return new Vector2f(0, 0);
        }
        return targetting.getDirectionTarget();
    }
}
