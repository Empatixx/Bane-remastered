package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.Tile;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public abstract class RoomGenInfo {
    private final static Logger LOGGER = LoggerFactory.getLogger(RoomGenInfo.class);
    protected final static int MAX_ROOM_NEIGHBOURS = 4;
    protected final static int ROOM_ORIENTATION_TOP = 0;
    protected final static int ROOM_ORIENTATION_BOTTOM = 1;
    protected final static int ROOM_ORIENTATION_LEFT = 2;
    protected final static int ROOM_ORIENTATION_RIGHT = 3;

    protected int xOffset, yOffset;
    protected int cols, rows;
    protected List<RoomGenInfo> neighbours = new ArrayList<>(MAX_ROOM_NEIGHBOURS);

    public abstract void initDimensions();

    public abstract Room constructRoom();

    public abstract boolean canCreateNeighbours();

    public void setCoordinates(int xOffset, int yOffset) {
        this.xOffset = xOffset;
        this.yOffset = yOffset;
    }

    public void addNeighbour(final RoomGenInfo neighbour) {
        neighbours.add(neighbour);
    }

    public List<RoomGenInfo> getNeighbours() {
        return neighbours;
    }

    public int getWidthCenter() {
        return xOffset + getWidth() / 2;
    }

    public int getHeightCenter() {
        return yOffset + getHeight() / 2;
    }

    public int getWidth() {
        return cols * Tile.TILE_SIZE;
    }

    public int getHeight() {
        return rows * Tile.TILE_SIZE;
    }

    public int getXOffset() {
        return xOffset;
    }

    public int getYOffset() {
        return yOffset;
    }

    public void setOrientationAround(final RoomGenInfo parent) {
        final int orientation = ThreadLocalRandom.current().nextInt(MAX_ROOM_NEIGHBOURS);

        final int parentWidthCenter = parent.getWidthCenter();
        final int parentHeightCenter = parent.getHeightCenter();

        final int selfWidthCenter = getWidthCenter();
        final int selfHeightCenter = getHeightCenter();

        final int newOffsetX, newOffsetY;

        switch (orientation) {
            case ROOM_ORIENTATION_TOP -> {
                newOffsetX = parentWidthCenter - selfWidthCenter + parent.getXOffset();
                newOffsetY = parent.getYOffset() - parent.getHeight();
            }
            case ROOM_ORIENTATION_BOTTOM -> {
                newOffsetX = parentWidthCenter - selfWidthCenter + parent.getXOffset();
                newOffsetY = parent.getYOffset() + parent.getHeight();
            }
            case ROOM_ORIENTATION_LEFT -> {
                newOffsetX = parent.getXOffset() - parent.getWidth();
                newOffsetY = parentHeightCenter - selfHeightCenter + parent.getYOffset();
            }
            case ROOM_ORIENTATION_RIGHT -> {
                newOffsetX = parent.getXOffset() + parent.getWidth();
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
        return this.getXOffset()< other.getXOffset() + other.getWidth() &&
                this.getXOffset() + this.getWidth() > other.getXOffset() &&
                this.getYOffset() < other.getYOffset() + other.getHeight() &&
                this.getYOffset() + this.getHeight() > other.getYOffset();
    }
}
