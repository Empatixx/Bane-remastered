package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.vitality.VitalityChange;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.lang.reflect.Field;
import java.time.Instant;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for {@link VitalityDisplayService}: it accumulates floating +/- popups for HP
 * changes and expires them after a fixed TTL. Popup text fetches a GL-free mock font (see
 * {@link HeadlessAssets}). Expiry is forced by back-dating the change's timestamp via reflection
 * rather than sleeping; the GL-only {@code collectSubRenderables} path is not driven.
 */
@ExtendWith(HeadlessAssets.class)
class VitalityDisplayServiceTest {

    private static final long TTL_MILLIS = 500L;

    @SuppressWarnings("unchecked")
    private static List<VitalityChange> changes(final VitalityDisplayService service) throws Exception {
        final Field field = VitalityDisplayService.class.getDeclaredField("changes");
        field.setAccessible(true);
        return (List<VitalityChange>) field.get(service);
    }

    /** Replaces every queued change's timestamp with one well beyond the TTL window. */
    private static void backdateAll(final VitalityDisplayService service) throws Exception {
        final List<VitalityChange> list = changes(service);
        for (int i = 0; i < list.size(); i++) {
            final VitalityChange old = list.get(i);
            list.set(i, new VitalityChange(old.text(), Instant.now().minusMillis(TTL_MILLIS + 5_000)));
        }
    }

    @Test
    void startsEmpty() {
        assertTrue(new VitalityDisplayService().isEmpty());
    }

    @Test
    void zeroChangeIsIgnored() {
        final VitalityDisplayService service = new VitalityDisplayService();

        service.addChange(0, new Vector2f(0, 0), 32, 32);

        assertTrue(service.isEmpty(), "a zero delta produces no popup");
    }

    @Test
    void positiveChangeAddsAGreenPopup() throws Exception {
        final VitalityDisplayService service = new VitalityDisplayService();

        service.addChange(7, new Vector2f(100, 100), 32, 32);

        assertFalse(service.isEmpty());
        assertEquals(1, changes(service).size());
    }

    @Test
    void negativeChangeAddsARedPopup() throws Exception {
        final VitalityDisplayService service = new VitalityDisplayService();

        service.addChange(-4, new Vector2f(100, 100), 32, 32);

        assertFalse(service.isEmpty());
        assertEquals(1, changes(service).size());
    }

    @Test
    void multipleChangesAccumulate() throws Exception {
        final VitalityDisplayService service = new VitalityDisplayService();

        service.addChange(1, new Vector2f(0, 0), 32, 32);
        service.addChange(-2, new Vector2f(0, 0), 32, 32);
        service.addChange(3, new Vector2f(0, 0), 32, 32);

        assertEquals(3, changes(service).size());
    }

    @Test
    void updateKeepsFreshPopups() {
        final VitalityDisplayService service = new VitalityDisplayService();
        service.addChange(5, new Vector2f(0, 0), 32, 32);

        service.update(0.016);

        assertFalse(service.isEmpty(), "a just-added popup is well within its TTL");
    }

    @Test
    void updateExpiresPopupsPastTheirTtl() throws Exception {
        final VitalityDisplayService service = new VitalityDisplayService();
        service.addChange(5, new Vector2f(0, 0), 32, 32);
        backdateAll(service); // push the creation instant beyond the TTL window

        service.update(0.016);

        assertTrue(service.isEmpty(), "popups older than the TTL are removed");
    }
}
