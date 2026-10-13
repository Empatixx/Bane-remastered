package cz.empatix.player;

import org.junit.jupiter.api.Test;

import java.util.ArrayList;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

class MoveForceEventHookCompositeTest {

    @Test
    void emptyCompositeReturnsInputUnchanged() {
        final MoveForceEventHookComposite composite = new MoveForceEventHookComposite();
        assertEquals(1234L, composite.onMoveForce(1234L));
    }

    @Test
    void singleHookTransformsValue() {
        final MoveForceEventHookComposite composite = new MoveForceEventHookComposite();
        composite.addHook(force -> force + 100L);
        assertEquals(1100L, composite.onMoveForce(1000L));
    }

    @Test
    void hooksInvokedInOrderAndValueThreadedThrough() {
        final MoveForceEventHookComposite composite = new MoveForceEventHookComposite();
        final List<Long> seen = new ArrayList<>();

        composite.addHook(force -> {
            seen.add(force);
            return force + 1L; // 10 -> 11
        });
        composite.addHook(force -> {
            seen.add(force);
            return force * 3L; // 11 -> 33
        });

        final long result = composite.onMoveForce(10L);

        assertEquals(33L, result);
        assertEquals(List.of(10L, 11L), seen);
    }

    @Test
    void removeHookExcludesItFromChain() {
        final MoveForceEventHookComposite composite = new MoveForceEventHookComposite();
        final MoveForceEventHook doubler = force -> force * 2L;
        composite.addHook(doubler);
        composite.addHook(force -> force + 1L);

        composite.removeHook(doubler);

        assertEquals(6L, composite.onMoveForce(5L));
    }

    @Test
    void removingUnknownHookIsNoOp() {
        final MoveForceEventHookComposite composite = new MoveForceEventHookComposite();
        composite.addHook(force -> force + 7L);
        composite.removeHook(force -> force);
        assertEquals(12L, composite.onMoveForce(5L));
    }
}
