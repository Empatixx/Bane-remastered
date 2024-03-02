package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import cz.empatix.render.shaderbindings.OutlineBinding;
import org.joml.Vector2f;
import org.joml.Vector4f;

import java.util.List;

public class ExplosiveBarrel implements RoomObject, Damageable {
    private final static int SCALE = 3;
    private final static int OFFSET_X = (22 * SCALE);
    private final static int OFFSET_Y = (22 * SCALE);
    private final static int COLLISSION_WIDTH = (19 * SCALE);
    private final static int COLLISSION_HEIGHT = (21 * SCALE);
    private final static int DIMENSION_WIDTH = 64 * SCALE;
    private final static int DIMENSION_HEIGHT = 64 * SCALE;
    private final Animation animation;
    private final InterpolatedPhysicsObject physicsBody;
    private int health = 4;
    private long lastHit;
    private final BarrelExplosion explosion = new BarrelExplosion();

    public ExplosiveBarrel() {
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .addState(AnimationState.HIT, 0, 3, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .addState(AnimationState.DEATH, 1, 4, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/barrel.tga"))
                .build();
        animation.setState(AnimationState.IDLE);

        physicsBody = PhysicsObjectFactory.createInterpolated(COLLISSION_WIDTH, COLLISSION_HEIGHT);
        physicsBody.setOffset(OFFSET_X, OFFSET_Y);
        physicsBody.setType(CollisionType.ROOM_OBJECT);
        physicsBody.setUserData(this);

    }

    @Override
    public RenderData renderData() {
        final float outline = 1 - (System.currentTimeMillis() - lastHit) / 1000f;
        if (outline > 0) {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation()),
                    animation.getTextureId(),
                    new OutlineBinding(Camera.getProjection(), new Vector2f(getWidth(), getHeight()),
                            animation.getNormalStartPositions(), animation.getNormalEndPositions(), new Vector4f(1, 0, 0, outline))
            );
        } else {
            return new RenderData(
                    animation.getVertices(physicsBody.getTranslation()),
                    animation.getTextureId(),
                    new CommonBinding(Camera.getProjection())
            );
        }
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return Camera.isInProjection(physicsBody.getTranslation(), getWidth(), getHeight());
    }
    @Override
    public int getWidth() {
        return DIMENSION_WIDTH;
    }
    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT;
    }

    @Override
    public Vector2f getPosition() {
        return physicsBody.getTranslation();
    }

    @Override
    public void update(double dt) {
        if (isDead()) {
            explosion.update(dt);
        }
        if (animation.getCurrentState() == AnimationState.DEATH && animation.hasPlayedOnce()) {
            return;
        }
        animation.update();
        switch (animation.getCurrentState()) {
            case HIT -> {
                if (animation.hasPlayedOnce()) {
                    animation.setState(AnimationState.IDLE);
                }
            }
            case DEATH -> {
                if (animation.hasPlayedOnce()) {
                    physicsBody.destroy();
                    animation.setCurrentFrame(animation.getCurrentStateNumSprites() - 1);
                }
            }
        }
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public void hit(DamageSourceType source, int damage) {
        if (source == DamageSourceType.ENEMY) {
            return;
        }
        if (isDead()) {
            return;
        }
        health -= damage;
        lastHit = System.currentTimeMillis();
        animation.setState(AnimationState.HIT);
        if (isDead()) {
            onDeath();
        }
    }

    private void onDeath() {
        physicsBody.destroy();
        animation.setState(AnimationState.DEATH);
        explosion.setPosition(physicsBody.getTranslation().x() + getWidth() / 2f - explosion.getWidth() / 2f,
                physicsBody.getTranslation().y() + getHeight() / 2f - explosion.getHeight() / 2f - 20);
        explosion.explode();
    }

    public boolean isDead() {
        return health <= 0;
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }
    @Override
    public List<Renderable> getSubRenderables() {
        if (isDead() && !explosion.hasPlayedOnce()) {
            return List.of(explosion);
        }
        return List.of();
    }
}
