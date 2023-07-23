package cz.empatix.pause.service;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.gamestate.service.GameStateController;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.textureatlas.domain.TextureAtlas;
import cz.empatix.ui.TransparentButton;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.Collection;
import java.util.List;
import java.util.stream.Stream;

public class PauseService {
    private final static int BACKGROUND_SCALE = 3;
    private final CommonImage background;
    private final List<TransparentButton> buttons;
    private boolean pause;

    public PauseService() {
        final TextureAtlas.Entry pauseBgTxt = AssetsService.getTexture("ingame/pausebg.tga");
        background = new CommonImage(pauseBgTxt, new Vector2f(WindowHandler.TARGET_WIDTH / 2f - pauseBgTxt.width() * BACKGROUND_SCALE / 2f, WindowHandler.TARGET_HEIGHT / 2f - pauseBgTxt.height() * BACKGROUND_SCALE / 2f), BACKGROUND_SCALE, 2, RenderScene.OVERLAYS);

        final TransparentButton resumeButton = new TransparentButton("Resume", new Vector2f(WindowHandler.TARGET_WIDTH / 2f, 225), 3);
        final TransparentButton settingsButton = new TransparentButton("Statistics", new Vector2f(WindowHandler.TARGET_WIDTH / 2f, 275), 3); //todo;
        final TransparentButton exitButton = new TransparentButton("Exit", new Vector2f(WindowHandler.TARGET_WIDTH / 2f, 325), 3);

        resumeButton.onClick(() -> setPause(false));
        settingsButton.onClick(() -> System.out.println("Statistics"));
        exitButton.onClick(() -> {
            setPause(false);
            GameStateController.getInstance().setCurrentState(GameStateType.MENU);
        });

        buttons = List.of(resumeButton, settingsButton, exitButton);
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        buttons.forEach(button -> button.update(dt, mouseX, mouseY));
    }

    public List<Renderable> toRender(final double dt) {
        if (!pause) {
            return List.of();
        }
        final List<? extends Renderable> drawables = Stream.of(buttons, List.of(background))
                .flatMap(Collection::stream)
                .toList();
        return List.copyOf(drawables);
    }

    public void keyPressed(final int key) {
        if (key == GLFW.GLFW_KEY_ESCAPE) {
            setPause(!pause);
        }
    }

    public void setPause(final boolean pause){
        this.pause = pause;
        GaussianBlur.setEnabled(this.pause);

        final CursorType cursorType = this.pause ? CursorType.DEFAULT : CursorType.CROSSHAIR;
        CursorService.setCursor(cursorType);
    }

    public void mousePressed(final int button, final float mouseX, final float mouseY) {
        if (pause) {
            buttons.forEach(uiElement -> uiElement.mousePressed(button, mouseX, mouseY));
        }
    }

    public boolean isPause() {
        return pause;
    }
}
