package cz.empatix.integration;

import cz.empatix.artefacts.service.ArtefactsService;
import cz.empatix.common.RngService;
import cz.empatix.death.service.StatisticsService;
import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.service.EnemyDropService;
import cz.empatix.guns.service.GunService;
import cz.empatix.items.domain.ItemDrop;
import cz.empatix.items.service.ItemFactory;
import cz.empatix.items.service.ItemService;
import cz.empatix.player.CoinService;
import cz.empatix.player.Player;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.extension.ExtendWith;

import java.lang.reflect.Field;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless INTEGRATION test of the death -> loot bridge: a real {@link EnemyDropService} backed by a
 * real {@link ItemFactory}/{@link ItemService} rolls a drop for a slain enemy and deposits a real,
 * positioned {@link ItemDrop} into the world. The enemy is a positioned stub (the system under test is
 * the drop-service -> factory -> item-service chain); the drop roll is made deterministic by forcing
 * the kill-streak counter so the guaranteed-drop branch fires.
 */
@Tag("integration")
@ExtendWith(HeadlessAssets.class)
class DeathDropIntegrationTest {

    private static void forceGuaranteedDrop() throws Exception {
        final Field f = EnemyDropService.class.getDeclaredField("enemiesKilledWithoutDrops");
        f.setAccessible(true);
        f.setInt(null, 10); // 10 + rand(6) > GUARANTEED_DROP(4) is always true
    }

    @Test
    void killingAnEnemyDropsAPositionedItemIntoTheWorld() throws Exception {
        final ItemService itemService = new ItemService(mock(StatisticsService.class));
        final ItemFactory factory = new ItemFactory(
                mock(GunService.class), new CoinService(), mock(Player.class), mock(ArtefactsService.class), new RngService(0L));
        itemService.setFactory(factory); // EnemyDropService reads the factory off the service
        final EnemyDropService dropService = new EnemyDropService(itemService, new RngService(0L));

        final Enemy enemy = mock(Enemy.class);
        when(enemy.getX()).thenReturn(300f);
        when(enemy.getY()).thenReturn(200f);
        when(enemy.getWidth()).thenReturn(40);
        when(enemy.getHeight()).thenReturn(40);

        forceGuaranteedDrop();
        dropService.tryDrop(enemy);

        assertEquals(1, itemService.getItems().size(), "a guaranteed kill drop reaches the item service");
        final ItemDrop drop = itemService.getItems().get(0);
        // Positioned around the enemy's centre, so it lands inside the enemy's footprint.
        assertTrue(drop.getPosition().x() >= 300 && drop.getPosition().x() <= 340,
                "drop spawns at the dead enemy's position");
        assertTrue(drop.getPosition().y() >= 200 && drop.getPosition().y() <= 240);
    }
}
