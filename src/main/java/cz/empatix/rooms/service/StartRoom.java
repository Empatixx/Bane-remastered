package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.*;

public class StartRoom implements Room {
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;

    public StartRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                     final Set<RoomDirection> neighbors) {
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
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
    }

    @Override
    public void update(double dt) {

    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> toRender() {
        return List.of((DrawableBatch) tileMap::toRender);
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

    @Override
    public RoomType getRoomType() {
        return RoomType.START;
    }
}
