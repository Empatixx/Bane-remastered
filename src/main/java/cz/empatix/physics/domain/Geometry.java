package cz.empatix.physics.domain;

public interface Geometry {
    boolean isIntersecting(final Geometry other);
    int getMass();
}
