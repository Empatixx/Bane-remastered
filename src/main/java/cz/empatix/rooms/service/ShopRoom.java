package cz.empatix.rooms.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.domain.ShopItem;
import cz.empatix.items.domain.ShopType;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.items.service.ShopItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.Pot;
import cz.empatix.roomobjects.domain.RoomObject;
import cz.empatix.roomobjects.domain.ShopTable;
import cz.empatix.roomobjects.domain.Shopkeeper;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public class ShopRoom implements Room {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShopRoom.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final static int TOTAL_TABLES = 3;
    private final Map<RoomDirection, Room> neighbors;
    private final ItemService itemService;
    private final RoomObjectService roomObjectService;
    private final TileMap tileMap;
    private Shopkeeper shopkeeper;
    private final StatisticsService statisticsService;
    private final ShopItemService shopItemService;

    public ShopRoom(final ItemService itemService,
                    final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors,
                    final StatisticsService statisticsService) {
        this.itemService = itemService;
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());
        this.statisticsService = statisticsService;
        shopItemService = new ShopItemService();
    }

    @Override
    public void enterEvent(final Player p) {
        shopkeeper.setInteractionText("Welcome to my shop!");
    }

    @Override
    public void leaveEvent(final Player p) {

    }
    @Override
    public void cleanUp() {
        roomObjectService.cleanUp();
        tileMap.cleanUp();
    }
    @Override
    public void init() {
        shopkeeper = new Shopkeeper();
        shopkeeper.setPosition(getMinX() + tileMap.getWidth()/2f - shopkeeper.getWidth() / 2f, getMinY() + tileMap.getHeight() / 4f - shopkeeper.getHeight() / 2f);
        roomObjectService.add(shopkeeper);

        final ItemFactory itemFactory = itemService.getItemFactory();
        final List<ShopType> types = new ArrayList<>(List.of(ShopType.OFFENSIVE, ShopType.AMMUNITION, ShopType.DEFENSIVE));
        Collections.reverse(types);
        for (int i = 0; i < TOTAL_TABLES; i++) {
            {
                final ShopTable shopTable = new ShopTable();
                shopTable.setPosition(getMinX() + tileMap.getWidth() / 4f * (i+1) - shopTable.getWidth()/2f, getMinY()+ tileMap.getHeight() / 2f - shopTable.getHeight() / 2f);
                roomObjectService.add(shopTable);

                final ShopItem shopItem = itemFactory.createShopDrop(types.get(i), statisticsService.getFloor());
                shopItem.setOnBuy(() -> {
                    shopkeeper.setInteractionText("Enjoy your purchase!");
                });
                shopItem.setPosition(getMinX() + tileMap.getWidth() / 4f * (i+1) - shopItem.getWidth()/2f, getMinY()+ tileMap.getHeight() / 2f - shopItem.getHeight() / 2f);
                shopItem.setHudPosition(shopTable.getX() + shopTable.getWidth()/2f, shopTable.getY());
                shopItem.setInteractionText(shopTable.getX(), shopTable.getX() + shopTable.getWidth(), shopTable.getY() + shopTable.getHeight());
                itemService.add(shopItem);
                shopItemService.add(shopItem);
            }
        }
        final Pot pot = new Pot(itemService);
        final Pot pot2 = new Pot(itemService);

        pot.setPosition(getMinX() + TileImpl.SCALE_TILE_SIZE * 2, getMinY() + tileMap.getHeight() * 4 / 5f - pot.getHeight() / 2f);
        pot2.setPosition(getMaxX() - TileImpl.SCALE_TILE_SIZE * 5, getMinY() + tileMap.getHeight() * 4 / 5f - pot.getHeight() / 2f);

        roomObjectService.add(pot);
        roomObjectService.add(pot2);

    }
    public void applyDiscounts(){
        final int chance = RANDOM.nextInt(0, 100);
        if (chance < 5){
            shopItemService.discount(0.75f, 1);
        } else if (chance < 10){
            shopItemService.discount(0.5f, 1);
        } else if (chance < 33){
            shopItemService.discount(0.25f, 1);
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
        final List<Renderable> renderables = new ArrayList<>(roomObjectService.toRender());
        final Renderable tileMapBatch = (DrawableBatch) tileMap::toRender;
        renderables.add(tileMapBatch);
        return renderables;
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

    @Override
    public RoomType getRoomType() {
        return RoomType.SHOP;
    }

    @Override
    public List<RoomObject> getRoomObjects() {
        return roomObjectService.getRoomObjects();
    }
}
