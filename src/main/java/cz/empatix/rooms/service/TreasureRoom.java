package cz.empatix.rooms.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class TreasureRoom implements Room {
    private final TileMap tileMap;
    private final int xOffset, yOffset;

    public TreasureRoom(final int rows, final int cols, final int xOffset, final int yOffset) {
        tileMap = new CommonTileMap(rows, cols);
        this.xOffset = xOffset;
        this.yOffset = yOffset;
    }

    @Override
    public void enterEvent() {

    }

    @Override
    public void leaveEvent() {

    }

    @Override
    public void init() {

    }

    @Override
    public void update() {

    }

    @Override
    public List<Drawable> toRender() {
        return tileMap.toRender();
    }

    @Override
    public int getWidth() {
        return 0;
    }

    @Override
    public int getHeight() {
        return 0;
    }
}
