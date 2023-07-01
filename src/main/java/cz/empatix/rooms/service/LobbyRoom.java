package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.Portal;
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
