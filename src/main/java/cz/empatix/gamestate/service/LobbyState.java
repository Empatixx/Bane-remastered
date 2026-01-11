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
import cz.empatix.pause.service.PauseService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.postprocessing.domain.GaussianBlur;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderableState;
import cz.empatix.render.service.Camera;
import cz.empatix.roomobjects.domain.Scientist;
import cz.empatix.tiles.domain.TileImpl;
import org.joml.Vector3i;
import org.lwjgl.glfw.GLFW;

import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class LobbyState extends GameState {
    private PauseService pauseService;
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

        final StaticMapGenerator proceduralMapGenerator = new StaticMapGenerator();
        final MapGenerationResult mapGenerationResult = proceduralMapGenerator.generate();

        pauseService = new PauseService();
        mapService = new MapService();
        mapService.apply(mapGenerationResult);

        player = new Player();
        player.setPosition(mapGenerationResult.startRoom().getMaxX() / 2f, mapGenerationResult.startRoom().getMaxY() / 2f);

        coinService = new CoinService("ingame/green_banecoin.tga", new Vector3i(236, 0, 0));

        scientist = new Scientist();
        scientist.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 25, TileImpl.TILE_SIZE * TileImpl.SCALE * 6.5f);

        AudioService.getInstance().play(Soundtrack.PROGRESS_ROOM);
        Camera.getInstance().setPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f);
    }

    @Override
    public void update(double dt) {
        final float mouseDirectionX = (mouseX - WindowHandler.TARGET_WIDTH / 2f) / 10f;
        final float mouseDirectionY = (mouseY - WindowHandler.TARGET_HEIGHT / 2f) / 10f;
        Camera.getInstance().setSmoothlyPosition(player.getX() + player.getWidth() - WindowHandler.TARGET_WIDTH / 2f + mouseDirectionX, player.getY() + player.getHeight() - WindowHandler.TARGET_HEIGHT / 2f + mouseDirectionY, (float) dt);

        player.update(dt);
        scientist.update(dt, mouseX, mouseY);
        mapService.update(dt, player);
        pauseService.update(dt, mouseX, mouseY);
    }

    @Override
    public void fixedUpdate(double dt) {
        player.fixedUpdate(dt);
        mapService.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> toRender(double dt) {
        final RenderableState coinState = new RenderableState(8);
        coinService.collectSubRenderables(coinState);
        final List<Renderable> renderables = Stream.of(mapService.toRender(dt),
                        pauseService.toRender(dt),
                        List.of(player, scientist),
                        coinState.getList())
                .flatMap(List::stream)
                .collect(Collectors.toList());
        renderables.add(CursorService.getCursor());
        return List.copyOf(renderables);
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
        pauseService.keyPressed(key);
        if (pauseService.isPause()) {
            player.stopMoving();
            return;
        }
        player.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        player.keyReleased(key);
    }

    @Override
    public void mousePressed(int button) {
        pauseService.mousePressed(button, mouseX, mouseY);
        scientist.mousePressed(button, mouseX, mouseY);
    }

    @Override
    public void mouseReleased(int button) {
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {

    }
}
