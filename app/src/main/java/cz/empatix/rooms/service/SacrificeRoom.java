package cz.empatix.rooms.service;

import cz.empatix.death.service.StatisticsService;
import cz.empatix.items.domain.SacrificialItem;
import cz.empatix.items.domain.ShopType;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.tilemaps.SacrificialTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import cz.empatix.common.RngService;

public class SacrificeRoom implements Room {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShopRoom.class);
    private final RngService rngService;
    private final Map<RoomDirection, Room> neighbors;
    private final ItemService itemService;
    private final RoomObjectService roomObjectService;
    private final TileMap tileMap;
    private final StatisticsService statisticsService;
    public SacrificeRoom(final ItemService itemService,
                    final int rows, final int cols, final int xOffset, final int yOffset,
                    final Set<RoomDirection> neighbors,
                    final StatisticsService statisticsService,
                    final RngService rngService) {
        this.itemService = itemService;
        tileMap = new SacrificialTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());
        this.statisticsService = statisticsService;
        this.rngService = rngService;
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
        final AngelStatue angelStatue = new AngelStatue();
        angelStatue.setPosition((getMaxX() + getMinX()) / 2f - angelStatue.getWidth() / 2f,
                (getMaxY() + getMinY()) / 2f - angelStatue.getHeight() / 2f - angelStatue.getHeight()/2f - 27);
        roomObjectService.add(angelStatue);

        final Candlestick leftCandlestick = new Candlestick();
        final Candlestick leftCandlestick2 = new Candlestick();
        final Candlestick rightCandlestick = new Candlestick();
        final Candlestick rightCandlestick2 = new Candlestick();
        final Candlestick bottomCandlestick = new Candlestick();

        leftCandlestick.setPosition(getMinX() + 8 * TileImpl.SCALE_TILE_SIZE, getMinY() + 6 * TileImpl.SCALE_TILE_SIZE);
        leftCandlestick2.setPosition(getMinX() + 7.5f * TileImpl.SCALE_TILE_SIZE, getMinY() + 8 * TileImpl.SCALE_TILE_SIZE);

        rightCandlestick.setPosition(getMinX() + 11 * TileImpl.SCALE_TILE_SIZE, getMinY() + 6 * TileImpl.SCALE_TILE_SIZE);
        rightCandlestick2.setPosition(getMinX() + 11.5f * TileImpl.SCALE_TILE_SIZE, getMinY() + 8 * TileImpl.SCALE_TILE_SIZE);

        bottomCandlestick.setPosition(getMinX() + 9.5f * TileImpl.SCALE_TILE_SIZE, getMinY() + 9f * TileImpl.SCALE_TILE_SIZE);

        roomObjectService.addAll(List.of(leftCandlestick, leftCandlestick2, rightCandlestick, rightCandlestick2, bottomCandlestick));

        final PentagramCircle pentagramCircle = new PentagramCircle();
        pentagramCircle.setPosition((getMaxX() + getMinX()) / 2f -  pentagramCircle.getWidth() / 2f,
                (getMaxY() + getMinY()) / 2f -  pentagramCircle.getHeight() / 2f);
        roomObjectService.add(pentagramCircle);

        final ItemFactory factory = itemService.getItemFactory();
        final SacrificialItem item = factory.createSacrificialItem(ShopType.values()[rngService.nextInt(ShopType.values().length)],
                statisticsService.getFloor());
        item.setPosition(pentagramCircle.getX() + pentagramCircle.getWidth() / 2f - item.getWidth() / 2f,
                pentagramCircle.getY() + pentagramCircle.getHeight() / 2f - item.getHeight() / 2f);
        item.setHudPosition(pentagramCircle.getX() + pentagramCircle.getWidth() / 2f, pentagramCircle.getY());
        item.setInteractionText(pentagramCircle.getX(), pentagramCircle.getX() + pentagramCircle.getWidth(), pentagramCircle.getY() + pentagramCircle.getHeight());
        item.onBuy(() -> {
            leftCandlestick.disableLight();
            leftCandlestick2.disableLight();
            rightCandlestick.disableLight();
            rightCandlestick2.disableLight();
            bottomCandlestick.disableLight();
        });
        itemService.add(item);

        final LayingBook layingBook = new LayingBook();
        final LayingBook layingBook2 = new LayingBook();
        final LayingBook layingBook3 = new LayingBook();

        final Chains chains = new Chains();
        final Chains chains2 = new Chains();

        layingBook.setPosition(getMinX() + 6 * TileImpl.SCALE_TILE_SIZE, getMinY() + 6 * TileImpl.SCALE_TILE_SIZE);
        layingBook2.setPosition(getMinX() + 13 * TileImpl.SCALE_TILE_SIZE, getMinY() + 3 * TileImpl.SCALE_TILE_SIZE);
        layingBook3.setPosition(getMinX() + 11 * TileImpl.SCALE_TILE_SIZE, getMinY() + 11 * TileImpl.SCALE_TILE_SIZE);

        roomObjectService.add(layingBook);
        roomObjectService.add(layingBook2);
        roomObjectService.add(layingBook3);

        chains.setPosition(getMinX() + 3 * TileImpl.SCALE_TILE_SIZE, getMinY() + 4 * TileImpl.SCALE_TILE_SIZE);
        chains2.setPosition(getMinX() + 15 * TileImpl.SCALE_TILE_SIZE, getMinY() + 11 * TileImpl.SCALE_TILE_SIZE);

        roomObjectService.add(chains);
        roomObjectService.add(chains2);
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
