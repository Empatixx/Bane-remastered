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
    private final Map<ShopType, List<Supplier<ItemDrop>>> shopDropSuppliers;
    private final Function<ItemDrop, ShopItem> shopDropSupplier;
    private final Function<ItemDrop, SacrificialItem> sacrificialItemSupplier;
    public ItemFactory(final GunService gunService,
                       final CoinService coinService,
                       final Player player,
                       final ArtefactsService artefactsService) {
        final Supplier<ItemDrop> coinSupplier = () -> new CoinItem(coinService::addCoins);
        final Supplier<ItemDrop> recoverHealthSupplier = () -> new RecoverPotion(RecoverSize.SMALL, RecoverType.HEALTH, player::hasFullHealth, player::addHealth);
        final Supplier<ItemDrop> recoverArmorSupplier = () -> new RecoverPotion(RecoverSize.SMALL, RecoverType.ARMOR, player::hasFullArmor, player::addArmor);
        final Supplier<ItemDrop> weaponSupplier = () -> new WeaponDrop(DropSource.NORMAL, gunService.getUnusedRandomGun(), gunService::addGun);
        final Supplier<ItemDrop> artefactSupplier = () -> new ArtefactDrop(DropSource.NORMAL, artefactsService.getUnusedRandomArtefact(), artefactsService::addArtefact);

        final Supplier<ItemDrop> pistolAmmoSupplier = () -> new AmmoBoxItem(AmmoBoxType.PISTOL, gunService::hasNotFullAmmo, gunService::addAmmo);
        final Supplier<ItemDrop> shotgunAmmoSupplier = () -> new AmmoBoxItem(AmmoBoxType.SHOTGUN, gunService::hasNotFullAmmo, gunService::addAmmo);
        final Supplier<ItemDrop> submachineAmmoSupplier = () -> new AmmoBoxItem(AmmoBoxType.SUBMACHINE, gunService::hasNotFullAmmo, gunService::addAmmo);
        final Supplier<ItemDrop> universalAmmoSupplier = () -> new AmmoBoxItem(AmmoBoxType.UNIVERSAL, gunService::hasNotFullAmmo, gunService::addAmmo);

        final Supplier<ItemDrop> recoverHealthLargeSupplier = () -> new RecoverPotion(RecoverSize.LARGE, RecoverType.HEALTH, player::hasFullHealth, player::addHealth);
        final Supplier<ItemDrop> recoverArmorLargeSupplier = () -> new RecoverPotion(RecoverSize.LARGE, RecoverType.ARMOR, player::hasFullArmor, player::addArmor);

        final List<Supplier<ItemDrop>> commonItemsSuppliers = List.of(coinSupplier, pistolAmmoSupplier, shotgunAmmoSupplier, submachineAmmoSupplier);
        final List<Supplier<ItemDrop>> uncommonItemsSuppliers = List.of(recoverHealthSupplier, recoverArmorSupplier);
        final List<Supplier<ItemDrop>> rareItemsSuppliers = List.of(weaponSupplier, artefactSupplier, universalAmmoSupplier, recoverHealthLargeSupplier, recoverArmorLargeSupplier);

        itemDropSuppliers = new EnumMap<>(ItemRarity.class);
        itemDropSuppliers.put(ItemRarity.COMMON, commonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.UNCOMMON, uncommonItemsSuppliers);
        itemDropSuppliers.put(ItemRarity.RARE, rareItemsSuppliers);

        shopDropSuppliers = new EnumMap<>(ShopType.class);
        shopDropSuppliers.put(ShopType.OFFENSIVE, List.of(weaponSupplier, artefactSupplier));
        shopDropSuppliers.put(ShopType.AMMUNITION, List.of(universalAmmoSupplier));
        shopDropSuppliers.put(ShopType.DEFENSIVE, List.of(recoverHealthLargeSupplier, recoverArmorLargeSupplier));

        shopDropSupplier = drop -> {
            return new ShopItem(drop, coinService::hasEnoughCoins, coinService::removeCoins);
        };
        sacrificialItemSupplier = drop -> {
            return new SacrificialItem(drop, player::hasEnoughHealth, player::removeHealth);
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

    public ShopItem createShopDrop(final ShopType type, final int floor) {
        final List<Supplier<ItemDrop>> suppliers = shopDropSuppliers.get(type);
        final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
        final ShopItem shopDrop = shopDropSupplier.apply(drop);
        shopDrop.setPrice(type, floor);
        shopDrop.setDespawnable(false);
        InteractionService.getInstance().addInteractable(shopDrop);
        return shopDrop;
    }

    public SacrificialItem createSacrificialItem(final ShopType type, final int floor) {
        final List<Supplier<ItemDrop>> suppliers = shopDropSuppliers.get(type);
        final ItemDrop drop = suppliers.get(RANDOM.nextInt(suppliers.size())).get();
        final SacrificialItem sacrificialItem = sacrificialItemSupplier.apply(drop);
        sacrificialItem.setDespawnable(false);
        InteractionService.getInstance().addInteractable(sacrificialItem);
        return sacrificialItem;
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
