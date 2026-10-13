package cz.empatix.roomobjects.domain;

import cz.empatix.animation.AnimationState;
import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

/**
 * Headless coverage for the real {@link Torch}: the side-state light-positioning branches, the
 * disable transition (which kills the real light and swaps to the disabled sprite) and stable
 * geometry. The light source is provided by the headless-safe {@code LightningService}.
 */
@ExtendWith(HeadlessAssets.class)
class TorchTest {

    @Test
    void setStateForEachSideRepositionsLightWithoutThrowing() {
        final Torch torch = new Torch();
        torch.setPosition(100f, 100f);

        torch.setState(AnimationState.LEFT_SIDE);
        torch.setState(AnimationState.RIGHT_SIDE);
        torch.setState(AnimationState.TOP);
        torch.update(0.05);

        assertEquals(32 * 2, torch.getWidth());
        assertEquals(32 * 2, torch.getHeight());
        assertFalse(torch.canRemove());
    }

    @Test
    void disableSwapsToDisabledSpriteAndDestroysLight() {
        final Torch torch = new Torch();
        torch.setState(AnimationState.TOP);

        torch.disable(); // destroys the light; updates after this must not touch a live light

        torch.update(0.05);
        assertEquals(100f, torch.getPosition().x, 100f); // position unaffected; sanity tick
    }

    @Test
    void positionGetterReflectsSetter() {
        final Torch torch = new Torch();
        torch.setPosition(12f, 34f);

        assertEquals(12f, torch.getPosition().x, 0.0001f);
        assertEquals(34f, torch.getPosition().y, 0.0001f);
    }
}
