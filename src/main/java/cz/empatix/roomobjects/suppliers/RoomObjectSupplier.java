package cz.empatix.roomobjects.suppliers;

import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.tilemaps.TileMap;

public interface RoomObjectSupplier<E>{
    E supply();
}
