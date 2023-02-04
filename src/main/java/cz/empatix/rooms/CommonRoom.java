package cz.empatix.rooms;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.List;

public class CommonRoom implements Room{
    private final TileMap tileMap;

    public CommonRoom(){
        tileMap = new CommonTileMap();
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
    public void update() {

    }

    @Override
    public List<Drawable> toRender() {
        return tileMap.toRender();
    }
}
