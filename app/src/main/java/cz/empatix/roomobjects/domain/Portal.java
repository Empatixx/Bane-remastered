package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.gamestate.service.GameStateController;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.domain.InteractEntity;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.joml.Vector3f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Set;

public class Portal implements RoomObject, Interactable {
    private final static Logger LOGGER = LoggerFactory.getLogger(Portal.class);
    private final static int SCALE = 2;
    private final static int DIMENSION_WIDTH = 96;
    private final static int DIMENSION_HEIGHT = 90;
    private final Animation animation;
    private final Vector2f position;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final AlternativeViewText alternativeViewText;
    private final LightSource lightSource;
    public Portal(final Vector2f position) {
        this.position = position;
        animation = Animation.builder()
                .setTotal(1, 8)
                .addState(AnimationState.IDLE, 0, 8, DIMENSION_WIDTH, DIMENSION_HEIGHT, 125)
                .texture(AssetsService.getTexture("ingame/portal.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        InteractionService.getInstance().addInteractable(this);
        alternativeViewText = new AlternativeViewText("Press E to enter the portal", "PixelFJVerdana12pt.ttf", new Vector2f(position).add(0,DIMENSION_HEIGHT*SCALE),1,FontSize.FONTSIZE_22);
        alternativeViewText.setCenterX(position.x(),position.x() + DIMENSION_WIDTH*SCALE);
        alternativeViewText.setColorRGB(222, 112, 38);
        lightSource = LightningService.getInstance().createLightSource(new Vector3f(128,0,128),50,2.5f);
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        if (interactionTracker.isInteracting()) {
            alternativeViewText.collectSubRenderables(state);
        }
    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                animation.getVertices(position, getZIndex()),
                animation.getTextureId(),
                new CommonLightBinding()
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
        lightSource.setPosition(position.x() + DIMENSION_WIDTH * SCALE / 2, position.y() + DIMENSION_HEIGHT * SCALE / 2);

    }

    @Override
    public boolean canRemove() {
        return false;
    }

    @Override
    public void interact(final Set<Integer> keys) {
        if(keys.contains(GLFW.GLFW_KEY_E)){
            GameStateController.getInstance().setCurrentState(GameStateType.INGAME);
        }
    }

    @Override
    public boolean canInteract(final Set<Integer> keys) {
        interactionTracker.interaction();
        return true;
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }
    @Override
    public int getWidth() {
        return DIMENSION_WIDTH * SCALE;
    }
    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT * SCALE;
    }
}
