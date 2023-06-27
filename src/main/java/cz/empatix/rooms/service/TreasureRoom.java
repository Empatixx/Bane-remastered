package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.Chest;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.tiles.tilemaps.TileMap;
import cz.empatix.tiles.tilemaps.TreasureTileMap;

import java.util.*;

public class TreasureRoom implements Room {
    private final RoomObjectService roomObjectService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;

    public TreasureRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                        final Set<RoomDirection> neighbors) {
        tileMap = new TreasureTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());
    }

    @Override
    public void enterEvent(final Player p) {

    }

    @Override
    public void leaveEvent(final Player p) {

    }

    @Override
    public void init() {
        final Chest chest = new Chest();
        chest.setPosition((getMaxX() + getMinX()) / 2f - chest.getWidth() / 2f,
                (getMaxY() + getMinY()) / 2f - chest.getHeight() / 2f);
        roomObjectService.add(chest);
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
        neighbors.put(direction, room);
    }
    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        return neighbors;
    }
}
