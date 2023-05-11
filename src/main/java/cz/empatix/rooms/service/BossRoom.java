package cz.empatix.rooms.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class BossRoom implements Room {
    private final TileMap tileMap;
    private final int xOffset, yOffset;
    public BossRoom(final int rows,final int cols,final int xOffset,final int yOffset) {
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
        tileMap.init();

    }

    @Override
    public void update(double dt) {

    }

    @Override
    public List<Drawable> toRender() {
        return tileMap.toRender();
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
    public int getX(){
        return xOffset;
    }
    @Override
    public int getY(){
        return yOffset;
    }
}
