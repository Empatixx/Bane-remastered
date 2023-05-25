package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.hud.service.Image;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
import cz.empatix.ui.Button;
import cz.empatix.ui.UIElement;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Stream;

public class MenuState extends GameState {
    private HUDElement background;
    private final List<UIElement> uiElements;
    private final List<HUDElement> hudElements;

    public MenuState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.MENU);
        uiElements = new ArrayList<>();
        hudElements = new ArrayList<>();
    }

    @Override
    public void init() {
        background = new CommonBackground("menu/bg.png");

        final Button playButton = new Button("Play", Camera.centerX().add(0, 225), 1.5f);
        playButton.setOnClick(() -> setCurrentState(GameStateType.INGAME));
        uiElements.add(playButton);

        final Button settingsButton = new Button("Settings", Camera.centerX().add(0, 325), 1.5f);
        uiElements.add(settingsButton);

        final Button exitButton = new Button("Exit", Camera.centerX().add(0, 425), 1.5f);
        exitButton.setOnClick(() -> GLFW.glfwSetWindowShouldClose(WindowHandler.WINDOW, true));
        uiElements.add(exitButton);

        hudElements.add(new Image("menu/logo.tga", Camera.centerX().add(-114,50), 3f));

        AudioService.getInstance().play(Soundtrack.MAIN_MENU);
    }

    @Override
    public void update(final double dt) {
        uiElements.forEach(uiElement -> uiElement.update(dt, mouseX, mouseY));
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> toRender(final double dt) {
        final List<? extends Renderable> renderable = Stream.of(
                        List.of(background),
                        uiElements,
                        hudElements
                )
                .flatMap(Collection::stream)
                .toList();
        return List.copyOf(renderable);
    }

    @Override
    public void cleanUp() {

    }

    @Override
    public void keyPressed(int key) {

    }

    @Override
    public void keyReleased(int key) {

    }

    @Override
    public void mousePressed(int button) {
        uiElements.forEach(uiElement -> uiElement.mousePressed(button, mouseX, mouseY));
    }

    @Override
    public void mouseReleased(int button) {

    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
