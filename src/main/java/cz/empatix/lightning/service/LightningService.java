package cz.empatix.lightning.service;

import cz.empatix.lightning.domain.LightSource;
import org.joml.Vector2f;

import java.util.ArrayList;
import java.util.List;

public class LightningService {
    private final List<LightSource> lightSources = new ArrayList<>();
    public static LightningService instance;
    public static LightningService getInstance() {
        if(instance == null){
            instance = new LightningService();
        }
        return instance;
    }
    public LightSource createLightSource(final Vector2f position,
                                  final float radius,
                                  final float intensity){
        final LightSource lightSource = new LightSource(position, radius, intensity);
        lightSources.add(lightSource);
        return lightSource;
    }

    public void update(){
        lightSources.removeIf(LightSource::isDestroyed);
    }
    public List<LightSource> getLightSources() {
        return lightSources;
    }
}
