package cz.empatix.guns.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Pure-logic coverage of {@link GunDamageEventComposite}: it chains registered hooks, feeding each
 * hook's output into the next, and supports add/remove of hooks. No GL/assets involved.
 */
class GunDamageEventCompositeTest {

    @Test
    void applyWithNoHooksReturnsDamageUnchanged() {
        final GunDamageEventComposite composite = new GunDamageEventComposite();
        assertEquals(10, composite.apply(10));
    }

    @Test
    void applyChainsHooksLeftToRight() {
        final GunDamageEventComposite composite = new GunDamageEventComposite();
        final GunDamageEventHook plusFive = mock(GunDamageEventHook.class);
        when(plusFive.apply(10)).thenReturn(15);
        final GunDamageEventHook doubler = mock(GunDamageEventHook.class);
        when(doubler.apply(15)).thenReturn(30);

        composite.addHook(plusFive);
        composite.addHook(doubler);

        // 10 -> plusFive -> 15 -> doubler -> 30
        assertEquals(30, composite.apply(10));
    }

    @Test
    void removedHookNoLongerContributes() {
        final GunDamageEventComposite composite = new GunDamageEventComposite();
        final GunDamageEventHook hook = mock(GunDamageEventHook.class);
        when(hook.apply(10)).thenReturn(99);

        composite.addHook(hook);
        composite.removeHook(hook);

        assertEquals(10, composite.apply(10));
    }
}
