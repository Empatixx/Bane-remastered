package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.Dash;
import cz.empatix.enemies.skills.Skill;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;

import java.util.List;

public class Ghost extends BaseEnemy{
    private final static int MOVE_FORCE = 4_500_000;
    private final static int SCALE = 4;
    private final static int DIMENSION_WIDTH = (int) (22 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (28 * SCALE);
    private final static int DEFAULT_HEALTH = 11;
    private final static int DEFAULT_DAMAGE = 2;
    private final Skill dash;

    public Ghost(){
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2,5)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.DEATH, 1, 5, DIMENSION_WIDTH, DIMENSION_HEIGHT, 85)
                .texture(AssetsService.getTexture("ingame/ghost.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
        dash = new Dash(() -> {}, () -> {}, force -> moveForce = moveForce + force);
    }
    @Override
    public void update(double dt) {
        super.update(dt);
        dash.update((float)dt);
        dash.activate();
    }
    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
    }

    @Override
    public void onSpawn() {

    }

}
