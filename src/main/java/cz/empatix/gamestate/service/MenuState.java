package cz.empatix.gamestate.service;

import com.google.common.collect.Streams;
import cz.empatix.background.Background;
import cz.empatix.background.CommonBackground;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.hud.HUDElement;
import cz.empatix.hud.Image;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.ui.Button;
import cz.empatix.ui.UIElement;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public class MenuState extends GameState {
    private Background background;
    private final List<UIElement> uiElements;
    private final List<HUDElement> hudElements;

    public MenuState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.MENU);
        uiElements = new ArrayList<>();
        hudElements = new ArrayList<>();
    }

    @Override
    public void init() {
        background = new CommonBackground("Menu/bg.png");

        final Button playButton = new Button("Play", Camera.centerX().add(0, 150));
        playButton.setOnClick(() -> setCurrentState(GameStateType.INGAME));
        uiElements.add(playButton);

        final Button settingsButton = new Button("Settings", Camera.centerX().add(0, 225));
        uiElements.add(settingsButton);

        final Button exitButton = new Button("Exit", Camera.centerX().add(0, 300));
        exitButton.setOnClick(() -> GLFW.glfwSetWindowShouldClose(WindowHandler.WINDOW, true));
        uiElements.add(exitButton);


        hudElements.add(new Image("Menu/logo.tga", Camera.centerX().add(0, 62.5f), 200, 100));
    }

    @Override
    public void update(final double dt) {
        uiElements.forEach(uiElement -> uiElement.update(dt, mouseX, mouseY));
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return Streams.concat(
                Stream.of(background),
                hudElements.stream(),
                uiElements.stream()
        ).toList();
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
