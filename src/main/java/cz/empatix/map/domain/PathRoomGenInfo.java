package cz.empatix.map.domain;

import com.google.common.base.Preconditions;
import cz.empatix.rooms.service.PathRoom;
import cz.empatix.rooms.service.Room;

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
        Preconditions.checkArgument(connections.size() == 2, "Path room must have 2 connections");
        final RoomGenConnection parentConn = connections.get(0);
        final RoomGenConnection childConn = connections.get(1);

        final boolean isVertical = parentConn.direction().isVertical() && childConn.direction().isVertical();
        final boolean isHorizontal = parentConn.direction().isHorizontal() && childConn.direction().isHorizontal();

        Preconditions.checkArgument(isVertical || isHorizontal,
                "Path room must have connections in the same direction");

        final RoomGenInfo parent = parentConn.roomGenInfo();
        final RoomGenInfo child = childConn.roomGenInfo();

        final RoomDirection direction = parentConn.direction();
        final int newOffsetX, newOffsetY;

        final int centerX = (parent.getWidth() - this.getWidth()) / 2;


        switch (direction) {
            case TOP -> {
                newOffsetX = parent.getXOffset() + (parent.getWidth() - this.getWidth()) / 2;
                newOffsetY = parent.getYOffset() - this.getHeight();
            }
            case BOTTOM -> {
                newOffsetX = parent.getXOffset() + (parent.getWidth() - this.getWidth()) / 2;
                newOffsetY = child.getYOffset() - this.getHeight();
            }
            case LEFT -> {
                newOffsetX = parent.getXOffset() - this.getWidth();
                newOffsetY = parent.getYOffset() + (parent.getHeight() - this.getHeight()) / 2;
            }
            case RIGHT -> {
                newOffsetX = child.getXOffset() - this.getWidth();
                newOffsetY = parent.getYOffset() + (parent.getHeight() - this.getHeight()) / 2;
            }
            default -> throw new IllegalStateException("Unexpected value: " + direction);
        }
        setCoordinates(newOffsetX, newOffsetY);
        return new PathRoom(xOffset, yOffset, rows, cols, !isHorizontal);
    }

    @Override
    public boolean canCreateNeighbours() {
        return false;
    }
}
