package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;

import java.util.List;

public interface TileMap {
    TileFactory tileFactory = new TileFactory();
    List<Drawable> toRender();
    default int getTotalCollumns(){
        return getWidth() / (Tile.TILE_SIZE * Tile.SCALE);
    }
    default int getTotalRows(){
        return getHeight() / (Tile.TILE_SIZE * Tile.SCALE);
    }
    int getWidth();
    int getHeight();
    int getX();
    int getY();
    void cleanUp();
    boolean isWall(int xTile, int yTile);
}
