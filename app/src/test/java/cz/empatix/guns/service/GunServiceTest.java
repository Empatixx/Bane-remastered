package cz.empatix.guns.service;

import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.items.domain.AmmoBoxType;
import cz.empatix.items.service.ItemService;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;

/**
 * Exercises the real {@link GunService} headlessly: it builds the full real gun roster (each gun
 * fetches GPU textures in its ctor) plus a {@code CommonImage} HUD, all served by the stubbed
 * {@link cz.empatix.assets.service.AssetsService} from {@link HeadlessAssets}. The static
 * initializer that fetches {@code ingame/weaponslot.tga} also runs under the active mockStatic.
 */
@ExtendWith(HeadlessAssets.class)
class GunServiceTest {

    private GunService newService() {
        final ItemService itemService = mock(ItemService.class);
        final StatisticsService statisticsService = mock(StatisticsService.class);
        return new GunService(itemService, statisticsService, new RngService(0L));
    }

    @Test
    void constructsSevenGunRoster() {
        final GunService service = newService();
        final List<Gun> guns = service.getGuns();
        assertEquals(7, guns.size());
        guns.forEach(g -> assertNotNull(g.getTexturePath()));
    }

    @Test
    void getUnusedRandomGunReturnsAGunNotInTheStartingSlots() {
        final GunService service = newService();
        final List<Gun> guns = service.getGuns();
        // default slots hold guns 0 (Pistol) and 1 (Shotgun)
        final Gun starterA = guns.get(0);
        final Gun starterB = guns.get(1);

        for (int i = 0; i < 25; i++) {
            final Gun unused = service.getUnusedRandomGun();
            assertNotNull(unused);
            assertTrue(guns.contains(unused));
            assertFalse(unused == starterA || unused == starterB,
                    "getUnusedRandomGun must not return a gun already in a slot");
        }
    }

    @Test
    void universalAmmoBoxConsideredNotFullAfterAmmoIsConsumed() {
        final GunService service = newService();
        // Drain every gun's ammo via a (negative) percent so at least one becomes not-full.
        service.addAmmo(-100f, AmmoBoxType.UNIVERSAL);
        assertTrue(service.hasNotFullAmmo(AmmoBoxType.UNIVERSAL),
                "after draining, the universal pool should report not-full");
    }

    @Test
    void addAmmoRefillsWithoutThrowingForEveryBoxType() {
        final GunService service = newService();
        // none of these should throw; they fan out to the matching guns in the slots
        service.addAmmo(10f, AmmoBoxType.PISTOL);
        service.addAmmo(10f, AmmoBoxType.SHOTGUN);
        service.addAmmo(10f, AmmoBoxType.UNIVERSAL);
    }

    @Test
    void addSlotAllowsAThirdGunWithoutDropping() {
        final GunService service = newService();
        // Slots start full (2 guns). Adding a slot then a gun should NOT trigger a drop.
        service.addSlot();
        service.addGun(service.getUnusedRandomGun());
        // No interaction with itemService.add expected: the verify lives in the drop path only,
        // here we simply assert it completes and the roster is unchanged.
        assertEquals(7, service.getGuns().size());
    }

    @Test
    void addAndRemoveHookAreIdempotentNoOps() {
        final GunService service = newService();
        final GunDamageEventHook hook = mock(GunDamageEventHook.class);
        // adding then removing a hook must not throw
        service.addHook(hook);
        service.removeHook(hook);
        service.removeHook(hook); // removing again is a harmless no-op
    }

    @Test
    void setShootingOnDefaultCurrentGunDoesNotThrow() {
        final GunService service = newService();
        service.setShooting(true);
        service.setShooting(false);
    }
}
