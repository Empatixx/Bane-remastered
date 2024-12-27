package cz.empatix.enemies.skills;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.DamageSource;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.InterpolatedPhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class CobwebProjectile implements Drawable {
    private final static Logger LOGGER = LoggerFactory.getLogger(CobwebProjectile.class);
    private final static int MOVE_FORCE = 150_000_000;
    private final static int SCALE = 2;
    private final static int WIDTH = 32 * SCALE;
    private final static int HEIGHT = 32 * SCALE;
    private final InterpolatedPhysicsObject physicsBody;
    private final Animation animation;
    private double increaseDamping;
    public CobwebProjectile() {
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 4, WIDTH, HEIGHT, 70)
                .texture(AssetsService.getTexture("ingame/cobweb_projectile.tga"))
                .build();
        physicsBody = PhysicsObjectFactory.createInterpolated(WIDTH, HEIGHT);
        physicsBody.setType(CollisionType.TRAP);
        physicsBody.setLinearDamping(1.5f);
        physicsBody.setUserData(this);
        physicsBody.onEnter((other) -> {
            final Object userData = other.getUserData();
            if (userData instanceof Player player) {
                final double currentDamping = other.getLinearDamping();
                if (currentDamping > 15){
                    increaseDamping = 5;
                    return;
                }
                LOGGER.info("Player entered cobweb updating damping from {} by damping {}", currentDamping, increaseDamping);
                increaseDamping = 5;
                other.setLinearDamping(currentDamping + increaseDamping);
            }
        });
        physicsBody.onLeave((other) -> {
            final Object userData = other.getUserData();
            if (userData instanceof Player player) {
                final double currentDamping = other.getLinearDamping();
                LOGGER.info("Player left cobweb updating damping from {} by damping {}", currentDamping, increaseDamping);
                other.setLinearDamping(currentDamping - increaseDamping);
                increaseDamping = 0;
            }
        });
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation(), getZIndex()),
                animation.getTextureId(),
                new CommonBinding(Camera.getInstance().getProjectionMatrix())
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
        return Camera.getInstance().isInProjection(physicsBody.getTranslation(), WIDTH * SCALE, HEIGHT * SCALE)
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
    public int getWidth(){
        return WIDTH;
    }
    public int getHeight(){
        return HEIGHT;
    }
}
