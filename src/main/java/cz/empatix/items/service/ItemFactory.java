package cz.empatix.items.service;

import cz.empatix.guns.service.GunService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.*;
import cz.empatix.player.Player;

import java.util.List;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class ItemFactory {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final List<Supplier<ItemDrop>> itemDropSuppliers;
    private final Supplier<ItemDrop> shopDropSupplier;

    public ItemFactory(final GunService gunService,
                       final Player player) {
        itemDropSuppliers = List.of(
                () -> new AmmoItem(gunService::hasNotFullAmmo, gunService::addAmmo),
                () -> new RecoverPotion(RecoverType.HEALTH, player::hasFullHealth, player::addHealth),
                () -> new RecoverPotion(RecoverType.ARMOR, player::hasFullArmor, player::addArmor),
                () -> new CoinItem(player::addCoins)
        );
        shopDropSupplier = () -> {
            final ItemDrop drop = itemDropSuppliers.get(RANDOM.nextInt(itemDropSuppliers.size())).get();
            return new ShopItem(drop, player::hasEnoughCoins, player::removeCoins);
        };
    }

    public ItemDrop createShopDrop() {
        final ItemDrop drop = shopDropSupplier.get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }

    public ItemDrop createItemDrop() {
        final ItemDrop drop = itemDropSuppliers.get(RANDOM.nextInt(itemDropSuppliers.size())).get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }
}
