package cz.empatix.lightning.service;

import cz.empatix.lightning.domain.LightSource;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

public class LightningService {
    private final List<LightSource> lightSources = new ArrayList<>();
    public static LightningService instance;

    public static LightningService getInstance() {
        if (instance == null) {
            instance = new LightningService();
        }
        return instance;
    }

    public LightSource createLightSource(final Vector2f position,
                                         final Vector3f color,
                                         final float radius,
                                         final float intensity) {
        final float r = color.x != 0 ? color.x/255f : 0;
        final float g = color.y != 0 ? color.y/255f : 0;
        final float b = color.z != 0 ? color.z/255f : 0;
        final LightSource lightSource = new LightSource(position, new Vector3f(r,g,b), radius, intensity);
        lightSources.add(lightSource);
        return lightSource;
    }
    public LightSource createLightSource(
                                         final Vector3f color,
                                         final float radius,
                                         final float intensity) {
        return createLightSource(new Vector2f(), color, radius, intensity);
    }

    public void update() {
        lightSources.removeIf(LightSource::isDestroyed);
    }

    public List<LightSource> lightsOnCamera(){
        return lightSources
                .stream()
                .filter(LightSource::isOnCamera)
                .toList();
    }
}
