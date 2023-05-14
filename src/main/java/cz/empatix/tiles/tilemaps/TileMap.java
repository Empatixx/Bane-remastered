package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;

import java.util.List;

public interface TileMap {
    int TILE_SIZE = Tile.SCALE * Tile.TILE_SIZE;
    TileFactory tileFactory = new TileFactory();
    List<Drawable> toRender();
    void init();
    int getWidth();
    int getHeight();
    int getX();
    int getY();
}
