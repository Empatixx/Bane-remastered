package cz.empatix.roomobjects.resolvers;

import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.function.Predicate;
import java.util.function.Supplier;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertThrows;

/**
 * Pure-logic coverage for {@link PositionFinder}: it repeatedly samples a supplier until a processor
 * whose conditions all pass is found, then returns that processor's post-processed position. We
 * drive the accept/reject/retry/exhaustion branches with deterministic supplier and predicate stubs
 * (no randomness, no physics).
 */
class PositionFinderTest {

    private final PositionFinder finder = new PositionFinder();

    @Test
    void returnsPostprocessedPositionWhenAllConditionsPass() {
        final Supplier<Vector2f> supplier = () -> new Vector2f(5, 5);
        final PositionProcessor processor = new PositionProcessorImpl(
                List.of(v -> true),
                v -> new Vector2f(v.x + 1, v.y + 1), // preprocess
                v -> new Vector2f(v.x * 2, v.y * 2)); // postprocess

        final Vector2f result = finder.getPosition(supplier, List.of(processor));

        // preprocess (5,5)->(6,6), conditions pass, postprocess ->(12,12)
        assertEquals(new Vector2f(12, 12), result);
    }

    @Test
    void retriesUntilTheSupplierYieldsAnAcceptedPosition() {
        // supplier returns 0,1,2,...; the only accepted x is 3
        final int[] counter = {0};
        final Supplier<Vector2f> supplier = () -> new Vector2f(counter[0]++, 0);
        final Predicate<Vector2f> acceptsThree = v -> v.x == 3f;
        final PositionProcessor processor =
                new PositionProcessorImpl(List.of(acceptsThree));

        final Vector2f result = finder.getPosition(supplier, List.of(processor));

        assertEquals(3f, result.x, 0.0001f);
    }

    @Test
    void fallsThroughToTheNextProcessorWhenTheFirstRejects() {
        final Supplier<Vector2f> supplier = () -> new Vector2f(1, 1);
        final PositionProcessor rejecting =
                new PositionProcessorImpl(List.of(v -> false));
        final Vector2f marker = new Vector2f(99, 99);
        final PositionProcessor accepting =
                new PositionProcessorImpl(List.of(v -> true), v -> v, v -> marker);

        final Vector2f result = finder.getPosition(supplier, List.of(rejecting, accepting));

        assertSame(marker, result, "the second processor handles what the first rejected");
    }

    @Test
    void throwsAfterExhaustingMaxTries() {
        final Supplier<Vector2f> supplier = () -> new Vector2f(0, 0);
        final PositionProcessor neverAccepts =
                new PositionProcessorImpl(List.of(v -> false));

        assertThrows(IllegalStateException.class,
                () -> finder.getPosition(supplier, List.of(neverAccepts), 5));
    }

    @Test
    void requiresEveryConditionOfAProcessorToPass() {
        final int[] counter = {0};
        // x climbs 0,1,2,...; accepted only when x >= 2 AND x is even -> x == 2
        final Supplier<Vector2f> supplier = () -> new Vector2f(counter[0]++, 0);
        final PositionProcessor processor = new PositionProcessorImpl(
                List.of(v -> v.x >= 2f, v -> v.x % 2f == 0f));

        final Vector2f result = finder.getPosition(supplier, List.of(processor));

        assertEquals(2f, result.x, 0.0001f);
    }
}
