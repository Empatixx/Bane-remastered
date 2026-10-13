package cz.empatix.pets;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.targetting.AStarTargetingState;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.collision.CollisionQueries;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsService;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.EnumSet;
import java.util.List;

public class AttackFollowingStrategy implements FollowingStrategy {
    private static final int AREA_ATTACK = 500;
    private AStarTargetting targetting;
    private Enemy followTarget;
    private final Pet self;
    public AttackFollowingStrategy(Pet self) {
        targetting = new AStarTargetting(self::getPosition, self.getWidth(), self.getHeight());
        this.self = self;
    }

    public void findEnemy() {
        final PhysicsService physicsService = PhysicsService.getInstance();

        final PhysicsObject physicsObject = self.getPhysicsObject();
        final int width = self.getWidth();
        final int height = self.getHeight();

        final Vector2f selfPosition = new Vector2f(physicsObject.getPhysicsTranslation().add(width / 2, height / 2))
                .sub(AREA_ATTACK / 2, AREA_ATTACK / 2);
        final List<PhysicsObject> objects = CollisionQueries.queryType(selfPosition, new Vector2i(AREA_ATTACK, AREA_ATTACK), EnumSet.of(CollisionType.ENEMY));
        if (!objects.isEmpty()) {
            final List<Enemy> closest = objects.stream()
                    .sorted((o1, o2) -> {
                        final Vector2f o1Position = o1.getPhysicsTranslation();
                        final Vector2f o2Position = o2.getPhysicsTranslation();
                        return (int) (o1Position.distanceSquared(selfPosition) - o2Position.distanceSquared(selfPosition));
                    })
                    .filter(o -> o.getUserData() instanceof Enemy)
                    .map(o -> (Enemy) o.getUserData())
                    .toList();
            if (closest.isEmpty()) {
                return;
            }
            follow(closest.get(0));
        }
    }

    @Override
    public void fixedUpdate(final float dt) {
        if (!hasTarget()){
            findEnemy();
            return;
        }
        performPathUpdate(dt);
    }

    @Override
    public boolean hasTarget() {
        return followTarget != null;
    }

    @Override
    public void follow(Targetable followTarget) {
        this.followTarget = (Enemy) followTarget;
        targetting.target(followTarget);
    }

    @Override
    public void update(final float dt) {
        if (followTarget == null){
            return;
        }
        if (followTarget.isDead()) {
            followTarget = null;
            return;
        }
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
