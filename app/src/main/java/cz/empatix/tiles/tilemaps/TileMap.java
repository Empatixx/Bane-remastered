package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.TileImpl;

import java.util.List;

public interface TileMap {
    List<Drawable> toRender();
    default int getTotalCollumns(){
        return getWidth() / (TileImpl.TILE_SIZE * TileImpl.SCALE);
    }
    default int getTotalRows(){
        return getHeight() / (TileImpl.TILE_SIZE * TileImpl.SCALE);
    }
    int getWidth();
    int getHeight();
    int getX();
    int getY();
    void cleanUp();
}
