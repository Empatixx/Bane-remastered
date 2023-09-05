package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.tilemaps.PathTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class PathRoom implements Room {
    private final int xMin;
    private final int yMin;
    private final int xMax;
    private final int yMax;
    private final TileMap tileMap;

    public PathRoom(final int xOffset,
                    final int yOffset,
                    final int rows,
                    final int cols,
                    final int xMin,
                    final int yMin,
                    final int xMax,
                    final int yMax,
                    final boolean isHorizontal) {
        this.xMin = xMin;
        this.yMin = yMin;
        this.xMax = xMax;
        this.yMax = yMax;
        tileMap = new PathTileMap(xOffset, yOffset, rows, cols, isHorizontal);
    }
    @Override
    public void cleanUp() {
        tileMap.cleanUp();
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
    public int getMinX() {
        return xMin;
    }
    @Override
    public int getMinY() {
        return yMin;
    }
    @Override
    public int getMaxX() {
        return xMax;
    }
    @Override
    public int getMaxY() {
        return yMax;
    }
    @Override
    public void addNeighbour(RoomDirection direction, Room room) {
    }
    @Override
    public java.util.Map<RoomDirection, Room> getNeighboursMap() {
        throw new IllegalStateException("PathRoom does not have neighbors");
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.PATH;
    }
}
