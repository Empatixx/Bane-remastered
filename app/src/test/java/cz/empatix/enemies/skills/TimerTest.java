package cz.empatix.enemies.skills;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class TimerTest {

    @Test
    void freshTimerIsFinishedSinceLastTimeDefaultsToZero() {
        // lastTime starts at 0 (epoch), so elapsed = now - 0 >> totalTime.
        final Timer timer = new Timer(1000L);
        assertTrue(timer.isFinished(),
                "Never-started timer treats elapsed time as the full epoch and is finished");
    }

    @Test
    void justStartedTimerWithLongDurationIsNotFinished() {
        final Timer timer = new Timer(60_000L);
        timer.start();
        assertFalse(timer.isFinished(),
                "A 60s timer started just now should not yet be finished");
    }

    @Test
    void zeroDurationTimerIsImmediatelyFinishedAfterStart() throws InterruptedException {
        final Timer timer = new Timer(0L);
        timer.start();
        // Need at least 1ms to elapse for strict > comparison.
        Thread.sleep(2L);
        assertTrue(timer.isFinished(),
                "A zero-duration timer should report finished once any time elapses");
    }

    @Test
    void restartingResetsTheElapsedClock() {
        final Timer timer = new Timer(60_000L);
        // Even though a never-started timer reports finished, starting it resets the clock.
        timer.start();
        assertFalse(timer.isFinished());
    }
}
