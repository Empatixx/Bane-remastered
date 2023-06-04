package cz.empatix.physics.service;

public record AABB (
        float xMin,
        float xMax,
        float yMin,
        float yMax
){
    public boolean intersects(final AABB other){
        return xMin < other.xMax &&
                xMax > other.xMin &&
                yMin < other.yMax &&
                yMax > other.yMin;
    }
}
