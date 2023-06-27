package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;

public abstract class RoomGenInfo {
    private final static Logger LOGGER = LoggerFactory.getLogger(RoomGenInfo.class);
    protected final static int MAX_ROOM_NEIGHBOURS = 4;

    protected int xOffset, yOffset;
    protected int cols, rows;
    protected Map<RoomDirection, RoomGenInfo> neighbours = new EnumMap<>(RoomDirection.class);

    public abstract void initDimensions();

    public abstract Room constructRoom();

    public abstract boolean canCreateNeighbours();

    public void setCoordinates(int xOffset, int yOffset) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
    }

    public void addNeighbour(final RoomGenInfo neighbour, final RoomDirection dir) {
        neighbours.put(dir, neighbour);
    }
    public List<RoomDirection> getAvailableDirections() {
        final List<RoomDirection> availableDirections = new ArrayList<>(List.of(RoomDirection.values()));
        availableDirections.removeIf(direction -> neighbours.containsKey(direction));
        return availableDirections;
    }

    public int getWidthCenter() {
        return xOffset + getWidth() / 2;
    }

    public int getHeightCenter() {
        return yOffset + getHeight() / 2;
    }

    public int getWidth() {
        return cols * Tile.TILE_SIZE * Tile.SCALE;
    }

    public int getHeight() {
        return rows * Tile.TILE_SIZE * Tile.SCALE;
    }

    public int getXOffset() {
        return xOffset;
    }

    public int getYOffset() {
        return yOffset;
    }

    public void setOrientationAround(final RoomGenInfo parent) {
        setCoordinates(0,0);
        final int orientation = ThreadLocalRandom.current().nextInt(MAX_ROOM_NEIGHBOURS);

        final int parentWidthCenter = parent.getWidthCenter();
        final int parentHeightCenter = parent.getHeightCenter();

        final int selfWidthCenter = getWidthCenter();
        final int selfHeightCenter = getHeightCenter();

        final int newOffsetX, newOffsetY;

        final int pathWidth = PathRoomGenInfo.MIN_COLS * Tile.TILE_SIZE * Tile.SCALE;
        final int pathHeight = PathRoomGenInfo.MIN_ROWS * Tile.TILE_SIZE * Tile.SCALE;

        switch (RoomDirection.getRandom()) {
            case TOP -> {
                newOffsetX = parentWidthCenter - selfWidthCenter + parent.getXOffset();
                newOffsetY = parent.getYOffset() - parent.getHeight() - pathHeight;
            }
            case BOTTOM -> {
                newOffsetX = parentWidthCenter - selfWidthCenter + parent.getXOffset();
                newOffsetY = parent.getYOffset() + parent.getHeight() + pathHeight;
            }
            case LEFT -> {
                newOffsetX = parent.getXOffset() - parent.getWidth() - pathWidth;
                newOffsetY = parentHeightCenter - selfHeightCenter + parent.getYOffset();
            }
            case RIGHT -> {
                newOffsetX = parent.getXOffset() + parent.getWidth() - pathWidth;
                newOffsetY = parentHeightCenter - selfHeightCenter + parent.getYOffset();
            }
            default -> throw new IllegalStateException("Unexpected value: " + orientation);
        }
        setCoordinates(newOffsetX, newOffsetY);
    }
    public boolean intersects(final RoomGenInfo other){
        if(other == null){
            LOGGER.warn("Cannot check intersection with null room");
            return false;
        }
        return this.getXOffset() < other.getXOffset() + other.getWidth() &&
                this.getXOffset() + this.getWidth() > other.getXOffset() &&
                this.getYOffset() < other.getYOffset() + other.getHeight() &&
                this.getYOffset() + this.getHeight() > other.getYOffset();
    }

    public Map<RoomDirection, RoomGenInfo> getNeighbours() {
        return neighbours;
    }
}
