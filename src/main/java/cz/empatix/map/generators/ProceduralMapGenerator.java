package cz.empatix.map.generators;

import com.google.common.base.Preconditions;
import cz.empatix.map.domain.*;
import cz.empatix.map.service.MapService;
import cz.empatix.rooms.service.PathRoom;
import cz.empatix.rooms.service.Room;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.Stream;

public class ProceduralMapGenerator implements MapGenerator {
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
    public MapService generate() {
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

        normalRooms.forEach(RoomGenInfo::initDimensions);
        shopRooms.forEach(RoomGenInfo::initDimensions);
        bossRooms.forEach(RoomGenInfo::initDimensions);
        treasureRooms.forEach(RoomGenInfo::initDimensions);

        setNeighboursForRooms(startRoomGenInfo, normalRooms, shopRooms, bossRooms, treasureRooms);
        setPositionsForRooms(startRoomGenInfo);

        final List<PathRoomGenInfo> pathsRoom = generatePaths(startRoomGenInfo);
        final List<? extends RoomGenInfo> allRooms = Stream.of(List.of(startRoomGenInfo), normalRooms, shopRooms, bossRooms, treasureRooms)
                .flatMap(List::stream)
                .toList();
        final List<Room> rooms = allRooms.stream()
                .map(RoomGenInfo::constructRoom)
                .toList();
        final List<Room> pathRooms = pathsRoom.stream()
                .map(RoomGenInfo::constructRoom)
                .toList();
        return new MapService(rooms, pathRooms);
    }

    private List<PathRoomGenInfo> generatePaths(final StartRoomGenInfo startRoomGenInfo) {
        final List<PathRoomGenInfo> paths = new ArrayList<>();

        final Queue<RoomGenInfo> queue = new ArrayDeque<>(TOTAL_ROOMS);
        final Set<RoomGenInfo> alreadyTraversed = new HashSet<>(TOTAL_ROOMS);

        queue.add(startRoomGenInfo);
        while(!queue.isEmpty()){
            final RoomGenInfo room = queue.poll();
            final List<RoomGenInfo> availableNeighbours = room.getNeighbours().stream()
                    .filter(neighbour -> !alreadyTraversed.contains(neighbour))
                    .toList();
            for(final RoomGenInfo neighbour : availableNeighbours) {
                final PathRoomGenInfo pathRoom = new PathRoomGenInfo();
                pathRoom.addNeighbour(room);
                pathRoom.addNeighbour(neighbour);
                paths.add(pathRoom);
            }
            alreadyTraversed.add(room);
        }
        return paths;
    }

    private void setPositionsForRooms(final RoomGenInfo start) {
        setPositionForRooms(start, start.getNeighbours(), new HashSet<>(TOTAL_ROOMS));
    }

    private void setPositionForRooms(final RoomGenInfo parent,
                                     final List<RoomGenInfo> neighbours,
                                     final Set<RoomGenInfo> usedRooms) {
        usedRooms.add(parent);
        for (final RoomGenInfo info : neighbours) {
            if (usedRooms.contains(info)) {
                continue;
            }
            do{
                info.setOrientationAround(parent);
            } while(usedRooms.stream().anyMatch(usedRoom -> usedRoom.intersects(info)));
            setPositionForRooms(info, info.getNeighbours(), usedRooms);
        }
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
                    .min(Comparator.comparingInt(room -> random.nextInt(room.getNeighbours().size())))
                    .ifPresentOrElse(room -> {
                        final RoomGenInfo neighbour = availableRooms.remove();
                        room.addNeighbour(neighbour);
                        neighbour.addNeighbour(room);
                        usedRooms.add(neighbour);
                    }, () -> {
                        LOGGER.error("Fatal error! No room can create neighbours");
                        throw new IllegalStateException("Fatal error! No room can create neighbours");
                    });
        } while (usedRooms.size() < TOTAL_ROOMS);
    }

}
