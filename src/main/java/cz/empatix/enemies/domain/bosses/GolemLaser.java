package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.enemies.targetting.RotationTargetting;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.dyn4j.geometry.Mass;
import org.dyn4j.geometry.MassType;
import org.dyn4j.geometry.Vector2;
import org.joml.Vector2f;
import org.slf4j.LoggerFactory;

import java.util.List;

public class GolemLaser implements DamageSource, Drawable {
    private final static float SCALE = 1.5f;
    private static final int WIDTH = (int) (810 * SCALE);
    private static final int HEIGHT = (int) (45 * SCALE);
    private static final int EYE_OFFSET_X = (int) (22 * SCALE);
    private final InterpolatedPhysicsObject physicsObject;
    private final RotationTargetting rotationTargetting;
    private final Animation animation;

    public GolemLaser() {
        this.physicsObject = PhysicsObjectFactory.createInterpolated(WIDTH, HEIGHT);
        physicsObject.setType(CollisionType.NONE);
        physicsObject.setUserData(this);
        physicsObject.setEnabled(false);

        this.animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, WIDTH, HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/laserbeam.tga"))
                .build();

        rotationTargetting = new RotationTargetting(physicsObject::getPhysicsTranslation, WIDTH, HEIGHT);
    }

    public void setPosition(final float x, final float y) {
        this.physicsObject.setPhysicsTranslation(x, y);
    }

    public void fixedUpdate(final float dt){
        physicsObject.getLocalCenter().set(EYE_OFFSET_X, HEIGHT/2f);
        physicsObject.rotateAboutCenter(0.01f);

    }
    public void setTarget(final Player player){
        this.rotationTargetting.target(player);
    }


    @Override
    public int damage() {
        return 1;
    }

    @Override
    public DamageSourceType type() {
        return DamageSourceType.ENEMY;
    }

    @Override
    public RenderData renderData() {
        final List<Vertex> vertices = animation.getVertices(physicsObject.getTranslation());
        final Vector2 rotationPointPhysics = physicsObject.getWorldCenter();
        final Vector2f rotation = new Vector2f((float) rotationPointPhysics.x, (float) rotationPointPhysics.y);
        vertices.forEach(vertex -> vertex.rotateAround(rotation, physicsObject.getRotation()));
        return new RenderData(
                vertices,
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
        return 4;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    public void destroy() {
        physicsObject.destroy();
    }
}
