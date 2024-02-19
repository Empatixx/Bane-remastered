package cz.empatix.roomobjects.resolvers;

import org.joml.Vector2f;

import java.util.List;
import java.util.function.Predicate;
import java.util.function.Supplier;

public class PositionFinder {
    public Vector2f getPosition(final Supplier<Vector2f> positionSupplier,
                                final List<PositionProcessor> processors){
        do {
            final Vector2f position = positionSupplier.get();
            A: for(final PositionProcessor processor : processors) {
                final Vector2f preprocessed = processor.preprocess(position);
                final List<Predicate<Vector2f>> conditions = processor.conditions();
                for (final Predicate<Vector2f> condition : conditions) {
                    if (!condition.test(preprocessed)) {
                        continue A;
                    }
                }
                return processor.postprocess(preprocessed);
            }
        } while (true);
    }
}
