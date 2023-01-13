package cz.empatix.common;

import cz.empatix.gamestate.GameStateManager;
import cz.empatix.handlers.WindowHandler;
import org.lwjgl.glfw.GLFW;

import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL11.GL_COLOR_BUFFER_BIT;
import static org.lwjgl.opengl.GL11.glClear;

public class GameLoop extends Thread {
    private final GameInitializator gameInitializator;
    private final GameStateManager gsm;

    public GameLoop() {
        gameInitializator = new GameInitializator(this);
        gsm = new GameStateManager();
    }

    @Override
    public void run() {
        try {
            while (!GLFW.glfwWindowShouldClose(WindowHandler.WINDOW)) {
                update();
                render();
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        } finally {
            gameInitializator.cleanUp();
        }
    }

    private void update() {
        glfwPollEvents();
        gsm.update();
    }

    private void render() {
        glClear(GL_COLOR_BUFFER_BIT);
        gsm.render();
        glfwSwapBuffers(WindowHandler.WINDOW);
    }

    public void keyPressed(final int key) {
        gsm.keyPressed(key);
    }

    public void keyReleased(final int key) {
        gsm.keyReleased(key);
    }

    public void mousePressed(final int button) {
        gsm.mousePressed(button);
    }

    public void mouseReleased(final int button) {
        gsm.mouseReleased(button);
    }

    public void mouseScrolled(final double xoffset, final double yoffset) {
        gsm.mouseScrolled(xoffset, yoffset);
    }
}
