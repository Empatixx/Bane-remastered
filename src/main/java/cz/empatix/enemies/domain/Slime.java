package cz.empatix.enemies.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.skills.GreenSlimebullet;
import cz.empatix.enemies.skills.SlimeSmash;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;
import java.util.stream.Stream;

public class Slime extends BaseEnemy {
    private final static int MOVE_FORCE = 1_000_000;
    private final static float SCALE = 3f;
    private final static int DIMENSION_WIDTH = (int) (32 * SCALE);
    private final static int DIMENSION_HEIGHT = (int) (24 * SCALE);
    private final static int COLLISION_OFFSET_X = (int) (0 * SCALE);
    private final static int COLLISION_OFFSET_Y = (int) (4 * SCALE);
    private final static Vector2i COLLISION_OFFSET = new Vector2i(COLLISION_OFFSET_X, COLLISION_OFFSET_Y);
    private final static int DEFAULT_HEALTH = 9;
    private final static int DEFAULT_DAMAGE = 1;
    private final SlimeSmash slimeSmash;

    public Slime() {
        super(DIMENSION_WIDTH, DIMENSION_HEIGHT);
        animation = Animation.builder()
                .setTotal(2, 6)
                .addState(AnimationState.IDLE, 0, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 175)
                .addState(AnimationState.DEATH, 1, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 65)
                .texture(AssetsService.getTexture("ingame/slime.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        maxHealth = health = DEFAULT_HEALTH;
        damage = DEFAULT_DAMAGE;
        moveForce = MOVE_FORCE;
        slimeSmash = new SlimeSmash(() -> new Vector2f(getX(), getY()).add(DIMENSION_WIDTH / 2, DIMENSION_HEIGHT / 2), GreenSlimebullet::new);

        physicsBody.setOffset(COLLISION_OFFSET.x(), COLLISION_OFFSET.y());
    }

    @Override
    public void update(double dt) {
        super.update(dt);
        slimeSmash.update((float) dt);
    }



    @Override
    public void fixedUpdate(double dt) {
        super.fixedUpdate(dt);
        slimeSmash.fixedUpdate((float)dt);
        if (isSpawning()){
            return;
        }
        if (isDead()) {
            return;
        }
        slimeSmash.activate();

    }
    @Override
    public void onDeath() {
        animation.setState(AnimationState.DEATH);
    }

    @Override
    public void onSpawn() {

    }


    @Override
    public boolean canRemove() {
        return isDead() && animation.hasPlayedOnce() && slimeSmash.noBullets();
    }


    @Override
    public List<Renderable> getSubRenderables() {
        return Stream.of(super.getSubRenderables(), slimeSmash.getSubRenderables())
                .flatMap(List::stream)
                .toList();
    }

    @Override
    public boolean canRender() {
        return (!isDead() || !animation.hasPlayedOnce()) && Camera.isInProjection(getPosition(), getWidth(), getHeight());
    }

}
