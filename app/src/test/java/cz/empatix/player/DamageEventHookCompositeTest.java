package cz.empatix.player;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

class DamageEventHookCompositeTest {

    @Test
    void emptyCompositeReturnsInputUnchanged() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        assertEquals(42, composite.onDamage(42));
    }

    @Test
    void singleHookTransformsValue() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        composite.addHook(damage -> damage + 5);
        assertEquals(15, composite.onDamage(10));
    }

    @Test
    void hooksInvokedInOrderAndValueThreadedThrough() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        final List<Integer> seen = new ArrayList<>();

        composite.addHook(damage -> {
            seen.add(damage);
            return damage + 1; // 10 -> 11
        });
        composite.addHook(damage -> {
            seen.add(damage);
            return damage * 2; // 11 -> 22
        });

        final int result = composite.onDamage(10);

        assertEquals(22, result);
        // value seen by each hook proves ordering + threading
        assertEquals(List.of(10, 11), seen);
    }

    @Test
    void removeHookExcludesItFromChain() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        final DamageEventHook plusTen = damage -> damage + 10;
        composite.addHook(plusTen);
        composite.addHook(damage -> damage + 1);

        composite.removeHook(plusTen);

        assertEquals(6, composite.onDamage(5));
    }

    @Test
    void removingUnknownHookIsNoOp() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        composite.addHook(damage -> damage + 3);
        composite.removeHook(damage -> damage); // never added
        assertEquals(8, composite.onDamage(5));
    }

    @Test
    void compositeIsItselfADamageEventHook() {
        final DamageEventHookComposite composite = new DamageEventHookComposite();
        composite.addHook(damage -> damage - 2);
        final DamageEventHook asHook = composite;
        assertTrue(asHook.onDamage(10) == 8);
    }
}
