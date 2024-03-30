package cz.empatix.items.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.guns.service.GunService;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.*;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;

import java.util.EnumMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ThreadLocalRandom;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Supplier;

public class ItemFactory {
    private final static ThreadLocalRandom RANDOM = ThreadLocalRandom.current();
    private final Map<ItemRarity, List<Supplier<ItemDrop>>> itemDropSuppliers;
    private final Function<Integer,ShopItem> shopDropSupplier;

    public ItemFactory(final GunService gunService,
                       final CoinService coinService,
                       final Player player,
                       final ArtefactsService artefactsService) {
        final List<Supplier<ItemDrop>> commonItemsSuppliers = List.of(
                () -> new CoinItem(coinService::addCoins)
        );
        final List<Supplier<ItemDrop>> uncommonItemsSuppliers = List.of(
                () -> new RecoverPotion(RecoverSize.SMALL, RecoverType.HEALTH, player::hasFullHealth, player::addHealth),
                () -> new RecoverPotion(RecoverSize.SMALL, RecoverType.ARMOR, player::hasFullArmor, player::addArmor)
        );
        final List<Supplier<ItemDrop>> rareItemsSuppliers = List.of(
                () -> new WeaponDrop(DropSource.NORMAL, gunService.getUnusedRandomGun(), gunService::addGun),
                () -> new ArtefactDrop(DropSource.NORMAL, artefactsService.getUnusedRandomArtefact(), artefactsService::addArtefact),
                () -> new AmmoBoxItem(gunService::hasNotFullAmmo, gunService::addAmmo),
                () -> new RecoverPotion(RecoverSize.LARGE, RecoverType.HEALTH, player::hasFullHealth, player::addHealth),
                () -> new RecoverPotion(RecoverSize.LARGE, RecoverType.ARMOR, player::hasFullArmor, player::addArmor)
        );
        itemDropSuppliers = new EnumMap<>(ItemRarity.class);
        itemDropSuppliers.put(ItemRarity.COMMON, commonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.UNCOMMON, uncommonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.RARE, rareItemsSuppliers);

        shopDropSupplier = floor -> {
            final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.randomRarity(ItemRarity.RARE));
            final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
            return new ShopItem(floor, drop, coinService::hasEnoughCoins, coinService::removeCoins);
        };
    }
    public MovableItemDrop createMovableWeaponDrop(final Gun gun, final Consumer<Gun> addGun) {
        final WeaponDrop drop = new WeaponDrop(DropSource.PLAYER, gun, addGun);
        final MovableItemDrop moveDrop = new MovableItemDrop(drop);
        InteractionService.getInstance().addInteractable(moveDrop);
        return moveDrop;
    }
    public MovableItemDrop createMovableArtefactDrop(final Artefact currentArtefact, final Consumer<Artefact> addArtefact) {
        final ArtefactDrop drop = new ArtefactDrop(DropSource.PLAYER, currentArtefact, addArtefact);
        final MovableItemDrop moveDrop = new MovableItemDrop(drop);
        InteractionService.getInstance().addInteractable(moveDrop);
        return moveDrop;
    }
    public ItemDrop createWeaponDrop() {
        final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.RARE);
        final ItemDrop drop = suppliers.get(0).get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }
    public ItemDrop createArtefactDrop() {
        final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.RARE);
        final ItemDrop drop = suppliers.get(1).get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }

    public ShopItem createShopDrop(final int floor) {
        final ShopItem drop = shopDropSupplier.apply(floor);
        drop.setDespawnable(false);
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }

    public ItemDrop createItemDrop() {
        final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.randomRarity(ItemRarity.COMMON, ItemRarity.UNCOMMON));
        final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
        InteractionService.getInstance().addInteractable(drop);
        return drop;
    }
    public MovableItemDrop createMovableItemDrop() {
        final List<Supplier<ItemDrop>> suppliers = itemDropSuppliers.get(ItemRarity.randomRarity(ItemRarity.COMMON));
        final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
        InteractionService.getInstance().addInteractable(drop);
        return new MovableItemDrop(drop);
    }
}
