package cz.empatix.enemies.skills;

import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.AnimationState;
import org.checkerframework.checker.units.qual.A;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class SpitCobweb implements Skill {
    private static final int COOLDOWN = 5000;
    private final Timer timer;
    private final Animation animation;
    private final PhysicsObject physicsObject;
    private final List<CobwebProjectile> cobwebProjectiles;
    private final Supplier<Vector2f> cobwebPositionSupplier;
    private Player target;
    private boolean canShoot;
    public SpitCobweb(final Animation animation,
                      final PhysicsObject physicsObject,
                      final Supplier<Vector2f> cobwebPositionSupplier) {
        timer = new Timer(COOLDOWN);
        timer.start();
        this.animation = animation;
        this.physicsObject = physicsObject;
        cobwebProjectiles = new ArrayList<>();
        this.cobwebPositionSupplier = cobwebPositionSupplier;
    }

    @Override
    public void activate() {
        if (timer.isFinished()) {
            timer.start();
            animation.setState(AnimationState.SPELL0);
            physicsObject.setVelocity(0,0);
            physicsObject.clearForce();
            physicsObject.clearAccumulatedForce();
        }
    }

    public void setTarget(final Player player) {
        this.target = player;
    }

    @Override
    public void update(float dt) {
        if (animation.hasPlayedOnce() && animation.getCurrentState() == AnimationState.SPELL0) {
            canShoot = true;
        }
    }
    @Override
    public void fixedUpdate(float dt) {
        if(canShoot){
            final Vector2f cobwebPosition = cobwebPositionSupplier.get();
            final Vector2f targetPosition = target.getPosition();
            final CobwebProjectile cobwebProjectile = new CobwebProjectile();
            cobwebProjectile.setPosition(cobwebPosition.x - cobwebProjectile.getWidth()/2f, cobwebPosition.y - cobwebProjectile.getWidth()/2f);
            cobwebProjectile.setDirection(cobwebPosition.x - cobwebProjectile.getWidth()/2f, cobwebPosition.y - cobwebProjectile.getWidth()/2f,
                    targetPosition.x, targetPosition.y,
                    0);
            cobwebProjectiles.add(cobwebProjectile);
            canShoot = false;
        }
    }
    public void destroy(){
        cobwebProjectiles.forEach(c -> c.destroy());
    }
    @Override
    public void collectSubRenderables(RenderableState state) {
        state.addAll(cobwebProjectiles);
    }
}
