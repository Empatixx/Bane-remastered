package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;

import java.util.List;
@SpawnableEnemy
public class Bat extends BaseEnemy{
    private final static int MOVE_FORCE = 14_500_000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int) (64 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (64 * SCALE);
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 2;
    public Bat(){
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2,4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .addState(AnimationState.DEATH, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/bat.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
    }

    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
    }

    @Override
    public void onSpawn() {

    }
}
