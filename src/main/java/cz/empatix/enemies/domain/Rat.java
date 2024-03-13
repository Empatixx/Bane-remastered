package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;

import java.util.List;

public class Rat extends BaseEnemy{
    private final static int MOVE_FORCE = 4_500000;
    private final static float SCALE = 1.5f;
    private final static int DIMENSION_WIDTH = (int)(85 * SCALE);
    private final static int DIMENSION_HEIGHT = (int)(37* SCALE);
    private final static int DEFAULT_HEALTH = 12;
    private final static int DEFAULT_DAMAGE = 2;
    public Rat(){
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2,4)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 120)
                .addState(AnimationState.DEATH, 1, 3, DIMENSION_WIDTH, DIMENSION_HEIGHT, 150)
                .texture(AssetsService.getTexture("ingame/rat.tga"))
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
