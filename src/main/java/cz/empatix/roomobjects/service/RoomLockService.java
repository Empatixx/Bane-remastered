package cz.empatix.roomobjects.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.roomobjects.domain.PathGate;
import cz.empatix.roomobjects.domain.PathGateDirection;
import cz.empatix.rooms.service.Room;
import cz.empatix.tiles.domain.TileImpl;

import java.util.List;
import java.util.Set;

public class RoomLockService {
    private final List<PathGate> pathGates;
    private boolean locked;

    public RoomLockService(final Room room, final Set<RoomDirection> directions) {
        pathGates = directions.stream()
                .map(d -> getPathGates(room, d))
                .flatMap(List::stream)
                .toList();
    }

    private List<PathGate> getPathGates(final Room room, final RoomDirection roomDirection) {
        final PathGate firstGate = new PathGate();
        final PathGate secondGate = new PathGate();

        final int centerX = room.getX() + room.getWidth() / 2;
        final int centerY = room.getY() + room.getHeight() / 2;

        final int tileSize = TileImpl.TILE_SIZE * TileImpl.SCALE;

        final int correctionX = Math.abs(centerX % (tileSize));
        final int correctionY = Math.abs(centerY % (tileSize));

        switch (roomDirection) {
            case BOTTOM -> {
                firstGate.setPosition(centerX - correctionX, room.getMaxY() - tileSize);
                secondGate.setPosition(centerX - correctionX - tileSize, room.getMaxY() - tileSize);
            }
            case TOP -> {
                firstGate.setPosition(centerX - correctionX, room.getY());
                secondGate.setPosition(centerX - correctionX - tileSize, room.getY());
            }
            case RIGHT -> {
                firstGate.setPosition(room.getMaxX() - tileSize, centerY - correctionY);
                secondGate.setPosition(room.getMaxX() - tileSize, centerY - correctionY - tileSize);
                firstGate.setDirection(PathGateDirection.RIGHT_SIDE);
                secondGate.setDirection(PathGateDirection.RIGHT_SIDE);
            }
            case LEFT -> {
                firstGate.setPosition(room.getX(), centerY - correctionY);
                secondGate.setPosition(room.getX(), centerY - correctionY - tileSize);
                firstGate.setDirection(PathGateDirection.LEFT_SIDE);
                secondGate.setDirection(PathGateDirection.LEFT_SIDE);
            }
        }
        return List.of(firstGate, secondGate);
    }

    public void setState(final boolean closed) {
        pathGates.forEach(pathGate -> pathGate.setState(closed));
        locked = closed;
    }


    public boolean isLocked() {
        return locked;
    }

    public void update(final double dt) {
        pathGates.forEach(pathGate -> pathGate.update(dt));
    }

    public List<Drawable> toRender() {
        return List.copyOf(pathGates);
    }

    public void cleanUp() {
        pathGates.forEach(PathGate::destroy);
    }
}
