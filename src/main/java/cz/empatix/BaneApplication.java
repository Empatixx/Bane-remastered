package cz.empatix;

import cz.empatix.common.GameLoop;

public class BaneApplication {
    public static void main(String[] args) {
        final GameLoop gameLoop = new GameLoop();
        gameLoop.run();
    }
}