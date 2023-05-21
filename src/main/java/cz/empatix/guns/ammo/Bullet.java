package cz.empatix.guns.ammo;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.Rectangle;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class Bullet implements Drawable {
    private final static int MOVE_FORCE = 1_000_0000;
    private final static int BULLET_WIDTH = 64;
    private final static int BULLET_HEIGHT = 64;
    private final static int SCALE = 1;
    private final PhysicsBody physicsBody;
    private final Animation animation;

    public Bullet() {
        physicsBody = PhysicsBodyFactory.createDynamic(Rectangle.from(BULLET_WIDTH, BULLET_HEIGHT));
        physicsBody.setLinearDamping(1.0f);
        animation = Animation.builder()
                .setTotal(2, 4)
                .addState(AnimationState.IDLE, 0, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .addState(AnimationState.COLLISION, 1, 4, BULLET_WIDTH, BULLET_HEIGHT, 70)
                .texture(AssetsService.getTexture("ingame/bullet.tga"))
                .scale(SCALE)
                .build();
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

        return position.x() + BULLET_WIDTH * SCALE > cameraPos.x()
                && position.x() < cameraPos.x() + cameraDimensions.x()
                && position.y() + BULLET_HEIGHT * SCALE > cameraPos.y()
                && position.y() < cameraPos.y() + cameraDimensions.y();
    }
    public void setPosition(final float x, final float y){
        physicsBody.setTranslation(x,y);
    }
    public void setDirection(float shooterX, float shooterY, float targetX, float targetY, float inaccuracy) {
        float angleToTarget = (float) Math.atan2(targetY - shooterY, targetX - shooterX);

        float randomInaccuracy = inaccuracy * (ThreadLocalRandom.current().nextFloat() - 0.5f);
        float finalAngle = angleToTarget + randomInaccuracy;

        float forceX = (float) Math.cos(finalAngle) * MOVE_FORCE;
        float forceY = (float) Math.sin(finalAngle) * MOVE_FORCE;

        physicsBody.applyForce(forceX, forceY);
    }
}
