package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.generators.ProceduralMapGenerator;
import cz.empatix.map.service.MapLoaderService;
import cz.empatix.newphysics.attributes.PhysicsObject;
import cz.empatix.player.CoinService;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.animation.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Set;

public class Ladder implements RoomObject, Interactable {
    private final static Logger LOGGER = LoggerFactory.getLogger(Ladder.class);
    private final static int SCALE = 3;
    private final static int DIMENSION_WIDTH = 32 * SCALE;
    private final static int DIMENSION_HEIGHT = 32 * SCALE;
    private final Animation animation;
    private final Vector2f position;
    private final AlternativeViewImage animatedImage;
    private final AlternativeViewText text;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final CommonInteractable commonInteractable;
    public Ladder(final MapLoaderService mapLoaderService,
                  final ItemService itemService,
                  final StatisticsService statisticsService,
                  final CoinService coinService) {
        commonInteractable = new CommonInteractable(keys -> {
            if (keys.contains(GLFW.GLFW_KEY_E)){
                LOGGER.info("Loading new map");
                mapLoaderService.clearPreviousMap();

                final ProceduralMapGenerator proceduralMapGenerator = new ProceduralMapGenerator();
                final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate(itemService, mapLoaderService, statisticsService, coinService);
                mapLoaderService.load(mapGenerationResult);
            }
        }, () -> true, () -> false, getPosition(), getWidth(), getHeight());
        animation = Animation.builder()
                .setTotal(1, 1)
                .addState(AnimationState.IDLE, 0, 1, DIMENSION_WIDTH, DIMENSION_HEIGHT, -1)
                .texture(AssetsService.getTexture("ingame/ladder.tga"))
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        animatedImage = new AlternativeViewImage("ingame/ladder_pointer.tga", new Vector2f(), 4, 2, RenderScene.SCENE,
                Camera.getInstance().getProjectionMatrix(), 4, 100);
        text = new AlternativeViewText("Press E to enter next floor", "PixelFJVerdana12pt.ttf", new Vector2f(), 1,
                FontSize.FONTSIZE_22);
        text.setColorRGB(222, 112, 38);
        InteractionService.getInstance().addInteractable(this);
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
        return Camera.getInstance().isInProjection(position, getWidth(), getHeight());
    }

    @Override
    public void update(double dt) {
        animation.update();
        animatedImage.update(dt);
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        state.add(animatedImage);
        if (interactionTracker.isInteracting()) {
            text.collectSubRenderables(state);
        }
    }


    @Override
    public boolean canRemove() {
        return commonInteractable.canRemove();
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
        animatedImage.setPosition(x + getWidth() / 2f - animatedImage.getWidth() / 2f, y - animatedImage.getHeight());
        text.setCenterX(x, x + getWidth());
        text.setPositionY(y + getHeight() + 20);
    }

    @Override
    public void interact(final Set<Integer> keys) {
        commonInteractable.interact(keys);
    }

    @Override
    public boolean canInteract(final Set<Integer> keys) {
        interactionTracker.interaction();
        return commonInteractable.canInteract(keys);
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public int getWidth() {
        return DIMENSION_WIDTH;
    }

    @Override
    public int getHeight() {
        return DIMENSION_HEIGHT;
    }
    @Override
    public void destroy() {
        InteractionService.getInstance().removeInteractable(this);
    }
}
