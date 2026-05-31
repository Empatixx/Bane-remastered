package cz.empatix.items.service;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
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
import cz.empatix.testsupport.MockedAssets;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockito.MockedStatic;

import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.mockStatic;
import static org.mockito.Mockito.when;

/** Exercises the supplier wiring of {@link ItemFactory} for each produced item type. */
class ItemFactoryTest {

    @BeforeAll
    static void headlessAudio() {
        ALInit.canUseAL = false;
    }

    @BeforeEach
    void physics() {
        new cz.empatix.newphysics.service.PhysicsService();
    }

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
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemDrop drop = newFactory().createCoinDrop();
            assertInstanceOf(CoinItem.class, drop);
        }
    }

    @Test
    void createWeaponDropProducesWeaponDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemDrop drop = newFactory().createWeaponDrop();
            assertInstanceOf(WeaponDrop.class, drop);
        }
    }

    @Test
    void createArtefactDropProducesArtefactDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemDrop drop = newFactory().createArtefactDrop();
            assertInstanceOf(ArtefactDrop.class, drop);
        }
    }

    @Test
    void createItemDropWithCommonRarityProducesACommonItem() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemDrop drop = newFactory().createItemDrop(ItemRarity.COMMON);
            // COMMON pool = coin or one of the three ammo boxes.
            assertNotNull(drop);
            final boolean expected = drop instanceof CoinItem || drop instanceof AmmoBoxItem;
            org.junit.jupiter.api.Assertions.assertTrue(expected, "unexpected common drop: " + drop.getClass());
        }
    }

    @Test
    void createItemDropWithUncommonRarityProducesRecoverPotion() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ItemDrop drop = newFactory().createItemDrop(ItemRarity.UNCOMMON);
            assertInstanceOf(RecoverPotion.class, drop);
        }
    }

    @Test
    void createMovableItemDropWrapsInMovableDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final MovableItemDrop drop = newFactory().createMovableItemDrop(ItemRarity.COMMON);
            assertNotNull(drop);
        }
    }

    @Test
    void createMovableCoinDropWrapsCoinDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final MovableItemDrop drop = newFactory().createMovableCoinDrop();
            assertNotNull(drop);
        }
    }

    @Test
    void createMovableWeaponDropWrapsWeaponDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Gun gun = mock(Gun.class);
            when(gun.getTexturePath()).thenReturn("guns/pistol.tga");
            final MovableItemDrop drop = newFactory().createMovableWeaponDrop(gun, g -> {});
            assertNotNull(drop);
        }
    }

    @Test
    void createMovableArtefactDropWrapsArtefactDrop() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final Artefact artefact = mock(Artefact.class);
            when(artefact.getTexturePath()).thenReturn("artefacts/shield.tga");
            final MovableItemDrop drop = newFactory().createMovableArtefactDrop(artefact, a -> {});
            assertNotNull(drop);
        }
    }

    @Test
    void createShopDropProducesPricedShopItem() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final ShopItem item = newFactory().createShopDrop(ShopType.AMMUNITION, 1);
            assertNotNull(item);
            assertInstanceOf(ShopItem.class, item);
        }
    }

    @Test
    void createSacrificialItemProducesSacrificialItem() {
        try (MockedStatic<AssetsService> assets = mockStatic(AssetsService.class)) {
            MockedAssets.stub(assets);
            final SacrificialItem item = newFactory().createSacrificialItem(ShopType.DEFENSIVE, 2);
            assertNotNull(item);
            assertInstanceOf(SacrificialItem.class, item);
        }
    }
}
