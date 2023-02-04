package cz.empatix.tiles.tilemaps;

import cz.empatix.render.attributes.Drawable;

import java.util.List;

public interface TileMap {
    List<Drawable> toRender();
    void init();
}
