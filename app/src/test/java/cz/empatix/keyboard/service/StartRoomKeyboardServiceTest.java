package cz.empatix.keyboard.service;

import cz.empatix.keyboard.domain.KeyboardImage;
import cz.empatix.keyboard.domain.KeyboardImageWithText;
import cz.empatix.keyboard.domain.KeyboardType;
import cz.empatix.rooms.service.StartRoom;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

/**
 * Headless coverage for the keyboard-hint widgets shown in the start room. {@link KeyboardImage} and the
 * {@link cz.empatix.hud.service.text.AlternativeViewText} labels fetch textures/fonts in their
 * constructors, so the test runs under a mocked {@code AssetsService} via {@link HeadlessAssets}; the
 * {@link StartRoom} is mocked to supply fixed geometry. Only construction, the per-frame {@code update}
 * tick, and the pure {@link KeyboardImage} geometry getters are exercised — the {@code toRender()} path
 * is deliberately avoided because it walks the (mocked, GL-bound) font drawables.
 */
@ExtendWith(HeadlessAssets.class)
class StartRoomKeyboardServiceTest {

    private static StartRoom startRoomAt(final int x, final int y, final int width) {
        final StartRoom room = mock(StartRoom.class);
        when(room.getX()).thenReturn(x);
        when(room.getY()).thenReturn(y);
        when(room.getWidth()).thenReturn(width);
        return room;
    }

    @Test
    void constructsForAStartRoomWithoutTouchingGl() {
        final StartRoom room = startRoomAt(0, 0, 800);

        assertDoesNotThrow(() -> new StartRoomKeyboardService(room));
    }

    @Test
    void updateTicksAllWidgetsWithoutThrowing() {
        final StartRoomKeyboardService service = new StartRoomKeyboardService(startRoomAt(100, 200, 640));

        assertDoesNotThrow(() -> service.update(0.016));
    }

    // ---- KeyboardImage pure geometry (the building block the service positions) ------------------

    @Test
    void keyboardImageReportsFixedScaledDimensions() {
        final KeyboardImage key = new KeyboardImage(KeyboardType.W);

        // WIDTH = 11 * SCALE(3) = 33 ; HEIGHT = 12 * SCALE(3) = 36
        assertEquals(33, key.getWidth());
        assertEquals(36, key.getHeight());
    }

    @Test
    void keyboardImagePositionIsZeroUntilSet() {
        final KeyboardImage key = new KeyboardImage(KeyboardType.A);
        assertEquals(0f, key.getPosition().x());
        assertEquals(0f, key.getPosition().y());

        key.setPosition(120, 250);
        assertEquals(120f, key.getPosition().x());
        assertEquals(250f, key.getPosition().y());
    }

    @Test
    void keyboardImageZIndexIsTwo() {
        assertEquals(2, new KeyboardImage(KeyboardType.E).getZIndex());
    }

    // ---- KeyboardImageWithText delegation --------------------------------------------------------

    @Test
    void keyboardImageWithTextDelegatesZIndexToImageAndKeepsParts() {
        final KeyboardImage image = new KeyboardImage(KeyboardType.F);
        final cz.empatix.hud.service.text.AlternativeViewText text =
                new cz.empatix.hud.service.text.AlternativeViewText(
                        "Use",
                        "PixelFJVerdana12pt.ttf",
                        new org.joml.Vector2f(),
                        2,
                        cz.empatix.assets.domain.font.FontSize.FONTSIZE_22,
                        cz.empatix.render.domain.RenderScene.SCENE);

        final KeyboardImageWithText pair = KeyboardImageWithText.of(image, text);

        assertSame(image, pair.image());
        assertSame(text, pair.text());
        assertEquals(image.getZIndex(), pair.getZIndex());
    }

    @Test
    void keyboardImageWithTextSetPositionMovesUnderlyingImage() {
        final KeyboardImage image = new KeyboardImage(KeyboardType.Q);
        final cz.empatix.hud.service.text.AlternativeViewText text =
                new cz.empatix.hud.service.text.AlternativeViewText(
                        "Drop",
                        "PixelFJVerdana12pt.ttf",
                        new org.joml.Vector2f(),
                        2,
                        cz.empatix.assets.domain.font.FontSize.FONTSIZE_22,
                        cz.empatix.render.domain.RenderScene.SCENE);
        final KeyboardImageWithText pair = KeyboardImageWithText.of(image, text);

        pair.setPosition(40, 70);

        // image sits at the requested coordinates; the label is offset (+50, +30)
        assertEquals(40f, image.getPosition().x());
        assertEquals(70f, image.getPosition().y());
        assertEquals(90f, text.getPosition().x());
        assertEquals(100f, text.getPosition().y());
    }
}
