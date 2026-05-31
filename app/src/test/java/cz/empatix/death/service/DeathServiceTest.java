package cz.empatix.death.service;

import cz.empatix.testsupport.HeadlessAssets;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.Mockito.atLeastOnce;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Covers the death-screen state machine of {@link DeathService} headlessly. The constructor builds a
 * stack of {@code CommonImage}/{@code CommonText} HUD elements (assets), and {@code update} drives a
 * timed reveal that reads the run statistics and computes the score reward. The actual GL render path
 * ({@code toRender}'s sub-renderable collection) is not exercised — only the logic/state and the
 * statistics wiring are asserted.
 */
@ExtendWith(HeadlessAssets.class)
class DeathServiceTest {

    @Test
    void startsInactiveAndRendersNothing() {
        final DeathService service = new DeathService(mock(StatisticsService.class));

        assertFalse(service.isActive());
        assertTrue(service.toRender().isEmpty());
    }

    @Test
    void inactiveUpdateWhilePlayerAliveDoesNothing() {
        final StatisticsService stats = mock(StatisticsService.class);
        final DeathService service = new DeathService(stats);

        service.update(0.016f, false);

        assertFalse(service.isActive());
        // Score is only computed once the screen is active.
        verify(stats, never()).calculateScoreForFloor();
    }

    @Test
    void playerDeathActivatesTheScreenAndComputesScore() {
        final StatisticsService stats = mock(StatisticsService.class);
        final DeathService service = new DeathService(stats);

        service.update(0.016f, true);

        assertTrue(service.isActive());
        // Every active tick recomputes the four reward components.
        verify(stats, atLeastOnce()).calculateScoreForAccuracy();
        verify(stats, atLeastOnce()).calculateScoreForEnemiesKilled();
        verify(stats, atLeastOnce()).calculateScoreForItemsCollected();
        verify(stats, atLeastOnce()).calculateScoreForFloor();
    }

    @Test
    void afterSkullTransitionTheStatisticLabelsAreFilledFromTheService() {
        final StatisticsService stats = mock(StatisticsService.class);
        when(stats.getAccuracy()).thenReturn(42f);
        when(stats.getEnemiesKilled()).thenReturn(7);
        when(stats.getItemsCollected()).thenReturn(3);
        when(stats.getFloor()).thenReturn(2);

        final DeathService service = new DeathService(stats);

        // Activate, then push the accumulator past the skull transition (2.5s) plus all labels.
        service.update(0.016f, true);
        service.update(10f, true);

        verify(stats, atLeastOnce()).getAccuracy();
        verify(stats, atLeastOnce()).getEnemiesKilled();
        verify(stats, atLeastOnce()).getItemsCollected();
        verify(stats, atLeastOnce()).getFloor();
    }

    @Test
    void keyPressedBeforeRewardsCompleteIsIgnored() {
        final DeathService service = new DeathService(mock(StatisticsService.class));

        // Not active yet -> a key press must not flip into the "continue" state, so the
        // screen remains inactive and rendering stays empty.
        service.keyPressed(0);
        assertTrue(service.toRender().isEmpty());

        // Active but the reward sequence has not finished -> key press is still ignored, the
        // screen stays active (does not transition away).
        service.update(0.016f, true);
        service.keyPressed(0);
        assertTrue(service.isActive());
    }
}
