package cz.empatix.tiles.tilemaps;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.factory.TileFactory;

import java.util.List;

public interface TileMap {
    TileFactory tileFactory = new TileFactory();
    List<Drawable> toRender();
    int getWidth();
    int getHeight();
    int getX();
    int getY();
}
