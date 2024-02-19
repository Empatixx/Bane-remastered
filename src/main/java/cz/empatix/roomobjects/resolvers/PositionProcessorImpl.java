package cz.empatix.roomobjects.resolvers;

import org.joml.Vector2f;

import java.util.List;
import java.util.function.Function;
import java.util.function.Predicate;

public class PositionProcessorImpl implements PositionProcessor {
    private final List<Predicate<Vector2f>> conditions;
    private final Function<Vector2f, Vector2f> preprocessor;
    private final Function<Vector2f, Vector2f> postprocessor;
    public PositionProcessorImpl(final List<Predicate<Vector2f>> conditions,
                                 final Function<Vector2f, Vector2f> preprocessor,
                                 final Function<Vector2f, Vector2f> postprocessor) {
        this.conditions = conditions;
        this.preprocessor = preprocessor;
        this.postprocessor = postprocessor;
    }
    public PositionProcessorImpl(final List<Predicate<Vector2f>> conditions) {
        this.conditions = conditions;
        this.preprocessor = Function.identity();
        this.postprocessor = Function.identity();
    }
    @Override
    public List<Predicate<Vector2f>> conditions() {
        return conditions;
    }

    @Override
    public Vector2f preprocess(Vector2f position) {
        return preprocessor.apply(position);
    }

    @Override
    public Vector2f postprocess(Vector2f position) {
        return postprocessor.apply(position);
    }
}
