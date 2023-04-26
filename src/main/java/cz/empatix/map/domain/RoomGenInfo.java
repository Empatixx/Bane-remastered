package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;

import java.util.ArrayList;
import java.util.List;

public abstract class RoomGenInfo {
    protected final static int MAX_ROOM_NEIGHBOURS = 4;
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
        return xOffset + cols / 2;
    }
    public int getHeightCenter() {
        return yOffset + rows / 2;
    }
    public int getXOffset() {
        return xOffset;
    }
    public int getYOffset() {
        return yOffset;
    }
}
