package cz.empatix.enemies.skills;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;

public class EyeWatcherProjectile implements Drawable, DamageSource {

    private static final int TIME_LIVENESS = 5000;
    private final static int MOVE_FORCE = 35_000_000;
    private final static int SCALE = 3;
    private final static int BULLET_WIDTH = 16 * SCALE;
    private final static int BULLET_HEIGHT = 16 * SCALE;
    private final InterpolatedPhysicsObject physicsBody;
    private final Animation animation;
    private final LightSource lightSource;
    private Player target;
    private InterpolatedPosition lastTargetedPosition;
    private final long spawnTime;

    public EyeWatcherProjectile() {
        animation = Animation.builder()
                .setTotal(2, 3)
                .addState(AnimationState.IDLE, 0, 3, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .addState(AnimationState.COLLISION, 1, 3, BULLET_WIDTH, BULLET_HEIGHT, 85)
                .texture(AssetsService.getTexture("ingame/eye_watcher_projectile.tga"))
                .build();
        lightSource = LightningService.getInstance().createLightSource(new Vector3f(250, 240, 0), 30f, 1f);

        physicsBody = PhysicsObjectFactory.createInterpolated(BULLET_WIDTH, BULLET_HEIGHT);
        physicsBody.setType(CollisionType.ENEMY_BULLET);
        physicsBody.setLinearDamping(0f);
        physicsBody.setUserData(this);
        physicsBody.onCollision(roomObject -> {
            animation.setState(AnimationState.COLLISION);
            physicsBody.setVelocity(0, 0);
            physicsBody.destroy();
        });
        spawnTime = System.currentTimeMillis();
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation(), getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
        );
    }

    public void update(final double dt) {
        animation.update();
        lightSource.setPosition(physicsBody.getTranslationX() + BULLET_WIDTH / 2,
                physicsBody.getTranslationY() + BULLET_HEIGHT / 2);
        if (animation.getCurrentState() != AnimationState.COLLISION) {
            return;
        }
        lightSource.setIntensity((1 - (float) animation.getCurrentTextureIndex() / animation.getCurrentStateNumSprites()));
        if (animation.hasPlayedOnce()) {
            lightSource.destroy();
        }
    }

    public void fixedUpdate(final double dt) {
        if (System.currentTimeMillis() - spawnTime > TIME_LIVENESS){
            animation.setState(AnimationState.COLLISION);
            physicsBody.setVelocity(0, 0);
            physicsBody.destroy();
        }
        if (animation.getCurrentState() != AnimationState.COLLISION) {
            setForceToFollow(dt);
        }
    }

    private void setForceToFollow(final double dt){
        if (lastTargetedPosition == null){
            lastTargetedPosition = new InterpolatedPosition();
            lastTargetedPosition.setPosition(target.getX(), target.getY());
        } else {
            lastTargetedPosition.setInterpolatedPosition(target.getX(), target.getY());
            lastTargetedPosition.update((float) (dt/16));
        }
        physicsBody.setVelocity(0,0);
        physicsBody.clearAccumulatedForce();
        physicsBody.clearForce();
        final Vector2f interpolatedPosition = lastTargetedPosition.getInterpolatedPosition();
        final float angleToTarget = (float) Math.atan2(interpolatedPosition.y() + target.getHeight()/2f - physicsBody.getTranslationY() - getHeight() / 2f,
                interpolatedPosition.x()  + target.getWidth()/2f - physicsBody.getTranslationX() - getWidth() / 2f);
        final float forceX = (float) Math.cos(angleToTarget) * MOVE_FORCE;
        final float forceY = (float) Math.sin(angleToTarget) * MOVE_FORCE;
        physicsBody.applyForce(forceX, forceY);
    }

    public int getHeight() {
        return BULLET_HEIGHT;
    }
    public int getWidth(){
        return BULLET_WIDTH;
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(physicsBody.getTranslation(), BULLET_WIDTH * SCALE, BULLET_HEIGHT * SCALE)
                && !shouldRemove();
    }

    public void setPosition(final float x, final float y) {
        physicsBody.setTranslation(x, y);
    }

    public boolean shouldRemove() {
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

    public void followTarget(final Player target) {
        this.target = target;
    }
}
