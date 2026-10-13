package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;

import java.util.List;
@SpawnableEnemy
public class Snake extends BaseEnemy {
    private final static int MOVE_FORCE = 8_000_000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int) (64 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (64 * SCALE);
    private final static int DEFAULT_HEALTH = 8;
    private final static int DEFAULT_DAMAGE = 3;

    public Snake() {
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 9)
                .addState(AnimationState.IDLE, 0, 9, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .addState(AnimationState.DEATH, 1, 5, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/snake.tga"))
                .build();
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;

        animation.setState(AnimationState.IDLE);
    }




    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
    }

    @Override
    public void onSpawn() {

    }
}
