package cz.empatix.pause.service;

import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.hud.service.Image;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.ui.Button;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.Collection;
import java.util.List;
import java.util.stream.Stream;

public class PauseService {
    private final Image background;
    private final List<Button> buttons;
    private boolean pause;

    public PauseService() {
        background = new Image("ingame/pausemenu.tga", new Vector2f(0, 0), 1f, 2);
        final Button resumeButton = new Button("Resume", new Vector2f(0, 225), 1.5f, 3);
        final Button settingsButton = new Button("Settings", new Vector2f(0, 325), 1.5f, 3);
        final Button exitButton = new Button("Exit", new Vector2f(0, 425), 1.5f, 3);

        buttons = List.of(resumeButton, settingsButton, exitButton);

        resumeButton.setOnClick(() -> pause = false);
        settingsButton.setOnClick(() -> System.out.println("Settings"));
        exitButton.setOnClick(() -> GLFW.glfwSetWindowShouldClose(WindowHandler.WINDOW, true));
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        buttons.forEach(button -> button.update(dt, mouseX, mouseY));
    }

    public List<Renderable> toRender(final double dt) {
        if(!pause){
            return List.of();
        }
        final List<? extends Drawable> drawables = Stream.of(buttons, List.of(background))
                .flatMap(Collection::stream)
                .toList();
        return List.copyOf(drawables);
    }

    public void keyPressed(final int key) {
        if (key == GLFW.GLFW_KEY_ESCAPE) {
            pause = !pause;
        }
    }
}
