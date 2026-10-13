package cz.empatix.pets;

import cz.empatix.enemies.targetting.AStarTargetingState;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.enemies.targetting.Targetable;
import org.joml.Vector2f;

public class FollowingStrategyImpl implements FollowingStrategy {
    protected static final int DISTANCE_MIN = 550;
    private AStarTargetting targetting;
    public FollowingStrategyImpl(Pet self) {
        targetting = new AStarTargetting(self::getPosition, self.getWidth(), self.getHeight());
    }

    @Override
    public void fixedUpdate(final float dt) {
        performPathUpdate(dt);
    }

    @Override
    public boolean hasTarget() {
        return targetting.hasTarget();
    }

    @Override
    public void follow(Targetable followTarget) {
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
        return targetting.getDirectionTarget();
    }
}
