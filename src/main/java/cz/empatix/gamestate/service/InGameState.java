package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.hud.service.StatusBar;
import cz.empatix.map.generators.ProceduralMapGenerator;
import cz.empatix.map.generators.StaticMapGenerator;
import cz.empatix.player.Player;
import cz.empatix.render.service.Camera;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.map.service.MapService;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.stream.Stream;

public class InGameState extends GameState {
    private final static Logger LOGGER = LoggerFactory.getLogger(InGameState.class);
    private MapService mapService;
    private Player player;
    private StatusBar statusBar;

    public InGameState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.INGAME);
    }

    @Override
    public void init() {
        final ProceduralMapGenerator proceduralMapGenerator = new ProceduralMapGenerator();

        mapService = proceduralMapGenerator.generate();
        player = new Player();

        AudioService.getInstance().play(Soundtrack.INGAME);
    }

    @Override
    public void update(final double dt) {
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH/2f + mouseDirectionX,player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT/2f + mouseDirectionY);

        player.update(dt);
        mapService.update(dt, player.getPosition());
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
    }

    @Override
    public List<Drawable> toRender(final double dt) {
        return Stream.concat(
                mapService.toRender(dt).stream(),
                Stream.of(player)
        ).toList();
    }

    @Override
    public void cleanUp() {

    }

    @Override
    public void keyPressed(int key) {
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        player.keyReleased(key);
    }

    @Override
    public void mousePressed(int button) {
        if(GLFW.GLFW_MOUSE_BUTTON_1 == button){
            player.setHealth(player.getHealth() - 3);
            player.setArmor(player.getArmor() - 1);
        }
        if(GLFW.GLFW_MOUSE_BUTTON_2 == button){
            player.setHealth(player.getHealth() + 1);
            player.setArmor(player.getArmor() + 1);
        }
    }

    @Override
    public void mouseReleased(int button) {

    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
