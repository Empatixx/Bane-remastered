package cz.empatix.items.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.guns.service.GunService;
import cz.empatix.items.domain.AmmoBoxItem;
import cz.empatix.items.domain.ArtefactDrop;
import cz.empatix.items.domain.CoinItem;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.domain.ItemRarity;
import cz.empatix.items.domain.MovableItemDrop;
import cz.empatix.items.domain.RecoverPotion;
import cz.empatix.items.domain.SacrificialItem;
import cz.empatix.items.domain.ShopItem;
import cz.empatix.items.domain.ShopType;
import cz.empatix.items.domain.WeaponDrop;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/** Exercises the supplier wiring of {@link ItemFactory} for each produced item type. */
@ExtendWith(HeadlessAssets.class)
class ItemFactoryTest {

    private GunService gunService;
    private CoinService coinService;
    private Player player;
    private ArtefactsService artefactsService;

    private ItemFactory newFactory() {
        gunService = mock(GunService.class);
        coinService = mock(CoinService.class);
        player = mock(Player.class);
        artefactsService = mock(ArtefactsService.class);

        final Gun gun = mock(Gun.class);
        when(gun.getTexturePath()).thenReturn("guns/pistol.tga");
        when(gunService.getUnusedRandomGun()).thenReturn(gun);

        final Artefact artefact = mock(Artefact.class);
        when(artefact.getTexturePath()).thenReturn("artefacts/shield.tga");
        when(artefactsService.getUnusedRandomArtefact()).thenReturn(artefact);

        return new ItemFactory(gunService, coinService, player, artefactsService);
    }

    @Test
    void createCoinDropProducesCoinItem() {
        final ItemDrop drop = newFactory().createCoinDrop();
        assertInstanceOf(CoinItem.class, drop);
    }

    @Test
    void createWeaponDropProducesWeaponDrop() {
        final ItemDrop drop = newFactory().createWeaponDrop();
        assertInstanceOf(WeaponDrop.class, drop);
    }

    @Test
    void createArtefactDropProducesArtefactDrop() {
        final ItemDrop drop = newFactory().createArtefactDrop();
        assertInstanceOf(ArtefactDrop.class, drop);
    }

    @Test
    void createItemDropWithCommonRarityProducesACommonItem() {
        final ItemDrop drop = newFactory().createItemDrop(ItemRarity.COMMON);
        // COMMON pool = coin or one of the three ammo boxes.
        assertNotNull(drop);
        final boolean expected = drop instanceof CoinItem || drop instanceof AmmoBoxItem;
        org.junit.jupiter.api.Assertions.assertTrue(expected, "unexpected common drop: " + drop.getClass());
    }

    @Test
    void createItemDropWithUncommonRarityProducesRecoverPotion() {
        final ItemDrop drop = newFactory().createItemDrop(ItemRarity.UNCOMMON);
        assertInstanceOf(RecoverPotion.class, drop);
    }

    @Test
    void createMovableItemDropWrapsInMovableDrop() {
        final MovableItemDrop drop = newFactory().createMovableItemDrop(ItemRarity.COMMON);
        assertNotNull(drop);
    }

    @Test
    void createMovableCoinDropWrapsCoinDrop() {
        final MovableItemDrop drop = newFactory().createMovableCoinDrop();
        assertNotNull(drop);
    }

    @Test
    void createMovableWeaponDropWrapsWeaponDrop() {
        final Gun gun = mock(Gun.class);
        when(gun.getTexturePath()).thenReturn("guns/pistol.tga");
        final MovableItemDrop drop = newFactory().createMovableWeaponDrop(gun, g -> {});
        assertNotNull(drop);
    }

    @Test
    void createMovableArtefactDropWrapsArtefactDrop() {
        final Artefact artefact = mock(Artefact.class);
        when(artefact.getTexturePath()).thenReturn("artefacts/shield.tga");
        final MovableItemDrop drop = newFactory().createMovableArtefactDrop(artefact, a -> {});
        assertNotNull(drop);
    }

    @Test
    void createShopDropProducesPricedShopItem() {
        final ShopItem item = newFactory().createShopDrop(ShopType.AMMUNITION, 1);
        assertNotNull(item);
        assertInstanceOf(ShopItem.class, item);
    }

    @Test
    void createSacrificialItemProducesSacrificialItem() {
        final SacrificialItem item = newFactory().createSacrificialItem(ShopType.DEFENSIVE, 2);
        assertNotNull(item);
        assertInstanceOf(SacrificialItem.class, item);
    }
}
