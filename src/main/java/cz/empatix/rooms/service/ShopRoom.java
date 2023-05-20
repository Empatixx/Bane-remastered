package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.tiles.tilemaps.ShopTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class ShopRoom implements Room {
    private final List<RoomDirection> neighbors;
    private final TileMap tileMap;

    public ShopRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                    final List<RoomDirection> roomDirections) {
        tileMap = new ShopTileMap(rows, cols, xOffset, yOffset, roomDirections);
        neighbors = roomDirections;
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
}
