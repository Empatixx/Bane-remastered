package cz.empatix.gamestate.service;

import cz.empatix.audio.domain.Soundtrack;
import cz.empatix.audio.service.AudioService;
import cz.empatix.common.handlers.WindowHandler;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.gamestate.domain.GameState;
import cz.empatix.gamestate.domain.GameStateType;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.map.generators.StaticMapGenerator;
import cz.empatix.map.service.MapService;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.service.Camera;
import cz.empatix.roomobjects.domain.Scientist;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.upgrades.service.UpgradeTreeFactory;
import org.joml.Vector3i;
import org.lwjgl.glfw.GLFW;

public class LobbyState extends GameState {
    private MapService mapService;
    private Player player;
    private CoinService coinService;
    private Scientist scientist;

    protected LobbyState(final GameStateService gameStateService) {
        super(gameStateService, GameStateType.LOBBY);
    }

    @Override
    public void init() {
        CursorService.setCursor(CursorType.DEFAULT);

        final StaticMapGenerator  proceduralMapGenerator = new StaticMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate();

        mapService = new MapService(mapGenerationResult, room -> {
        });
        player = new Player();
        player.setPosition(mapGenerationResult.startRoom().getMaxX() / 2f, mapGenerationResult.startRoom().getMaxY() / 2f);

        coinService = new CoinService("ingame/green_banecoin.tga", new Vector3i(0,236,1));

        scientist = new Scientist();
        scientist.setPosition(Tile.TILE_SIZE * Tile.SCALE * 4, Tile.TILE_SIZE * Tile.SCALE * 2);

        AudioService.getInstance().play(Soundtrack.PROGRESS_ROOM);
        Camera.setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(double dt) {
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);

        player.update(dt);
        scientist.update(dt, mouseX, mouseY);
        mapService.update(dt, player);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
    }

    @Override
    public void toRender(double dt, RenderableState state) {
        state.addAll(mapService.toRender(dt))
             .add(player)
             .add(scientist)
             .addAll(coinService.toRender())
             .add(CursorService.getCursor());
    }

    @Override
    public void cleanUp() {
        player.cleanUp();
        PhysicsService.getInstance().clear();
        InteractionService.getInstance().clear();
        LightningService.getInstance().clear();
        scientist.cleanUp();
    }

    @Override
    public void keyPressed(int key) {
        if (key == GLFW.GLFW_KEY_ESCAPE){
            setCurrentState(GameStateType.MENU);
            GaussianBlur.setEnabled(false);
        }
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        player.keyReleased(key);
    }

    @Override
    public void mousePressed(int button) {
        scientist.mousePressed(button, mouseX, mouseY);
    }

    @Override
    public void mouseReleased(int button) {
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
