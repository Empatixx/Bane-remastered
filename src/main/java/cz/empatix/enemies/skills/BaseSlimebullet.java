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
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;
import org.joml.Vector3f;

public abstract class BaseSlimebullet implements Drawable, DamageSource {
    private final static int MOVE_FORCE = 5_000_000;
    private final static int BULLET_WIDTH = 16;
    private final static int BULLET_HEIGHT = 16;
    private final static int SCALE = 1;
    private final InterpolatedPhysicsObject physicsBody;
    private final Animation animation;
    private final LightSource lightSource;
    public BaseSlimebullet(final String texture, final Vector3f colorLightning) {
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .addState(AnimationState.COLLISION, 1, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .texture(AssetsService.getTexture(texture))
                .scale(SCALE)
                .build();
        lightSource = LightningService.getInstance().createLightSource(colorLightning, 25f, 0.75f);

        physicsBody = PhysicsObjectFactory.createInterpolated(BULLET_WIDTH, BULLET_HEIGHT);
        physicsBody.setType(CollisionType.ENEMY_BULLET);
        physicsBody.setLinearDamping(0f);
        physicsBody.setUserData(this);
        physicsBody.onCollision(roomObject -> {
            animation.setState(AnimationState.COLLISION);
            physicsBody.setVelocity(0, 0);
            physicsBody.setEnabled(false);
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
        if(animation.getCurrentState() != AnimationState.COLLISION){
            return;
        }
        lightSource.setPosition(physicsBody.getTranslationX() + BULLET_WIDTH * SCALE / 2,
                physicsBody.getTranslationY() + BULLET_HEIGHT * SCALE / 2);
        lightSource.setIntensity(0.75f * (1 - (float)animation.getCurrentTextureIndex()/animation.getCurrentStateNumSprites()));
    }
    public void fixedUpdate(final double dt){
        if(animation.getCurrentState() != AnimationState.COLLISION){
            return;
        }
        if(animation.hasPlayedOnce()){
            lightSource.destroy();
            physicsBody.destroy();
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
        return Camera.isInProjection(physicsBody.getTranslation(), BULLET_WIDTH * SCALE, BULLET_HEIGHT * SCALE);
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
        return animation.getCurrentState() == AnimationState.COLLISION && animation.hasPlayedOnce();
    }

    public void destroy() {
        lightSource.destroy();
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
