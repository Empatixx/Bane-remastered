package cz.empatix.newphysics.attributes;

import org.joml.Vector2f;

import java.util.List;
import java.util.Vector;

public interface SAT extends Collisionable {
    List<Vector2f> getAxes();
    List<Vector2f> getPoints();
}
