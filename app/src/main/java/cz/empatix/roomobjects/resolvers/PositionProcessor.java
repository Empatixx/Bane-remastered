package cz.empatix.roomobjects.resolvers;

import org.joml.Vector2f;

import java.util.List;
import java.util.function.Predicate;

public interface PositionProcessor {
    List<Predicate<Vector2f>> conditions();
    Vector2f preprocess(Vector2f position);
    Vector2f postprocess(Vector2f position);
}
