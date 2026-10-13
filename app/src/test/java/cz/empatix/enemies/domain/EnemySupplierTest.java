package cz.empatix.enemies.domain;

import org.junit.jupiter.api.Test;

import java.util.function.Supplier;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class EnemySupplierTest {

    private static final Supplier<Enemy> NULL_SUPPLIER = () -> null;

    @Test
    void accessorsReturnConstructorArguments() {
        final EnemySupplier s = new EnemySupplier(NULL_SUPPLIER, 2, 5);
        assertSame(NULL_SUPPLIER, s.supplier());
        // record accessors expose the floor bounds
        assertTrue(s.isSpawnable(2));
        assertTrue(s.isSpawnable(5));
    }

    @Test
    void spawnableInclusiveAtBothBounds() {
        final EnemySupplier s = new EnemySupplier(NULL_SUPPLIER, 3, 7);
        assertTrue(s.isSpawnable(3));
        assertTrue(s.isSpawnable(5));
        assertTrue(s.isSpawnable(7));
    }

    @Test
    void notSpawnableOutsideRange() {
        final EnemySupplier s = new EnemySupplier(NULL_SUPPLIER, 3, 7);
        assertFalse(s.isSpawnable(2));
        assertFalse(s.isSpawnable(8));
    }
}
