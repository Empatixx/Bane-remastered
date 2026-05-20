package cz.empatix.map.domain;

import com.google.common.base.Preconditions;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.PathRoom;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.TileImpl;

import java.util.List;
import java.util.Map;

public class PathRoomGenInfo extends RoomGenInfo {
    public final static int MIN_ROWS = 4;
    public final static int MIN_COLS = 4;

    @Override
    public void initDimensions() {
        rows = MIN_ROWS;
        cols = MIN_COLS;
    }

    @Override
    public Room constructRoom() {
        Preconditions.checkArgument(neighbours.size() == 2, "Path room must have 2 connections");
        final List<Map.Entry<RoomDirection, RoomGenInfo>> entries = List.copyOf(neighbours.entrySet());

        final Map.Entry<RoomDirection, RoomGenInfo> parentNeighbour = entries.get(0);
        final Map.Entry<RoomDirection, RoomGenInfo> childNeighbour = entries.get(1);

        final RoomDirection parentDirection = parentNeighbour.getKey();
        final RoomGenInfo parent = parentNeighbour.getValue();

        final RoomDirection childDirection = childNeighbour.getKey();
        final RoomGenInfo child = childNeighbour.getValue();

        final boolean isVertical = parentDirection.isVertical() && childDirection.isVertical();
        final boolean isHorizontal = parentDirection.isHorizontal() && childDirection.isHorizontal();

        Preconditions.checkArgument(isVertical || isHorizontal,
                "Path room must have connections in the same direction");


        final int newOffsetX, newOffsetY;

        final int centerX = (parent.getWidth() - this.getWidth()) / 2;
        final int centerY = (parent.getHeight() - this.getHeight()) / 2;

        final int tileSize = TileImpl.TILE_SIZE * TileImpl.SCALE;

        // to tile based sizes
        final int shiftCenterX = centerX % tileSize;
        final int shiftCenterY = centerY % tileSize;

        switch (parentDirection) {
            case TOP -> {
                newOffsetX = parent.getXOffset() - shiftCenterX + centerX;
                newOffsetY = parent.getYOffset() - this.getHeight();
            }
            case BOTTOM -> {
                newOffsetX = parent.getXOffset() - shiftCenterX + centerX;
                newOffsetY = child.getYOffset() - this.getHeight();
            }
            case LEFT -> {
                newOffsetX = parent.getXOffset() - this.getWidth();
                newOffsetY = parent.getYOffset() - shiftCenterY + centerY;
            }
            case RIGHT -> {
                newOffsetX = child.getXOffset() - this.getWidth();
                newOffsetY = parent.getYOffset() - shiftCenterY + centerY;
            }
            default -> throw new IllegalStateException("Unexpected value: " + parentDirection);
        }

        // room dimensions for Camera
        final int minX, minY, maxX, maxY;
        switch (parentDirection){
            case TOP, BOTTOM -> {
                minX = parent.getXOffset();
                maxX = parent.getXOffset() + parent.getWidth();

                minY = Math.min(parent.getYOffset(), child.getYOffset());
                maxY = Math.max(parent.getYOffset() + parent.getHeight(), child.getYOffset() + child.getHeight());
            }
            case LEFT, RIGHT -> {
                minX = Math.min(parent.getXOffset(), child.getXOffset());
                maxX = Math.max(parent.getXOffset() + parent.getWidth(), child.getXOffset() + child.getWidth());

                minY = parent.getYOffset();
                maxY = parent.getYOffset() + parent.getHeight();
            }
            default -> throw new IllegalStateException("Unexpected value: " + parentDirection);
        }
        setCoordinates(newOffsetX, newOffsetY);
        return new PathRoom(xOffset, yOffset, rows, cols, minX, minY, maxX, maxY, isHorizontal, parentDirection);
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.PATH;
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
