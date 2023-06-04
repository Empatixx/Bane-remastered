package cz.empatix.physics.domain;

import org.joml.Vector2f;

public record CollisionResult (
        Vector2f normal,
        float depth
){
}
