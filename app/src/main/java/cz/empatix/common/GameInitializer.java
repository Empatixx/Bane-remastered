package cz.empatix.common;
import cz.empatix.core.Initializer;
import cz.empatix.core.Viewport;

import cz.empatix.common.handlers.InputHandler;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.audio.init.ALInit;
import cz.empatix.common.initializators.GLFWInit;
import cz.empatix.render.init.GLInit;
import cz.empatix.settings.domain.GameSettings;
import cz.empatix.settings.service.SettingsService;

import java.util.List;

public class GameInitializer {
    /** Bane's internal render resolution (16:9). */
    private static final int TARGET_WIDTH = 960;
    private static final int TARGET_HEIGHT = 540;
    private final List<Initializer> initializers;
    public GameInitializer(final GameLoop gameLoop){
        Viewport.configure(TARGET_WIDTH, TARGET_HEIGHT);
        final GameSettings settings = SettingsService.getInstance().getSettings();
        final GLFWInit glfwInit = new GLFWInit();
        final GLInit glInit = new GLInit(settings.getResolutionWidth(), settings.getResolutionHeight());
        final ALInit alInit = new ALInit();

        initializers = List.of(glfwInit, glInit, alInit);
        initializers.forEach(Initializer::init);

        WindowHandler.registerCallbacks(gameLoop);
        InputHandler.registerCallbacks(gameLoop);
    }

    public void cleanUp(){
        //todo; fix errors
        //initializers.forEach(Initializer::destroy);
        //AssetsService.dispose();
    }
}
