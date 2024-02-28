package cz.empatix.upgrades.appliers;

import cz.empatix.map.service.MapService;
import cz.empatix.player.ActiveUpgradeService;
import cz.empatix.player.Player;
import cz.empatix.roomobjects.domain.Trap;
import cz.empatix.rooms.domain.RoomState;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.ShopRoom;
import cz.empatix.upgrades.domain.PlayerUpgradeName;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class MapUpgradesApplier implements UpgradesApplier {
    private static final Logger LOGGER = LoggerFactory.getLogger(MapUpgradesApplier.class);
    private static final ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final MapService mapService;
    private final Player player;
    private final ActiveUpgradeService activeUpgradeService;
    public MapUpgradesApplier(final MapService mapService,
                              final Player player,
                              final ActiveUpgradeService activeUpgradeService) {
        this.mapService = mapService;
        this.player = player;
        this.activeUpgradeService = activeUpgradeService;
    }

    @Override
    public void apply() {
        if (activeUpgradeService.isBought(PlayerUpgradeName.DISCOUNT)){
            final List<ShopRoom> shopRooms = mapService.getRooms()
                    .stream()
                    .filter(room -> room.getRoomType().equals(RoomType.SHOP))
                    .map(room -> (ShopRoom) room)
                    .toList();
            shopRooms.forEach(ShopRoom::applyDiscounts);
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.ARMOR_REGENERATION)){
            mapService.onRoomEventChange(event -> {
                if (event.state().equals(RoomState.OPEN) && event.lastState().equals(RoomState.CLOSED)){
                    player.addArmor(1);
                }
            });
        }
        if (activeUpgradeService.isBought(PlayerUpgradeName.TRAP_MASTER)){
            mapService.getRooms().stream()
                    .flatMap(room -> room.getRoomObjects().stream())
                    .filter(object -> object instanceof Trap)
                    .map(object -> (Trap) object)
                    .filter(trap -> RANDOM.nextInt(100) <= 30)
                    .forEach(trap -> {
                        LOGGER.info("Deactivating trap {}", trap);
                        trap.deactivate();
                    });
        }
    }
}
