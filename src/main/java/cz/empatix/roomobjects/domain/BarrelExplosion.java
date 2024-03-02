package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.EnumSet;
import java.util.Set;

public class BarrelExplosion implements Drawable {
    private static final int SCALE = 2;
    private static final int WIDTH = 64 * SCALE;
    private static final int HEIGHT = 64 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private static final Vector2i AREA_DAMAGE = new Vector2i(WIDTH * 2, HEIGHT * 2);

    public BarrelExplosion() {
        animation = Animation.builder()
                .setTotal(1, 9)
                .addState(AnimationState.IDLE, 0, 9, WIDTH, HEIGHT, 80)
                .texture(AssetsService.getTexture("ingame/barrel_explosion.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
    }

    public void explode() {
        animation.setState(AnimationState.IDLE);
        PhysicsService.getInstance().queryAround(new Vector2f(position).add(getWidth()/2F,getHeight()/2f), AREA_DAMAGE,
                        EnumSet.of(CollisionType.ITEM, CollisionType.TRAP, CollisionType.PLAYER_BULLET, CollisionType.ENEMY_BULLET))
                .forEach(roomObj -> {
                    final Object obj = roomObj.getUserData();
                    if (obj instanceof Damageable) {
                        ((Damageable) obj).hit(DamageSourceType.TRAP, 3);
                    }
                });
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position),
                animation.getTextureId(),
                new CommonBinding(Camera.getProjection())
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 1;
    }

    @Override
    public boolean canRender() {
        return Camera.isInProjection(position, WIDTH, HEIGHT);
    }

    public boolean hasPlayedOnce() {
        return animation.hasPlayedOnce();
    }

    public void update(final double dt) {
        animation.update();
    }

    public void setPosition(final float x, final float y) {
        this.position.set(x, y);
    }

    public int getWidth() {
        return WIDTH;
    }

    public int getHeight() {
        return HEIGHT;
    }
}
