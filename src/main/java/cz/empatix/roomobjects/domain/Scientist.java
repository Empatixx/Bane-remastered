package cz.empatix.roomobjects.domain;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.Animation;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.CommonLightBinding;
import cz.empatix.upgrades.service.PlayerUpgradeService;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

public class Scientist implements Interactable, Drawable {
    private final static float SCALE = 2.5f;
    private final static int DIMENSION_WIDTH = 16;
    private final static int DIMENSION_HEIGHT = 40;
    private final static int OFFSET = 30;
    private final Animation animation;
    private final Vector2f position;
    private final InteractionTracker interactionTracker;
    private final AlternativeViewText alternativeViewText;
    private final PlayerUpgradeService playerUpgradeService;

    public Scientist() {
        animation = Animation.builder()
                .setTotal(1, 6)
                .addState(AnimationState.IDLE, 0, 6, DIMENSION_WIDTH, DIMENSION_HEIGHT, 100)
                .texture(AssetsService.getTexture("ingame/scientist.tga"))
                .scale(SCALE)
                .build();
        animation.setState(AnimationState.IDLE);
        position = new Vector2f();
        interactionTracker = new InteractionTracker();
        alternativeViewText = new AlternativeViewText("Press E to upgrade", "PixelFJVerdana12pt.ttf", new Vector2f(position).add(0, DIMENSION_HEIGHT * SCALE + OFFSET), 1, FontSize.FONTSIZE_22);
        alternativeViewText.setCenterX(position.x(), position.x() + DIMENSION_WIDTH * SCALE);
        alternativeViewText.setColorRGB(222, 112, 38);

        playerUpgradeService = new PlayerUpgradeService();

        InteractionService.getInstance().addInteractable(this);
    }

    @Override
    public List<Renderable> getSubRenderables() {
        if (!interactionTracker.isInteracting()) {
            return List.of();
        }
        final List<Renderable> renderables = new ArrayList<>();
        renderables.addAll(alternativeViewText.getSubRenderables());
        if (playerUpgradeService.isActive()) {
            renderables.addAll(playerUpgradeService.toRender());
        }
        return renderables;
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
        return Camera.getInstance().isInProjection(position, (int) (DIMENSION_WIDTH * SCALE), (int) (DIMENSION_HEIGHT * SCALE));
    }

    public void update(double dt, final float mouseX, final float mouseY) {
        animation.update();
        playerUpgradeService.update(dt, mouseX, mouseY);
        if (playerUpgradeService.isActive() && !interactionTracker.isInteracting()) {
            playerUpgradeService.setActive(false);
        }
    }


    @Override
    public boolean canRemove() {
        return false;
    }

    public void setPosition(final float x, final float y) {
        position.set(x, y);
        alternativeViewText.setPosition(new Vector2f(position).add(0, DIMENSION_HEIGHT * SCALE + OFFSET));
        alternativeViewText.setCenterX(position.x(), position.x() + DIMENSION_WIDTH * SCALE);
    }

    @Override
    public void interact(Set<Integer> keys) {
        interactionTracker.interaction();
        if (keys.contains(GLFW.GLFW_KEY_E)) {
            playerUpgradeService.setActive(!playerUpgradeService.isActive());
        }
    }

    @Override
    public boolean canInteract(final Set<Integer> keys) {
        return true;
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public int getWidth() {
        return (int) (DIMENSION_WIDTH * SCALE);
    }

    @Override
    public int getHeight() {
        return (int) (DIMENSION_HEIGHT * SCALE);
    }

    public void mousePressed(final int button, final float mouseX, final float mouseY) {
        if (playerUpgradeService.isActive()) {
            playerUpgradeService.mousePressed(button, mouseX, mouseY);
        }
    }

    public void cleanUp() {
        playerUpgradeService.save();
    }
}
