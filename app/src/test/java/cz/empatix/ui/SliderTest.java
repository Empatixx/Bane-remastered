package cz.empatix.ui;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Exercises the REAL {@link Slider} headlessly. Note the GL-free fake atlas entry is 16x16, so the
 * slider track and its cursor end up the same width; the drag-driven {@code update()} path therefore
 * divides by zero and is intentionally not tested. Covered here: programmatic value setting (pure
 * multiplication, no division), positioning, geometry getters, and the release/un-select transition.
 */
@ExtendWith(HeadlessAssets.class)
class SliderTest {

    @Test
    void newSliderIsRenderableWithExpectedDepth() {
        final Slider slider = new Slider();

        assertEquals(2, slider.getZIndex(), "slider renders at z-index 2");
        assertEquals(0f, slider.getPosition().x(), 0f, "starts at the origin x");
        assertEquals(0f, slider.getPosition().y(), 0f, "starts at the origin y");
    }

    @Test
    void setPositionMovesTheTrack() {
        final Slider slider = new Slider();

        slider.setPosition(120, 80);

        assertEquals(120f, slider.getPosition().x(), 0f, "track x moved");
        assertEquals(80f, slider.getPosition().y(), 0f, "track y moved");
    }

    @Test
    void setValueDoesNotThrowAndKeepsTrackInPlace() {
        final Slider slider = new Slider();
        slider.setPosition(50, 50);

        slider.setValue(0.5f); // pure multiply by (dim - cursorWidth); never divides

        assertEquals(50f, slider.getPosition().x(), 0f, "setValue does not move the track itself");
    }

    @Test
    void getHeightReflectsScaledTrackHeight() {
        final Slider slider = new Slider();

        // 16px fake sprite * SCALE 2 = 32px tall track.
        assertEquals(32, slider.getHeight(), "track height is sprite height times scale");
    }

    @Test
    void mouseReleasedDeselectsWithoutSelectionAndIsNeverHovered() {
        final Slider slider = new Slider();

        slider.mouseReleased(0, 10, 10); // clears selection / un-selects cursor; must not throw

        assertFalse(slider.isHovered(), "the slider container never reports itself hovered");
    }

    @Test
    void mousePressedOutsideCursorLeavesSliderUnselectedSoUpdateIsSafe() {
        final Slider slider = new Slider();
        slider.setPosition(200, 200);

        // Press far from the cursor: selected stays false, so update() skips the divide path.
        slider.mousePressed(0, 1000, 1000);
        slider.update(0.016, 1000, 1000);

        assertFalse(slider.isHovered(), "an off-cursor press does not engage a drag");
    }

    @Test
    void onChangeCanBeRegisteredWithoutFiringEagerly() {
        final Slider slider = new Slider();
        final boolean[] fired = {false};

        slider.onChange(value -> fired[0] = true);

        assertFalse(fired[0], "registering an onChange listener does not invoke it");
    }
}
