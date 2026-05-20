package cz.empatix.pets;

import cz.empatix.enemies.targetting.AStarTargetingState;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Set;

import static cz.empatix.newphysics.service.CollisionType.ENEMY;

public class OwnerFollowingStrategy implements FollowingStrategy {
    private static final Logger LOGGER = LoggerFactory.getLogger(OwnerFollowingStrategy.class);
    protected static final int DISTANCE_MIN = 550;
    protected static final int STOP_DISTANCE = 100;
    private final AStarTargetting targetting;
    private Targetable followTarget;
    private final Pet self;

    public OwnerFollowingStrategy(Pet self) {
        targetting = new AStarTargetting(self::getPosition, self.getWidth(), self.getHeight());
        this.self = self;
    }

    @Override
    public void fixedUpdate(final float dt) {
        if (followTarget == null) {
            return;
        }

        final Vector2f position = followTarget.getPosition();
        final Vector2f selfPosition = self.getPosition();
        final float distance = position.distance(selfPosition);
        if (distance > DISTANCE_MIN) {
            teleportToTarget(dt);
            return;
        }

        final AStarTargetingState pathFound = performPathUpdate(dt);
        if (pathFound == AStarTargetingState.NOT_FOUND) {
            teleportToTarget(dt);
        }
    }

    private void teleportToTarget(final float dt) {
        final float x = followTarget.getX() + followTarget.getWidth() / 2f - self.getWidth() / 2f;
        final float y = followTarget.getY() + followTarget.getHeight() / 2f - self.getHeight() / 2f;

        // check if player is not stuck in some collision or pet won't be stuck in collision
        final PhysicsService physicsService = PhysicsService.getInstance();
        final boolean collision = physicsService.queryExists(followTarget.getPosition(), new Vector2i(followTarget.getWidth(), followTarget.getHeight()),
                Set.of(CollisionType.PLAYER, CollisionType.PLAYER_BULLET, ENEMY)) || physicsService.queryExists(new Vector2f(x, y), new Vector2i(self.getWidth(), self.getHeight()),
                        Set.of(CollisionType.PLAYER, CollisionType.PLAYER_BULLET, ENEMY));
        if (collision) {
            LOGGER.error("Player is stuck in collision, cannot teleport to target");
            return;
        }

        self.setPosition(x, y);
        LOGGER.info("Teleporting {} to target to location {}, {}", followTarget, x, y);
        final AStarTargetingState aStarTargetingState = performPathUpdate(dt);
        if (aStarTargetingState != AStarTargetingState.FOUND) {
            LOGGER.error("Path not found after teleporting to target, state: {}", aStarTargetingState);
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
