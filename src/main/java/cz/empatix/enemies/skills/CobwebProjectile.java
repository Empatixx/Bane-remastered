package cz.empatix.enemies.skills;

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
import org.joml.Vector3f;

public class CobwebProjectile implements Drawable, DamageSource {
    private final static int MOVE_FORCE = 70_000_000;
    private final static int SCALE = 2;
    private final static int BULLET_WIDTH = 32 * SCALE;
    private final static int BULLET_HEIGHT = 32 * SCALE;
    private final InterpolatedPhysicsObject physicsBody;
    private final Animation animation;
    public CobwebProjectile() {
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .texture(AssetsService.getTexture("ingame/cobweb_projectile.tga"))
                .build();
        physicsBody = PhysicsObjectFactory.createInterpolated(BULLET_WIDTH, BULLET_HEIGHT);
        physicsBody.setType(CollisionType.ENEMY_BULLET);
        physicsBody.setLinearDamping(2f);
        physicsBody.setUserData(this);
        physicsBody.onCollision(roomObject -> {
            physicsBody.setVelocity(0, 0);
            physicsBody.destroy();
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
    }
    public void fixedUpdate(final double dt){
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
        return Camera.isInProjection(physicsBody.getTranslation(), BULLET_WIDTH * SCALE, BULLET_HEIGHT * SCALE)
                && !shouldRemove();
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }

    public void setDirection(final float shooterX, final float shooterY,
                             final float targetX, final float targetY,
                             final float inaccuracy) {
        final float angleToTarget = (float) Math.atan2(targetY - shooterY, targetX - shooterX);

        final float finalAngle = angleToTarget + inaccuracy;

        final float forceX = (float) Math.cos(finalAngle) * MOVE_FORCE;
        final float forceY = (float) Math.sin(finalAngle) * MOVE_FORCE;

        physicsBody.applyForce(forceX, forceY);
    }
    public boolean shouldRemove(){
        return false;
    }

    public void destroy() {
        physicsBody.destroy();
    }
    @Override
    public int damage() {
        return 1;
    }
    @Override
    public DamageSourceType type() {
        return DamageSourceType.ENEMY;
    }
}
