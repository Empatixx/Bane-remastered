package cz.empatix.roomobjects.resolvers;

import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.function.Function;
import java.util.function.Predicate;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

/**
 * Pure-logic coverage for {@link PositionProcessorImpl}: a value holder that delegates pre/post
 * processing to its functions and exposes its conditions. The convenience constructor must default
 * both transforms to identity.
 */
class PositionProcessorImplTest {

    @Test
    void exposesTheSuppliedConditions() {
        final List<Predicate<Vector2f>> conditions = List.of(v -> true, v -> false);

        final PositionProcessorImpl processor = new PositionProcessorImpl(conditions);

        assertSame(conditions, processor.conditions());
    }

    @Test
    void shortConstructorUsesIdentityTransforms() {
        final PositionProcessorImpl processor = new PositionProcessorImpl(List.of());
        final Vector2f position = new Vector2f(7, 11);

        assertSame(position, processor.preprocess(position));
        assertSame(position, processor.postprocess(position));
    }

    @Test
    void preprocessAndPostprocessDelegateToTheGivenFunctions() {
        final Function<Vector2f, Vector2f> pre = v -> new Vector2f(v.x + 1, v.y + 2);
        final Function<Vector2f, Vector2f> post = v -> new Vector2f(v.x * 10, v.y * 10);
        final PositionProcessorImpl processor = new PositionProcessorImpl(List.of(), pre, post);

        final Vector2f pre_ = processor.preprocess(new Vector2f(0, 0));
        final Vector2f post_ = processor.postprocess(new Vector2f(3, 4));

        assertEquals(new Vector2f(1, 2), pre_);
        assertEquals(new Vector2f(30, 40), post_);
    }
}
