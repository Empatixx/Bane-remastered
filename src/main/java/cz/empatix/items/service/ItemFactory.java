package cz.empatix.items.service;

import cz.empatix.guns.service.GunService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.*;
import cz.empatix.player.Player;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Supplier;

public class ItemFactory {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final Map<ItemRarity, List<Supplier<ItemDrop>>> itemDropSuppliers;
    private final Supplier<ShopItem> shopDropSupplier;

    public ItemFactory(final GunService gunService,
                       final Player player) {
        final List<Supplier<ItemDrop>> commonItemsSuppliers = List.of(
                () -> new CoinItem(player::addCoins)
        );
        final List<Supplier<ItemDrop>> uncommonItemsSuppliers = List.of(
                () -> new RecoverPotion(RecoverType.HEALTH, player::hasFullHealth, player::addHealth),
                () -> new RecoverPotion(RecoverType.ARMOR, player::hasFullArmor, player::addArmor)
        );
        final List<Supplier<ItemDrop>> rareItemsSuppliers = List.of(
                () -> new AmmoItem(gunService::hasNotFullAmmo, gunService::addAmmo)
        );
        itemDropSuppliers = new EnumMap<>(ItemRarity.class);
        itemDropSuppliers.put(ItemRarity.COMMON, commonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.UNCOMMON, uncommonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.RARE, rareItemsSuppliers);

        shopDropSupplier = () -> {
            final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.randomRarity(ItemRarity.UNCOMMON, ItemRarity.RARE));
            final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
            return new ShopItem(drop, player::hasEnoughCoins, player::removeCoins);
        };
    }

    public ShopItem createShopDrop() {
        final ShopItem drop = shopDropSupplier.get();
        drop.setDespawnable(false);
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }

    public ItemDrop createItemDrop() {
        final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.randomRarity(ItemRarity.COMMON));
        final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }
}
