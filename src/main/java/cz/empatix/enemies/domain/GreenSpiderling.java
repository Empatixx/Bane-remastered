package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.EnumSet;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;

@SpawnableEnemy(minFloor = 2)
public class GreenSpiderling extends BaseEnemy {
    private static final int SCALE = 3;
    private static final int HEIGHT = 20 * SCALE;
    private static final int WIDTH = 32 * SCALE;
    private static final int DEFAULT_DAMAGE = 2;
    private static final int DEFAULT_HEALTH = 5;
    private static final int MOVE_FORCE = 5_500_000;
    private static final int HEAL_RADIUS = 200;
    private static final Set<Class<?>> SPIDERLING_CLAZZES = Set.of(GreenSpiderling.class, PurpleSpiderling.class, RedSpiderling.class);
    public GreenSpiderling() {
        super(WIDTH, HEIGHT);
        animation = Animation.builder()
                .setTotal(2,5)
                .addState(AnimationState.IDLE, 0, 5, WIDTH, HEIGHT, 120)
                .addState(AnimationState.DEATH, 1, 3, WIDTH, HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/green_spiderling.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
    }
    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
        final Vector2f position = new Vector2f(getX() + getWidth() / 2, getY() + getHeight() / 2);
        final List<PhysicsObject> physicsObjects = PhysicsService.getInstance().query(position, new Vector2i(HEAL_RADIUS, HEAL_RADIUS),
                EnumSet.noneOf(CollisionType.class));
        physicsObjects.forEach(physicsObject -> {
            final Object userData = physicsObject.getUserData();
            if (userData instanceof BaseEnemy enemy){
                if (SPIDERLING_CLAZZES.contains(enemy.getClass())){
                    enemy.heal(1);
                }
            }
        });
    }

    public void removeSpawning() {
        spawnTime -= SPAWN_INVINCIBILITY;
    }

    @Override
    public void onSpawn() {

    }
}
