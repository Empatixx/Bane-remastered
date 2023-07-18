package cz.empatix.items.service;

import cz.empatix.guns.service.GunService;
import cz.empatix.items.domain.AmmoItem;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.domain.RecoverPotion;
import cz.empatix.items.domain.RecoverType;
import cz.empatix.player.Player;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class ItemFactory {
    private final List<Supplier<ItemDrop>> itemDropSuppliers;
    public ItemFactory(final GunService gunService,
                       final Player player) {
        itemDropSuppliers = List.of(
                () -> new AmmoItem(gunService::hasNotFullAmmo, gunService::addAmmo),
                () -> new RecoverPotion(RecoverType.HEALTH, player::hasFullHealth, player::addHealth),
                () -> new RecoverPotion(RecoverType.ARMOR, player::hasFullArmor, player::addArmor)
        );
    }

    public ItemDrop createItemDrop() {
        return itemDropSuppliers.get(ThreadLocalRandom.current().nextInt(itemDropSuppliers.size())).get();
    }
}
