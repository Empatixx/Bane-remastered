package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.newphysics.attributes.DamageSourceType;
import cz.empatix.newphysics.attributes.Damageable;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsEngine;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
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
    private static final AudioService AUDIO_SERVICE = AudioService.getInstance();
    private static final int SCALE = 2;
    private static final int WIDTH = 64 * SCALE;
    private static final int HEIGHT = 64 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private final static Vector2i DAMAGE_SIZE = new Vector2i(ExplosiveBarrel.COLLISSION_WIDTH + 128, ExplosiveBarrel.COLLISSION_HEIGHT + 96);
    private final static Vector2i DAMAGE_OFFSET = new Vector2i(64, 48);

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
        PhysicsService.getInstance().query(new Vector2f(position).sub(DAMAGE_OFFSET.x(), DAMAGE_OFFSET.y()), DAMAGE_SIZE,
                        EnumSet.of(CollisionType.ITEM, CollisionType.TRAP, CollisionType.PLAYER_BULLET, CollisionType.ENEMY_BULLET))
                .forEach(roomObj -> {
                    final Object obj = roomObj.getUserData();
                    if (obj instanceof Damageable) {
                        ((Damageable) obj).hit(DamageSourceType.TRAP, 3);
                    }
                });
        AUDIO_SERVICE.play(SoundEffect.EXPLOSION);
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
