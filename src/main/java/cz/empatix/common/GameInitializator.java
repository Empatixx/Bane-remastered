package cz.empatix.common;

import cz.empatix.handlers.WindowHandler;
import cz.empatix.initializators.ALInit;
import cz.empatix.initializators.GLFWInit;
import cz.empatix.initializators.GLInit;
import cz.empatix.initializators.Initializator;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.handlers.InputHandler;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class GameInitializator {
    private final List<Initializator> initializators = new ArrayList<>();
    private final InputHandler inputHandler;
    private final WindowHandler windowHandler;
    public GameInitializator(final GameLoop gameLoop){
        final List<Initializator> initializators = new ArrayList<>();
        initializators.add(new GLFWInit());
        initializators.add(new GLInit());
        initializators.add(new ALInit());

        initializators.forEach(Initializator::init);

        inputHandler = new InputHandler(gameLoop);
        windowHandler = new WindowHandler(gameLoop);
    }

    public void cleanUp(){
        Collections.reverse(initializators);
        initializators.forEach(Initializator::destroy);
        AssetsService.dispose();
    }
}
