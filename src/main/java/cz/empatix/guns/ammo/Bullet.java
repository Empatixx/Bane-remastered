package cz.empatix.guns.ammo;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

public class Bullet implements Drawable, DamageSource {
    private final static int MOVE_FORCE = 1_000_0000;
    private final static int SCALE = 1;
    private final static int BULLET_WIDTH = 16 * SCALE;
    private final static int BULLET_HEIGHT = 16 * SCALE;
    private final InterpolatedPhysicsObject physicsBody;
    private final Animation animation;
    private final LightSource lightSource;
    private int damage;

    public Bullet(final int damage) {
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .addState(AnimationState.COLLISION, 1, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .texture(AssetsService.getTexture("ingame/bullet.tga"))
                .build();
        lightSource = LightningService.getInstance().createLightSource(new Vector3f(255, 0, 0), 25f, 0.75f);
        physicsBody = PhysicsObjectFactory.createInterpolated(BULLET_WIDTH, BULLET_HEIGHT);
        physicsBody.setType(CollisionType.PLAYER_BULLET);
        physicsBody.setLinearDamping(0f);
        physicsBody.setUserData(this);
        this.damage = damage;
        physicsBody.onCollision(() -> {
            physicsBody.destroy();
            animation.setState(AnimationState.COLLISION);
            physicsBody.setVelocity(0, 0);
            this.damage = 0;
        });
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation()),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );
    }

    public void update(final double dt) {
        animation.update();
        lightSource.setPosition(physicsBody.getTranslationX() + BULLET_WIDTH * SCALE / 2,
                physicsBody.getTranslationY() + BULLET_HEIGHT * SCALE / 2);
        final Vector2f velocity = physicsBody.getVelocity();
        if (velocity.x() == 0 || velocity.y() == 0) {
            animation.setState(AnimationState.COLLISION);
            physicsBody.setVelocity(0, 0);
        }
        updateCollisionLightning();
    }

    public void updateCollisionLightning() {
        if (animation.getCurrentState() != AnimationState.COLLISION) {
            return;
        }
        if (animation.hasPlayedOnce()) {
            lightSource.destroy();
            physicsBody.destroy();
        }
        lightSource.setIntensity(0.75f * (1 - (float) animation.getCurrentTextureIndex() / animation.getCurrentStateNumSprites()));
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
        final Vector2f cameraPos = Camera.getPosition();
        final Vector2f cameraDimensions = new Vector2f(Camera.getTargetWidth(), Camera.getTargetHeight());
        final Vector2f position = physicsBody.getPhysicsTranslation();

        return position.x() + BULLET_WIDTH > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + BULLET_HEIGHT > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }

    public void setDirection(final float shooterX, final float shooterY, final float targetX, final float targetY, final float inaccuracy) {
        float angleToTarget = (float) Math.atan2(targetY - BULLET_HEIGHT/2f - shooterY, targetX - BULLET_WIDTH/2f - shooterX);

        float finalAngle = angleToTarget + inaccuracy;

        float forceX = (float) Math.cos(finalAngle) * MOVE_FORCE;
        float forceY = (float) Math.sin(finalAngle) * MOVE_FORCE;

        physicsBody.applyForce(forceX, forceY);
    }

    public boolean shouldRemove() {
        return animation.getCurrentState() == AnimationState.COLLISION && animation.hasPlayedOnce();
    }

    @Override
    public int damage() {
        return damage;
    }

    @Override
    public DamageSourceType type() {
        return DamageSourceType.PLAYER;
    }

    public void destroy() {
    }
}
