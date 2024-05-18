package cz.empatix.rooms.service;

import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.roomobjects.suppliers.BonesSupplier;
import cz.empatix.roomobjects.suppliers.CobwebSupplier;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.TileImpl;
import cz.empatix.tiles.tilemaps.TileMap;
import cz.empatix.tiles.tilemaps.ProceduralCarpetTileMap;
import cz.empatix.tiles.tilemaps.TreasureCarpetTileMap;
import cz.empatix.tiles.tilemaps.TreasureTileMap;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.stream.Stream;

public class TreasureRoom implements Room {
    private final RoomObjectService roomObjectService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;
    private final TileMap carpetTileMap;
    private final ItemService itemService;

    public TreasureRoom(final int rows, final int cols, final int xOffset, final int yOffset,
                        final Set<RoomDirection> neighbors, final ItemService itemService) {
        tileMap = new TreasureTileMap(rows, cols, xOffset, yOffset, neighbors);
        carpetTileMap = new TreasureCarpetTileMap(rows, cols, xOffset, yOffset, neighbors);
        roomObjectService = new RoomObjectService();
        this.neighbors = new HashMap<>(neighbors.size());
        this.itemService = itemService;
    }

    @Override
    public void enterEvent(final Player p) {

    }

    @Override
    public void leaveEvent(final Player p) {

    }

    @Override
    public void init() {
        {
            final Torch topLeftTorch = new Torch();
            final Torch topRightTorch = new Torch();
            final Torch bottomLeftTorch = new Torch();
            final Torch bottomRightTorch = new Torch();

            final Bricks topLeftBricks = new Bricks();
            final Bricks bottomLeftBricks = new Bricks();

            topLeftBricks.setPosition(getMinX() + TileImpl.SCALE_TILE_SIZE * 4, getMinY() + TileImpl.SCALE_TILE_SIZE * 4.5f);
            bottomLeftBricks.setPosition(getMinX() + TileImpl.SCALE_TILE_SIZE * 3, getMaxY() - TileImpl.SCALE_TILE_SIZE * 3.5f);

            topLeftTorch.setPosition(getMinX() + TileImpl.SCALE_TILE_SIZE  * 5, getMinY() + TileImpl.SCALE_TILE_SIZE * 3);
            topRightTorch.setPosition(getMaxX() - TileImpl.SCALE_TILE_SIZE * 6, getMinY() + TileImpl.SCALE_TILE_SIZE * 3);
            bottomLeftTorch.setPosition(getMinX() + TileImpl.SCALE_TILE_SIZE * 5, getMaxY() - TileImpl.SCALE_TILE_SIZE * 5);
            bottomRightTorch.setPosition(getMaxX() - TileImpl.SCALE_TILE_SIZE * 6, getMaxY() - TileImpl.SCALE_TILE_SIZE * 5);

            topLeftTorch.setState(AnimationState.LEFT_SIDE);
            topRightTorch.setState(AnimationState.RIGHT_SIDE);
            bottomLeftTorch.setState(AnimationState.LEFT_SIDE);
            bottomRightTorch.setState(AnimationState.RIGHT_SIDE);

            roomObjectService.addAll(List.of(topLeftTorch, topRightTorch, bottomLeftTorch, bottomRightTorch));
            roomObjectService.addAll(List.of(topLeftBricks, bottomLeftBricks));

            final CobwebSupplier cobwebSupplier = new CobwebSupplier(this, tileMap);
            final BonesSupplier bonesSupplier = new BonesSupplier(this, tileMap);
            final List<Cobweb> cobwebs = Stream.generate(cobwebSupplier::supply)
                    .limit(3)
                    .toList();
            final List<Bones> bones = Stream.generate(bonesSupplier::supply)
                    .limit(3)
                    .toList();
            roomObjectService.addAll(cobwebs);
            roomObjectService.addAll(bones);

        }

        final Chest chest = new Chest(itemService, ChestType.WEAPON);
        chest.setPosition((getMaxX() + getMinX()) / 2f - chest.getWidth() / 2f,
                (getMaxY() + getMinY()) / 2f - chest.getHeight() / 2f);
        roomObjectService.add(chest);
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
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable carpetTiles = (DrawableBatch) carpetTileMap::toRender;
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        return List.of(tiles, roomObjects, carpetTiles);
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
        return RoomType.TREASURE;
    }

    @Override
    public List<RoomObject> getRoomObjects() {
        return roomObjectService.getRoomObjects();
    }

    @Override
    public void cleanUp() {
        tileMap.cleanUp();
        roomObjectService.cleanUp();
    }
}
