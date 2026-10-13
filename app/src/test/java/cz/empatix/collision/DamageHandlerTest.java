package cz.empatix.collision;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

class DamageHandlerTest {

    /** Records the last hit it received and reports a fixed amount of damage absorbed. */
    private static final class FakeDamageable implements Damageable {
        private DamageSourceType lastType;
        private int lastDamage;
        private int hits;
        private final int absorb;

        FakeDamageable(final int absorb) {
            this.absorb = absorb;
        }

        @Override
        public int hit(final DamageSourceType source, final int damage) {
            this.lastType = source;
            this.lastDamage = damage;
            this.hits++;
            return absorb;
        }
    }

    /** Reports a fixed damage/type and records what postDamage received back. */
    private static final class FakeDamageSource implements DamageSource {
        private final int damage;
        private final DamageSourceType type;
        private Integer postDamageReceived;

        FakeDamageSource(final int damage, final DamageSourceType type) {
            this.damage = damage;
            this.type = type;
        }

        @Override
        public int damage() {
            return damage;
        }

        @Override
        public DamageSourceType type() {
            return type;
        }

        @Override
        public void postDamage(final int damage) {
            this.postDamageReceived = damage;
        }
    }

    private final DamageHandler handler = new DamageHandler();

    @Test
    void appliesDamageFromSourceToTarget() {
        final FakeDamageable target = new FakeDamageable(7);
        final FakeDamageSource source = new FakeDamageSource(20, DamageSourceType.ENEMY);

        handler.onCollision(source, target);

        assertEquals(1, target.hits);
        assertEquals(20, target.lastDamage);
        assertEquals(DamageSourceType.ENEMY, target.lastType);
        // postDamage gets the value hit() returned (damage actually absorbed)
        assertEquals(7, source.postDamageReceived);
    }

    @Test
    void appliesDamageRegardlessOfArgumentOrder() {
        final FakeDamageable target = new FakeDamageable(0);
        final FakeDamageSource source = new FakeDamageSource(5, DamageSourceType.TRAP);

        // target passed first, source second — handler tries both directions
        handler.onCollision(target, source);

        assertEquals(1, target.hits);
        assertEquals(5, target.lastDamage);
    }

    @Test
    void skipsWhenDamageIsNonPositive() {
        final FakeDamageable target = new FakeDamageable(3);
        final FakeDamageSource source = new FakeDamageSource(0, DamageSourceType.PLAYER);

        handler.onCollision(source, target);

        assertEquals(0, target.hits);
        assertNull(source.postDamageReceived);
    }

    @Test
    void ignoresPairsThatAreNotDamageRelated() {
        assertDoesNotThrow(() -> handler.onCollision(new Object(), new Object()));
    }
}
