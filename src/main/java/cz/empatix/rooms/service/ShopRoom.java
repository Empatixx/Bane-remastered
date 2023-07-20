package cz.empatix.rooms.service;

import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.ShopTable;
import cz.empatix.roomobjects.domain.Shopkeeper;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;

import java.util.*;

public class ShopRoom implements Room {
    private final static int ITEM_TABLE_OFFSET = 15 ;
    private final static int TOTAL_TABLES = 3;
    private final Map<RoomDirection, Room> neighbors;
    private final RoomObjectService roomObjectService;
    private final TileMap tileMap;

    public ShopRoom(final ItemService itemService,
                    final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors) {
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());

        final ItemFactory itemFactory = itemService.getItemFactory();
        for (int i = 0; i < TOTAL_TABLES; i++){
            final ItemDrop shopItem = itemFactory.createShopDrop();
            shopItem.setPosition(getMinX() + tileMap.getWidth() / 4f * (i+1) - shopItem.getWidth()/2f,
                    getMinY()+ tileMap.getHeight() / 2f - shopItem.getHeight() / 2f - ITEM_TABLE_OFFSET);
            itemService.add(shopItem);
        }
    }

    @Override
    public void enterEvent(final Player p) {

    }

    @Override
    public void leaveEvent(final Player p) {

    }

    @Override
    public void init() {
        final Shopkeeper shopkeeper = new Shopkeeper();
        shopkeeper.setPosition(getMinX() + tileMap.getWidth()/2f - shopkeeper.getWidth() / 2f, getMinY() + tileMap.getHeight() / 4f - shopkeeper.getHeight() / 2f);
        roomObjectService.add(shopkeeper);
        for (int i = 0; i < TOTAL_TABLES; i++) {
            final ShopTable shopTable = new ShopTable();
            shopTable.setPosition(getMinX() + tileMap.getWidth() / 4f * (i+1) - shopTable.getWidth()/2f, getMinY()+ tileMap.getHeight() / 2f - shopTable.getHeight() / 2f);
            roomObjectService.add(shopTable);
        }

    }


    @Override
    public void update(double dt) {
        roomObjectService.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {
    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tileMapBatch = (DrawableBatch) tileMap::toRender;
        final Renderable roomObjectsBatch = (DrawableBatch) roomObjectService::toRender;
        return List.of(tileMapBatch, roomObjectsBatch);
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
    public int getX() {
        return tileMap.getX();
    }

    @Override
    public int getY() {
        return tileMap.getY();
    }
    @Override
    public void addNeighbour(RoomDirection direction, Room room) {
        neighbors.put(direction, room);
    }
    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        return neighbors;
    }
}
