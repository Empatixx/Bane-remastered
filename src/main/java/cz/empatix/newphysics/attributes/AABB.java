package cz.empatix.newphysics.attributes;

public interface AABB extends Collisionable{
    float xMin();
    float yMin();
    float xMax();
    float yMax();
}
