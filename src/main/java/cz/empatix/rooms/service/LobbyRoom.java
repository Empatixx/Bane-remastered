package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.Portal;
import cz.empatix.roomobjects.domain.Torch;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.LobbyTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;

import java.util.List;
import java.util.Map;

public class LobbyRoom implements Room {
    private final TileMap tileMap;
    private final RoomObjectService roomObjectService;

    public LobbyRoom() {
        tileMap = new LobbyTileMap();
        roomObjectService = new RoomObjectService();
    }

    @Override
    public void enterEvent(Player p) {

    }

    @Override
    public void leaveEvent(Player p) {

    }

    @Override
    public void init() {
        final Portal portal = new Portal(new Vector2f((getMinX() + getMaxX()) / 2f - 96, Tile.SCALE * Tile.TILE_SIZE/2f));
        roomObjectService.add(portal);

        final Torch torch1 = new Torch();
        final Torch torch2 = new Torch();
        final Torch torch3 = new Torch();
        final Torch torch4 = new Torch();

        torch1.setPosition(Tile.TILE_SIZE * Tile.SCALE, Tile.TILE_SIZE * Tile.SCALE * 2);
        torch2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 10, Tile.TILE_SIZE * Tile.SCALE * 2);
        torch3.setPosition(Tile.TILE_SIZE * Tile.SCALE * 18, Tile.TILE_SIZE * Tile.SCALE * 2);
        torch4.setPosition(Tile.TILE_SIZE * Tile.SCALE * 27, Tile.TILE_SIZE * Tile.SCALE * 2);

        torch2.setState(AnimationState.RIGHT_SIDE);
        torch4.setState(AnimationState.RIGHT_SIDE);

        roomObjectService.add(torch1);
        roomObjectService.add(torch2);
        roomObjectService.add(torch3);
        roomObjectService.add(torch4);
    }

    @Override
    public void update(double dt) {
        roomObjectService.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        return List.of(tiles, roomObjects);
    }

    @Override
    public int getWidth() {
        return tileMap.getWidth();
    }

    @Override
    public int getHeight() {
        return tileMap.getHeight();
    }

    @Override
    public int getX() {
        return tileMap.getX();
    }

    @Override
    public int getY() {
        return tileMap.getY();
    }

    @Override
    public void addNeighbour(RoomDirection direction, Room room) {
        throw new UnsupportedOperationException("Lobby room cannot have neighbours");
    }

    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        throw new UnsupportedOperationException("Lobby room cannot have neighbours");
    }
}
