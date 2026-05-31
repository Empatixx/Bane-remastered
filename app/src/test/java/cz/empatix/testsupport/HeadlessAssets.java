package cz.empatix.testsupport;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.init.ALInit;
import cz.empatix.newphysics.service.PhysicsService;
import org.junit.jupiter.api.extension.AfterEachCallback;
import org.junit.jupiter.api.extension.BeforeEachCallback;
import org.junit.jupiter.api.extension.ExtensionContext;
import org.mockito.MockedStatic;

import static org.mockito.Mockito.mockStatic;

/**
 * One annotation that makes a test headless instead of repeating the same scaffolding in every class:
 * <pre>{@code @ExtendWith(HeadlessAssets.class)}</pre>
 *
 * <p>Per test it: disables OpenAL, spins up a REAL (headless) dyn4j {@link PhysicsService} so
 * PhysicsObjectFactory builds real bodies, and stubs the static {@link AssetsService} accessors to
 * return GL-free fakes (see {@link MockedAssets}). Tests then just {@code new SomeClass(...)} directly.
 *
 * <p>Tests that need to add their own asset stubs can still open {@code mockStatic} manually instead.
 */
public final class HeadlessAssets implements BeforeEachCallback, AfterEachCallback {

    private MockedStatic<AssetsService> assets;

    @Override
    public void beforeEach(final ExtensionContext context) {
        ALInit.canUseAL = false;
        new PhysicsService(); // headless dyn4j world; registers itself as the static instance
        assets = mockStatic(AssetsService.class);
        MockedAssets.stub(assets);
    }

    @Override
    public void afterEach(final ExtensionContext context) {
        if (assets != null) {
            assets.close();
            assets = null;
        }
    }
}
