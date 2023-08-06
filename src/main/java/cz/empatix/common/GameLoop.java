package cz.empatix.common;

import cz.empatix.gamestate.service.GameStateService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import static org.lwjgl.glfw.GLFW.*;
import static org.lwjgl.opengl.GL11.*;

public class GameLoop{
    private static final Logger LOGGER = LoggerFactory.getLogger(GameLoop.class);
    private final GameInitializer gameInitializer;
    private final GameStateService gss;

    public static final double FIXED_DELTA_TIME = 1.0 / 60.0;

    public GameLoop() {
        gameInitializer = new GameInitializer(this);
        gss = new GameStateService();
    }

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
                    fixedUpdate();
                    accumulator -= FIXED_DELTA_TIME;
                    ups++;
                }
                update(dt);
                render(dt);
                fps++;

                if (System.currentTimeMillis() - fpsTimer > 1000) {
                    LOGGER.info("FPS: {}, UPS: {}", fps, ups);
                    fpsTimer += 1000;
                    ups = 0;
                    fps = 0;
                }
            }
        } catch (Exception e) {
            LOGGER.error("Error in game loop", e);
            throw new RuntimeException(e);
        } finally {
            LOGGER.info("Cleaning up game loop");
            SettingsService.getInstance().saveSettings();
            gameInitializer.cleanUp();
        }
        System.exit(0);
    }

    private void fixedUpdate() {
        gss.fixedUpdate(FIXED_DELTA_TIME);
    }
    private void update(final double dt){
        glfwPollEvents();
        gss.update(dt);
    }

    public void render(final double dt) {
        glClear(GL_COLOR_BUFFER_BIT);
        glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
        gss.render(dt);
        glfwSwapBuffers(WindowHandler.WINDOW);
    }

    public void keyPressed(final int key) {
        gss.keyPressed(key);
    }

    public void keyReleased(final int key) {
        gss.keyReleased(key);
    }

    public void mousePressed(final int button) {
        gss.mousePressed(button);
    }

    public void mouseReleased(final int button) {
        gss.mouseReleased(button);
    }

    public void mouseScrolled(final double xoffset, final double yoffset) {
        gss.mouseScrolled(xoffset, yoffset);
    }
}
