package cz.empatix.common.utils;

import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;

class DelayedTaskTest {

    @Test
    void doesNotFireBeforeStart() {
        final AtomicInteger count = new AtomicInteger();
        final DelayedTask task = new DelayedTask(1f, count::incrementAndGet);
        // updates without start are ignored
        for (int i = 0; i < 10; i++) {
            task.update(1f);
        }
        assertEquals(0, count.get());
    }

    @Test
    void firesOnceAfterDelayElapses() {
        final AtomicInteger count = new AtomicInteger();
        final DelayedTask task = new DelayedTask(1f, count::incrementAndGet);
        task.start();

        task.update(0.5f);
        assertEquals(0, count.get(), "not enough time yet");
        task.update(0.5f);
        assertEquals(1, count.get(), "delay reached -> fires");

        // does not fire again once done
        task.update(5f);
        assertEquals(1, count.get());
    }

    @Test
    void firesWhenSingleUpdateExceedsDelay() {
        final AtomicInteger count = new AtomicInteger();
        final DelayedTask task = new DelayedTask(1f, count::incrementAndGet);
        task.start();
        task.update(3f);
        assertEquals(1, count.get());
    }

    @Test
    void startResetsElapsedTime() {
        final AtomicInteger count = new AtomicInteger();
        final DelayedTask task = new DelayedTask(1f, count::incrementAndGet);
        task.start();
        task.update(0.9f);
        // restart wipes accumulated time and the done flag
        task.start();
        task.update(0.5f);
        assertEquals(0, count.get(), "elapsed was reset, should not have fired");
        task.update(0.6f);
        assertEquals(1, count.get());
    }

    @Test
    void stopHaltsProgress() {
        final AtomicInteger count = new AtomicInteger();
        final DelayedTask task = new DelayedTask(1f, count::incrementAndGet);
        task.start();
        task.update(0.5f);
        task.stop();
        task.update(5f);
        assertEquals(0, count.get());
    }
}
