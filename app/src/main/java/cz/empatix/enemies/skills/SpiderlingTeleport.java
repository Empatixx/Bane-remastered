package cz.empatix.enemies.skills;

import cz.empatix.enemies.domain.PurpleSpiderling;
import cz.empatix.enemies.targetting.BehindTargetTargetting;
import cz.empatix.player.Player;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.domain.AnimationState;
import org.checkerframework.checker.units.qual.A;
import org.joml.Vector2f;

public class SpiderlingTeleport implements Skill {
    private final static int COOLDOWN = 8000;
    private final Animation animation;
    private final PurpleSpiderling spiderling;
    private final Timer cooldown;
    private boolean canTeleport;
    private final BehindTargetTargetting targetting;

    public SpiderlingTeleport(final Animation animation,
                              final PurpleSpiderling spiderling) {
        this.animation = animation;
        cooldown = new Timer(COOLDOWN);
        this.spiderling = spiderling;
        targetting = new BehindTargetTargetting(spiderling::getPosition, spiderling.getWidth(), spiderling.getHeight());
    }

    @Override
    public void activate() {
        if (cooldown.isFinished()) {
            cooldown.start();
            if (!targetting.canTargetBehind()){
                return;
            }
            animation.setState(AnimationState.SPELL0);
            canTeleport = true;
        }
    }

    @Override
    public void update(final float dt) {

    }

    @Override
    public void fixedUpdate(final float dt) {
        if (animation.getCurrentState() == AnimationState.SPELL0 && animation.hasPlayedOnce() && canTeleport) {
            animation.setState(AnimationState.IDLE);
            final Vector2f position = targetting.getPosition();
            spiderling.setPosition(position.x, position.y);
            canTeleport = false;
            animation.setState(AnimationState.SPELL1);
        }
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        // No sub-renderables
    }

    public void setTarget(final Player target) {
        targetting.setTarget(target);
    }
}
