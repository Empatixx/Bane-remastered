package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.domain.EnemySupplier;
import org.junit.jupiter.api.Test;

import java.lang.reflect.Field;
import java.util.List;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Supplier;

import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.verify;

/**
 * Drives the floor-filtering + random-pick selection of {@link EnemyFactory#createEnemy(int)} and
 * {@link EnemyFactory#createBoss(int)}. The factory normally fills its supplier lists asynchronously
 * via classpath reflection scanning; here the static {@code AtomicReference} fields are seeded
 * directly so the selection logic can be tested deterministically without the live scan.
 */
class EnemyFactoryTest {

    @SuppressWarnings("unchecked")
    private static void seed(final String fieldName, final List<EnemySupplier> value) throws Exception {
        final Field field = EnemyFactory.class.getDeclaredField(fieldName);
        field.setAccessible(true);
        final AtomicReference<List<EnemySupplier>> ref = (AtomicReference<List<EnemySupplier>>) field.get(null);
        ref.set(value);
    }

    private static EnemySupplier supplierFor(final Enemy enemy, final int minFloor, final int maxFloor) {
        final Supplier<Enemy> supplier = () -> enemy;
        return new EnemySupplier(supplier, minFloor, maxFloor);
    }

    @Test
    void createEnemyReturnsAnEnemyFromAFloorEligibleSupplier() throws Exception {
        final Enemy onlyEligible = mock(Enemy.class);
        seed("commonEnemiesSuppliers", List.of(
                supplierFor(onlyEligible, 0, 10),
                supplierFor(mock(Enemy.class), 50, 60) // not eligible for floor 5
        ));

        for (int i = 0; i < 20; i++) {
            final Enemy produced = EnemyFactory.createEnemy(5, new DifficultyService());
            assertSame(onlyEligible, produced,
                    "floor 5 should only ever pick the supplier whose [min,max] covers it");
        }
    }

    @Test
    void createEnemyPicksAmongAllEligibleSuppliers() throws Exception {
        final Enemy a = mock(Enemy.class);
        final Enemy b = mock(Enemy.class);
        seed("commonEnemiesSuppliers", List.of(
                supplierFor(a, 0, 100),
                supplierFor(b, 0, 100)
        ));

        for (int i = 0; i < 20; i++) {
            final Enemy produced = EnemyFactory.createEnemy(3, new DifficultyService());
            assertTrue(produced == a || produced == b);
        }
    }

    @Test
    void createEnemyThrowsWhenNoSupplierMatchesFloor() throws Exception {
        seed("commonEnemiesSuppliers", List.of(
                supplierFor(mock(Enemy.class), 80, 90)
        ));
        assertThrows(RuntimeException.class, () -> EnemyFactory.createEnemy(1, new DifficultyService()));
    }

    @Test
    void createBossReturnsAnEligibleBoss() throws Exception {
        final Enemy boss = mock(Enemy.class);
        seed("bossEnemiesSuppliers", List.of(
                supplierFor(boss, 0, 10)
        ));

        final Enemy produced = EnemyFactory.createBoss(4, new DifficultyService());
        assertNotNull(produced);
        assertSame(boss, produced);
    }

    @Test
    void createBossThrowsWhenNoBossMatchesFloor() throws Exception {
        seed("bossEnemiesSuppliers", List.of(
                supplierFor(mock(Enemy.class), 20, 30)
        ));
        assertThrows(RuntimeException.class, () -> EnemyFactory.createBoss(1, new DifficultyService()));
    }

    @Test
    void createEnemyScalesTheSpawnedEnemyToTheFloor() throws Exception {
        final Enemy enemy = mock(Enemy.class);
        final DifficultyService difficulty = new DifficultyService();
        seed("commonEnemiesSuppliers", List.of(supplierFor(enemy, 0, 100)));

        EnemyFactory.createEnemy(7, difficulty);

        verify(enemy).scaleToFloor(difficulty, 7);
    }

    @Test
    void createBossScalesTheSpawnedBossToTheFloor() throws Exception {
        final Enemy boss = mock(Enemy.class);
        final DifficultyService difficulty = new DifficultyService();
        seed("bossEnemiesSuppliers", List.of(supplierFor(boss, 0, 100)));

        EnemyFactory.createBoss(7, difficulty);

        verify(boss).scaleToFloor(difficulty, 7);
    }

    @Test
    void enemySupplierIsSpawnableHonorsInclusiveBounds() {
        final EnemySupplier supplier = supplierFor(mock(Enemy.class), 2, 4);
        assertTrue(supplier.isSpawnable(2));
        assertTrue(supplier.isSpawnable(4));
        assertTrue(!supplier.isSpawnable(1));
        assertTrue(!supplier.isSpawnable(5));
    }
}
