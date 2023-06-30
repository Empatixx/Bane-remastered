package cz.empatix.map.generators;

import com.google.common.base.Preconditions;
import cz.empatix.map.domain.*;
import cz.empatix.rooms.service.Room;
import cz.empatix.rooms.service.StartRoom;
import cz.empatix.tiles.domain.Tile;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class ProceduralMapGenerator implements MapGenerator {
    private final static float ROOMS_DENSITY = 0.15f;
    private final static Logger LOGGER = LoggerFactory.getLogger(ProceduralMapGenerator.class);
    private final static int TOTAL_NORMAL_ROOMS = 5;
    private final static int TOTAL_BOSS_ROOMS = 1;
    private final static int TOTAL_SHOP_ROOMS = 1;
    private final static int TOTAL_LOOT_ROOMS = 1;
    private final static int TOTAL_START_ROOMS = 1;
    private final static int TOTAL_ROOMS = TOTAL_NORMAL_ROOMS + TOTAL_BOSS_ROOMS + TOTAL_SHOP_ROOMS + TOTAL_START_ROOMS + TOTAL_LOOT_ROOMS;

    static {
        Preconditions.checkArgument(TOTAL_ROOMS > 0, "Total rooms must be greater than 0");
        Preconditions.checkArgument(TOTAL_START_ROOMS == 1, "Total start rooms must be 1");
        Preconditions.checkArgument(TOTAL_BOSS_ROOMS == 1, "Total boss rooms must be 1");
    }

    @Override
    public MapGenerationResult generate() {
        final StartRoomGenInfo startRoomGenInfo = new StartRoomGenInfo();
        final List<CommonRoomGenInfo> normalRooms = Stream.generate(CommonRoomGenInfo::new)
                .limit(TOTAL_NORMAL_ROOMS)
                .toList();
        final List<ShopRoomGenInfo> shopRooms = Stream.generate(ShopRoomGenInfo::new)
                .limit(TOTAL_SHOP_ROOMS)
                .toList();
        final List<BossRoomGenInfo> bossRooms = Stream.generate(BossRoomGenInfo::new)
                .limit(TOTAL_BOSS_ROOMS)
                .toList();
        final List<TreasureRoomGenInfo> treasureRooms = Stream.generate(TreasureRoomGenInfo::new)
                .limit(TOTAL_LOOT_ROOMS)
                .toList();
        startRoomGenInfo.initDimensions();
        normalRooms.forEach(RoomGenInfo::initDimensions);
        shopRooms.forEach(RoomGenInfo::initDimensions);
        bossRooms.forEach(RoomGenInfo::initDimensions);
        treasureRooms.forEach(RoomGenInfo::initDimensions);

        setNeighboursForRooms(startRoomGenInfo, normalRooms, shopRooms, bossRooms, treasureRooms);

        final List<PathRoomGenInfo> pathsRoom = generatePaths(startRoomGenInfo);
        pathsRoom.forEach(RoomGenInfo::initDimensions);

        final List<Room> pathRooms = pathsRoom.stream().map(RoomGenInfo::constructRoom).toList();
        pathRooms.forEach(Room::init);

        final List<Room> allRooms = transform(startRoomGenInfo, normalRooms, shopRooms, bossRooms, treasureRooms);
        allRooms.forEach(Room::init);

        final Room startRoom = allRooms.stream()
                .filter(room -> room instanceof StartRoom) //todo; enums
                .findFirst()
                .orElseThrow(() -> new IllegalStateException("Start room not found"));

        return new MapGenerationResult(startRoom, allRooms, pathRooms);
    }

    private List<PathRoomGenInfo> generatePaths(final StartRoomGenInfo startRoomGenInfo) {
        final List<PathRoomGenInfo> paths = new ArrayList<>();

        final Queue<RoomGenInfo> queue = new ArrayDeque<>(TOTAL_ROOMS);
        final Set<RoomGenInfo> usedRooms = new HashSet<>(TOTAL_ROOMS);

        queue.add(startRoomGenInfo);
        while (!queue.isEmpty()) {
            final RoomGenInfo room = queue.poll();
            room.getNeighbours().forEach((direction, neighbour) -> {
                if (!usedRooms.contains(neighbour)) {
                    final PathRoomGenInfo pathRoom = new PathRoomGenInfo();
                    pathRoom.addNeighbour(room, direction);
                    pathRoom.addNeighbour(neighbour, direction.getOpposite());
                    paths.add(pathRoom);
                    queue.add(neighbour);
                }
            });
            usedRooms.add(room);
        }
        return paths;
    }

    private void setOrientationAround(final RoomGenInfo parent,
                                      final RoomGenInfo child,
                                      final RoomDirection direction) {
        child.setCoordinates(0, 0);

        final int newOffsetX, newOffsetY;

        final int pathWidth = PathRoomGenInfo.MIN_COLS * Tile.TILE_SIZE * Tile.SCALE;
        final int pathHeight = PathRoomGenInfo.MIN_ROWS * Tile.TILE_SIZE * Tile.SCALE;

        final int centerWidthOffset = (parent.getWidth() - child.getWidth()) / 2;
        final int centerHeightOffset = (parent.getHeight() - child.getHeight()) / 2;

        switch (direction) {
            case TOP -> {
                newOffsetX = centerWidthOffset + parent.getXOffset();
                newOffsetY = parent.getYOffset() - parent.getHeight() - pathHeight;
            }
            case BOTTOM -> {
                newOffsetX = centerWidthOffset + parent.getXOffset();
                newOffsetY = parent.getYOffset() + parent.getHeight() + pathHeight;
            }
            case LEFT -> {
                newOffsetX = parent.getXOffset() - parent.getWidth() - pathWidth;
                newOffsetY = centerHeightOffset + parent.getYOffset();
            }
            case RIGHT -> {
                newOffsetX = parent.getXOffset() + parent.getWidth() + pathWidth;
                newOffsetY = centerHeightOffset + parent.getYOffset();
            }
            default -> throw new IllegalStateException("Unexpected value: " + direction);
        }
        child.setCoordinates(newOffsetX, newOffsetY);
    }

    private void setNeighboursForRooms(final StartRoomGenInfo startRoomGenInfo,
                                       final List<CommonRoomGenInfo> normalRooms,
                                       final List<ShopRoomGenInfo> shopRooms,
                                       final List<BossRoomGenInfo> bossRooms,
                                       final List<TreasureRoomGenInfo> treasureRooms) {
        final List<RoomGenInfo> usedRooms = new ArrayList<>(TOTAL_ROOMS);
        final Queue<RoomGenInfo> availableRooms = new ArrayDeque<>(TOTAL_ROOMS);

        availableRooms.addAll(normalRooms);
        availableRooms.addAll(treasureRooms);
        availableRooms.addAll(shopRooms);
        availableRooms.addAll(bossRooms);

        usedRooms.add(startRoomGenInfo);

        final ThreadLocalRandom random = ThreadLocalRandom.current();

        do {
            usedRooms
                    .stream()
                    .filter(RoomGenInfo::canCreateNeighbours)
                    .min(Comparator.comparingDouble(room -> random.nextFloat() + ROOMS_DENSITY * room.getNeighbours().size()))
                    .ifPresentOrElse(room -> {
                        final RoomGenInfo neighbour = availableRooms.peek();
                        final List<RoomDirection> availableDirections = room.getAvailableDirections();
                        while (!availableDirections.isEmpty()) {
                            final RoomDirection direction = availableDirections.get(random.nextInt(availableDirections.size()));

                            setOrientationAround(room, neighbour, direction);

                            if (usedRooms
                                    .stream()
                                    .anyMatch(r -> r.intersects(neighbour))) {
                                availableDirections.remove(direction);
                            } else {
                                room.addNeighbour(neighbour, direction);
                                neighbour.addNeighbour(room, direction.getOpposite());

                                availableRooms.remove();
                                usedRooms.add(neighbour);

                                return;
                            }
                        }
                    }, () -> {
                        LOGGER.error("Fatal error! No room can create neighbours");
                        throw new IllegalStateException("Fatal error! No room can create neighbours");
                    });
        } while (usedRooms.size() < TOTAL_ROOMS);
    }

    private List<Room> transform(final StartRoomGenInfo startRoomGenInfo,
                                 final List<CommonRoomGenInfo> normalRooms,
                                 final List<ShopRoomGenInfo> shopRooms,
                                 final List<BossRoomGenInfo> bossRooms,
                                 final List<TreasureRoomGenInfo> treasureRooms) {
        final List<RoomGenInfo> rooms = new ArrayList<>(TOTAL_ROOMS);

        rooms.add(startRoomGenInfo);
        rooms.addAll(normalRooms);
        rooms.addAll(shopRooms);
        rooms.addAll(bossRooms);
        rooms.addAll(treasureRooms);

        final Map<RoomGenInfo, Room> roomMap = rooms.stream()
                .collect(Collectors.toMap(Function.identity(), RoomGenInfo::constructRoom));
        roomMap.forEach((gen, room) -> {
            gen.getNeighbours()
                    .entrySet()
                    .stream()
                    .map(entry -> Map.entry(entry.getKey(), roomMap.get(entry.getValue())))
                    .forEach(entry -> room.addNeighbour(entry.getKey(), entry.getValue()));
        });
        return roomMap.values()
                .stream()
                .toList();
    }

}
