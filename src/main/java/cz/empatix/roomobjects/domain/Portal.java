package cz.empatix.roomobjects.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.gamestate.service.GameStateController;
import cz.empatix.physics.domain.PhysicsBody;
import cz.empatix.physics.domain.geometry.Rectangle;
import cz.empatix.physics.events.consumers.KeyPressConsumer;
import cz.empatix.physics.service.PhysicsBodyFactory;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class Portal implements RoomObject {
    private final static Logger LOGGER = LoggerFactory.getLogger(Portal.class);
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 96;
    private final static int DIMENSION_HEIGHT = 90;
    private final Animation animation;
    private final PhysicsBody physicsBody;

    public Portal(final Vector2f position) {
        animation = Animation.builder()
                .setTotal(1, 8)
                .addState(AnimationState.IDLE, 0, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .texture(AssetsService.getTexture("ingame/portal.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);

        physicsBody = PhysicsBodyFactory.createStaticWithoutCollision(new Rectangle(DIMENSION_WIDTH * SCALE, DIMENSION_HEIGHT * SCALE));
        physicsBody.setTranslation(position.x(), position.y());
        physicsBody.addEventConsumer(new KeyPressConsumer(keys -> {
            if(keys.contains(GLFW.GLFW_KEY_E)){
                LOGGER.info("Player activated portal");
                GameStateController.getInstance().setCurrentState(GameStateType.INGAME);
            }
        }));
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(physicsBody.getTranslation()),
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
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public void update(double dt) {
        animation.update();
    }

    @Override
    public boolean canRemove() {
        return false;
    }

    public float getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }

    public float getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }
}
