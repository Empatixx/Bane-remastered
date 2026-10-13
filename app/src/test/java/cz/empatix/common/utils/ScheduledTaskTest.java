package cz.empatix.common.utils;

import org.junit.jupiter.api.Test;

import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;

class ScheduledTaskTest {

    @Test
    void doesNotFireBeforeStart() {
        final AtomicInteger count = new AtomicInteger();
        final ScheduledTask task = new ScheduledTask(1f, count::incrementAndGet);
        for (int i = 0; i < 10; i++) {
            task.update(1f);
        }
        assertEquals(0, count.get());
    }

    @Test
    void firesRepeatedlyEveryInterval() {
        final AtomicInteger count = new AtomicInteger();
        final ScheduledTask task = new ScheduledTask(1f, count::incrementAndGet);
        task.start();

        task.update(0.5f);
        assertEquals(0, count.get());
        task.update(0.5f);
        assertEquals(1, count.get(), "first interval");

        // accumulates again from zero
        task.update(0.5f);
        assertEquals(1, count.get());
        task.update(0.5f);
        assertEquals(2, count.get(), "second interval");
    }

    @Test
    void stopHaltsRecurrence() {
        final AtomicInteger count = new AtomicInteger();
        final ScheduledTask task = new ScheduledTask(1f, count::incrementAndGet);
        task.start();
        task.update(1f);
        assertEquals(1, count.get());
        task.stop();
        task.update(5f);
        assertEquals(1, count.get());
    }

    @Test
    void resetClearsAccumulatedTime() {
        final AtomicInteger count = new AtomicInteger();
        final ScheduledTask task = new ScheduledTask(1f, count::incrementAndGet);
        task.start();
        task.update(0.9f);
        task.reset();
        task.update(0.5f);
        assertEquals(0, count.get(), "time was reset, interval not reached");
        task.update(0.5f);
        assertEquals(1, count.get());
    }
}
