package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.hud.service.CommonBackground;
import cz.empatix.hud.service.HUDElement;
import cz.empatix.hud.service.image.CommonImage;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.service.Camera;
import cz.empatix.settings.service.SettingsMenuService;
import cz.empatix.ui.CommonButton;
import cz.empatix.ui.UIElement;
import org.lwjgl.glfw.GLFW;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.stream.Stream;

public class MenuState extends GameState {
    private SettingsMenuService settingsMenuService;
    private HUDElement background;
    private List<UIElement> uiElements;
    private List<HUDElement> hudElements;
    private boolean loadedOnce = false;

    public MenuState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.MENU);
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.DEFAULT);
        AudioService.getInstance().play(Soundtrack.MAIN_MENU);
        if (loadedOnce) {
            return;
        }

        settingsMenuService = new SettingsMenuService();

        background = new CommonBackground("menu/bg.png");

        final CommonButton playButton = new CommonButton("Play", Camera.centerX().add(0, 225), 1.5f);
        final CommonButton settingsButton = new CommonButton("Settings", Camera.centerX().add(0, 325), 1.5f);
        final CommonButton exitButton = new CommonButton("Exit", Camera.centerX().add(0, 425), 1.5f);

        playButton.onClick(() -> setCurrentState(GameStateType.LOBBY));
        exitButton.onClick(() -> GLFW.glfwSetWindowShouldClose(WindowHandler.WINDOW, true));
        settingsButton.onClick(() -> settingsMenuService.activate(true));

        final CommonImage title = new CommonImage("menu/logo.tga", Camera.centerX().add(-114, 50), 3f);

        uiElements = List.of(playButton, settingsButton, exitButton);
        hudElements = List.of(title);
        loadedOnce = true;
    }

    @Override
    public void update(final double dt) {
        settingsMenuService.update(dt, mouseX, mouseY);
        uiElements.forEach(uiElement -> uiElement.update(dt, mouseX, mouseY));
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> toRender(final double dt) {
        if(settingsMenuService.isActive()){
            final ArrayList<Renderable> renderables = new ArrayList<>(5);
            renderables.add(background);
            renderables.addAll(settingsMenuService.toRender(dt));
            renderables.add(CursorService.getCursor());
            return renderables;
        }
        final List<? extends Renderable> renderable = Stream.of(
                        List.of(background),
                        List.of(CursorService.getCursor()),
                        uiElements,
                        hudElements,
                        settingsMenuService.toRender(dt)
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
        if(settingsMenuService.isActive()){
            settingsMenuService.mousePressed(button, mouseX, mouseY);
            return;
        }
        uiElements.forEach(uiElement -> uiElement.mousePressed(button, mouseX, mouseY));
    }

    @Override
    public void mouseReleased(int button) {
        if(settingsMenuService.isActive()){
            settingsMenuService.mouseReleased(button, mouseX, mouseY);
            return;
        }
        uiElements.forEach(uiElement -> uiElement.mouseReleased(button, mouseX, mouseY));
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
