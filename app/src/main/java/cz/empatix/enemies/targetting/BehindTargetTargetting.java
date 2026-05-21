package cz.empatix.enemies.targetting;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Set;
import java.util.Vector;
import java.util.function.Supplier;


public class BehindTargetTargetting {
    private final static Logger LOGGER = LoggerFactory.getLogger(BehindTargetTargetting.class);
    private static final int MIN_OFFSET = 50;
    private static final int MAX_OFFSET = 100;
    private static final int STEP = 20;
    private Player target;
    private final Supplier<Vector2f> positionSupplier;
    private final int selfWidth;
    private final int selfHeight;
    public BehindTargetTargetting(final Supplier<Vector2f> positionSupplier,
                          final int width,
                          final int height){
        this.selfWidth = width;
        this.selfHeight = height;
        this.positionSupplier = positionSupplier;
    }
    public void setTarget(final Player target){
        this.target = target;
    }
    public Vector2f getPosition(){
        if (target == null){
            return positionSupplier.get();
        }
        final Vector2f targetPosition = target.getPosition();
        final Vector2f selfPosition = positionSupplier.get();
        final Vector2f direction = new Vector2f(targetPosition).sub(selfPosition).normalize();
        for (int offset = MAX_OFFSET; offset >= MIN_OFFSET; offset -= STEP){
            LOGGER.info("Trying offset {} to target", offset);
            final Vector2f newPosition = new Vector2f(targetPosition).add(direction.mul(selfWidth + offset, selfHeight + offset));
            final List<PhysicsObject> collisions = PhysicsService.getInstance().query(newPosition, new Vector2i(selfWidth, selfHeight),
                    Set.of(CollisionType.ENEMY, CollisionType.ITEM, CollisionType.ENEMY_BULLET, CollisionType.PLAYER_BULLET, CollisionType.ROOM_OBJECT,
                            CollisionType.TRAP));
            if (collisions.isEmpty()){
                LOGGER.info("Found valid position with offset {}", offset);
                return newPosition;
            }
        }
        LOGGER.info("No valid position found, returning self position");
        return selfPosition;
    }

    public boolean canTargetBehind(){
        final Vector2f targetPosition = target.getPosition();
        final Vector2f selfPosition = positionSupplier.get();
        final Vector2f direction = new Vector2f(targetPosition).sub(selfPosition).normalize();
        final Vector2f newPosition = new Vector2f(targetPosition).add(direction.mul(selfWidth + MAX_OFFSET, selfHeight + MAX_OFFSET));
        final List<PhysicsObject> collisions = PhysicsService.getInstance().query(newPosition, new Vector2i(selfWidth, selfHeight),
                Set.of(CollisionType.ENEMY, CollisionType.ITEM, CollisionType.ENEMY_BULLET, CollisionType.PLAYER_BULLET, CollisionType.ROOM_OBJECT,
                        CollisionType.TRAP));
        return collisions.isEmpty();
    }

}
