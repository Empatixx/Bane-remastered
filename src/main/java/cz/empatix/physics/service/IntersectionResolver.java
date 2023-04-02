package cz.empatix.physics.service;

import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

//todo: works only for rects, add more shapes?
public class IntersectionResolver {
    private final List<Shape> shapes;

    private record Shape(
            Vector2f pos,
            Vector2f dimensions
    ) {
        private boolean isIntersecting(final Shape other) {
            return this.pos.x <= other.pos.x + other.dimensions.x  &&
                    this.pos.x + this.dimensions.x >= other.pos.x  &&
                    this.pos.y <= other.pos.y + other.dimensions.y &&
                    this.pos.y + this.dimensions.y >= other.pos.y;
        }


    }

    public IntersectionResolver() {
        this.shapes = new ArrayList<>();
    }
    public static IntersectionResolver newInstance(){
        return new IntersectionResolver();
    }

    public IntersectionResolver addShape(final Vector2f pos, final Vector2f size) {
        final Shape shape = new Shape(pos, size);
        this.shapes.add(shape);
        return this;
    }
    public IntersectionResolver addShape(final float x, final float y){
        return this.addShape(new Vector2f(x,y));
    }
    public IntersectionResolver addShape(final Vector2f pos){
        return this.addShape(pos, new Vector2f(1,1));
    }
    public IntersectionResolver addShape(final float x, final float y, final float width, final float height){
        return this.addShape(new Vector2f(x,y), new Vector2f(width,height));
    }

    public boolean resolve() {
        return shapes.stream()
                .anyMatch(shape -> shapes.stream()
                        .anyMatch(other -> shape != other && shape.isIntersecting(other))
                );
    }
}
