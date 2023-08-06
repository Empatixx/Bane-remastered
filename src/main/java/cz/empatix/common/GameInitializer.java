package cz.empatix.common;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.common.handlers.InputHandler;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.common.initializators.*;

import java.util.List;

public class GameInitializer {
    private final List<Initializer> initializers;
    public GameInitializer(final GameLoop gameLoop){
        final GLFWInit glfwInit = new GLFWInit();
        final GLInit glInit = new GLInit();
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
