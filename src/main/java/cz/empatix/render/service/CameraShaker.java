package cz.empatix.render.service;

import org.joml.Vector2f;

import java.util.concurrent.ThreadLocalRandom;

public class CameraShaker {
    private final static float DEFAULT_TWEEN = 0.5f;
    private float shakeCoefficient;
    private final float tween;
    public CameraShaker(){
        shakeCoefficient = 0;
        tween = DEFAULT_TWEEN;
    }
    public CameraShaker(final float tween){
        shakeCoefficient = 0;
        this.tween = tween;
    }
    public void update(final float dt){
        shakeCoefficient -= shakeCoefficient * tween * dt;
        if(shakeCoefficient < 0){
            shakeCoefficient = 0;
        }
        if(shakeCoefficient > 1){
            final Vector2f position = Camera.getPosition();
            final float shakeX = ThreadLocalRandom.current().nextFloat() * shakeCoefficient * 2 - shakeCoefficient;
            final float shakeY = ThreadLocalRandom.current().nextFloat() * shakeCoefficient * 2 - shakeCoefficient;
            position.add(shakeX, shakeY);
            Camera.setSmoothlyPosition(position.x(), position.y(), dt);
        }
    }
    public void shake(final float shakeCoefficient){
        this.shakeCoefficient += shakeCoefficient;
    }
}
