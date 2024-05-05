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
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.CommonTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public class SacrificeRoom implements Room {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShopRoom.class);
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final Map<RoomDirection, Room> neighbors;
    private final ItemService itemService;
    private final RoomObjectService roomObjectService;
    private final TileMap tileMap;
    private final StatisticsService statisticsService;
    public SacrificeRoom(final ItemService itemService,
                    final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors,
                    final StatisticsService statisticsService) {
        this.itemService = itemService;
        tileMap = new CommonTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());
        this.statisticsService = statisticsService;
    }

    @Override
    public void enterEvent(final Player p) {
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
        final PentagramCircle pentagramCircle = new PentagramCircle();
        pentagramCircle.setPosition((getMaxX() + getMinX()) / 2f -  pentagramCircle.getWidth() / 2f,
                (getMaxY() + getMinY()) / 2f -  pentagramCircle.getHeight() / 2f);
        roomObjectService.add(pentagramCircle);
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
        return RoomType.SACRIFICE;
    }

    @Override
    public List<RoomObject> getRoomObjects() {
        return roomObjectService.getRoomObjects();
    }
}
