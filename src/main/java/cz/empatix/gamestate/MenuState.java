package cz.empatix.gamestate;

import com.google.common.collect.Streams;
import cz.empatix.render.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.background.Background;
import cz.empatix.render.background.CommonBackground;
import cz.empatix.render.hud.HUDElement;
import cz.empatix.render.hud.Image;
import cz.empatix.render.ui.Button;
import cz.empatix.render.ui.UIElement;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Stream;

public class MenuState extends GameState {
    private Background background;
    private final List<UIElement> uiElements;
    private final List<HUDElement> hudElements;

    protected MenuState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.MENU);
        uiElements = new ArrayList<>();
        hudElements = new ArrayList<>();
    }

    @Override
    void init() {
        background = new CommonBackground("Menu/bg.png");
        uiElements.add(new Button("Play", Camera.centerX().add(0,150)));
        uiElements.add(new Button("Settings", Camera.centerX().add(0,225)));
        uiElements.add(new Button("Exit", Camera.centerX().add(0,300)));

        hudElements.add(new Image("Menu/logo.tga",
                "image-title.json", Camera.centerX().add(0,62.5f)));
    }

    @Override
    void update(final double dt) {
        uiElements.forEach(uiElement -> uiElement.update(dt, mouseX, mouseY));
    }

    @Override
    List<Drawable> toRender(final double dt) {
        return Streams.concat(
                Stream.of(background),
                hudElements.stream(),
                uiElements.stream()
        ).toList();
    }

    @Override
    void cleanUp() {

    }

    @Override
    void keyPressed(int key) {

    }

    @Override
    void keyReleased(int key) {

    }

    @Override
    void mousePressed(int button) {

    }

    @Override
    void mouseReleased(int button) {

    }

    @Override
    void mouseScrolled(double xoffset, double yoffset) {

    }
}
