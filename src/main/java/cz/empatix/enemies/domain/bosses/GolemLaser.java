package cz.empatix.enemies.domain.bosses;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.newphysics.attributes.*;
import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsObjectFactory;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.dyn4j.geometry.MassType;
import org.joml.Vector2f;
import org.slf4j.LoggerFactory;

public class GolemLaser implements DamageSource, Drawable {
    private final static float SCALE = 1.5f;
    private final InterpolatedPhysicsObject physicsObject;
    private final Animation animation;
    public GolemLaser(){
        this.physicsObject = PhysicsObjectFactory.createInterpolated(810, 45);
        physicsObject.setType(CollisionType.TRAP);
        physicsObject.setUserData(this);
        this.animation = Animation.builder()
                .setTotal(14, 1)
                .addState(AnimationState.IDLE, 13, 1, 810, 45, 100)
                .texture(AssetsService.getTexture("ingame/laserbeam.tga"))
                .scale(SCALE)
                .build();
    }

    public void setPosition(final float x, final float y) {
        this.physicsObject.setInterpolatedTranslation(x, y);
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
        return new RenderData(
                animation.getVertices(physicsObject.getTranslation()),
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
}
