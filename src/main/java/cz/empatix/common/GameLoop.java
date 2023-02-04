package cz.empatix.common;

import cz.empatix.gamestate.GameStateService;
import cz.empatix.common.handlers.WindowHandler;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL11.*;

public class GameLoop extends Thread {
    private static final Logger LOGGER = LoggerFactory.getLogger(GameLoop.class);
    private final GameInitializator gameInitializator;
    private final GameStateService gsm;

    private final double FIXED_DELTA_TIME = 1.0 / 60.0;

    public GameLoop() {
        gameInitializator = new GameInitializator(this);
        gsm = new GameStateService();
    }

    @Override
    public void run() {
        try {
            int fps = 0;
            int ups = 0;
            long fpsTimer = System.currentTimeMillis();

            double accumulator = 0;
            long lastTime = System.nanoTime();
            while (!GLFW.glfwWindowShouldClose(WindowHandler.WINDOW)) {
                final long currentTime = System.nanoTime();
                final double dt = (currentTime - lastTime) / 1_000_000_000.0;

                accumulator += dt;
                lastTime = currentTime;

                while(accumulator >= FIXED_DELTA_TIME){
                    update(FIXED_DELTA_TIME);
                    accumulator -= FIXED_DELTA_TIME;
                    ups++;
                }
                render(dt);
                fps++;

                if (System.currentTimeMillis() - fpsTimer > 1000) {
                    LOGGER.info("FPS: " + fps + " UPS: " + ups);
                    fpsTimer += 1000;
                    ups = 0;
                    fps = 0;
                }
            }
        } catch (Exception e) {
            throw new RuntimeException(e);
        } finally {
            gameInitializator.cleanUp();
        }
    }

    public void update(final double dt) {
        glfwPollEvents();
        gsm.update(dt);
    }

    public void render(final double dt) {
        glClear(GL_COLOR_BUFFER_BIT);
        glClearColor(1.0f, 1.0f, 1.0f, 0.0f);
        gsm.render(dt);
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
