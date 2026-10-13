package cz.empatix.artefacts.domain;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class ShieldDamageEventHookTest {

    @Test
    void startsActive() {
        assertTrue(new ShieldDamageEventHook().isActive());
    }

    @Test
    void onDamageAbsorbsAllDamageAndDeactivates() {
        final ShieldDamageEventHook hook = new ShieldDamageEventHook();
        // shield absorbs the hit fully -> remaining damage is zero
        assertEquals(0, hook.onDamage(50));
        assertFalse(hook.isActive());
    }

    @Test
    void staysInactiveAfterFurtherDamage() {
        final ShieldDamageEventHook hook = new ShieldDamageEventHook();
        hook.onDamage(10);
        assertEquals(0, hook.onDamage(99));
        assertFalse(hook.isActive());
    }
}
