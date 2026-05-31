package cz.empatix.enemies.domain.vitality;

import cz.empatix.assets.domain.font.FontSize;
import cz.empatix.hud.service.text.AlternativeViewText;
import cz.empatix.hud.service.text.MovableAlternativeViewText;
import cz.empatix.testsupport.HeadlessAssets;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import java.time.Instant;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Headless coverage for the {@link VitalityChange} record: it bundles a moving popup text with the
 * instant it was created and delegates {@code update} to that text. The font is a GL-free mock
 * (see {@link HeadlessAssets}); we drive {@code update} (which never touches GL) and the record
 * accessors, but deliberately avoid {@code collectSubRenderables} (font drawables -> GL).
 */
@ExtendWith(HeadlessAssets.class)
class VitalityChangeTest {

    private static MovableAlternativeViewText popup() {
        final AlternativeViewText text =
                new AlternativeViewText("+5", "PixelFJVerdana12pt.ttf", new Vector2f(10, 20), 2, FontSize.FONTSIZE_22);
        return new MovableAlternativeViewText(text, new Vector2f(0, -30f));
    }

    @Test
    void recordExposesItsComponents() {
        final MovableAlternativeViewText popup = popup();
        final Instant now = Instant.now();

        final VitalityChange change = new VitalityChange(popup, now);

        assertSame(popup, change.text());
        assertSame(now, change.timeWhen());
    }

    @Test
    void updateDelegatesToTheTextWithoutThrowing() {
        final VitalityChange change = new VitalityChange(popup(), Instant.now());

        change.update(0.016);

        assertTrue(true, "update propagated to the underlying movable text");
    }

    @Test
    void recordEqualityIsValueBased() {
        final MovableAlternativeViewText popup = popup();
        final Instant now = Instant.now();

        assertEquals(new VitalityChange(popup, now), new VitalityChange(popup, now));
    }
}
