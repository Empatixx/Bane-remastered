package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.factory.TileFactory;

import java.util.List;

public interface TileMap {
    TileFactory tileFactory = new TileFactory();
    List<Drawable> toRender();
    int getWidth();
    int getHeight();
    int getX();
    int getY();
    void cleanUp();
}
