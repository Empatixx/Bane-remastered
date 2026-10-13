package cz.empatix.lightning.service;

import cz.empatix.lightning.domain.LightSourceChangePayload;
import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.domain.LightsMetaPayload;
import cz.empatix.render.service.ObservePayload;
import cz.empatix.render.service.ObserveSubject;
import cz.empatix.render.service.Observer;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

public class LightningService implements ObserveSubject, Observer{
    private final List<LightSource> lightSources = new ArrayList<>();
    private final List<Observer> observers = new ArrayList<>();
    public static final LightningService INSTANCE = new LightningService();

    public static LightningService getInstance() {
        return INSTANCE;
    }

    public LightSource createLightSource(final Vector2f position,
                                         final Vector3f color,
                                         final float radius,
                                         final float intensity) {
        final Vector3f rgb = getNormalizedColor(color.x(), color.y(), color.z());
        final LightSource lightSource = new LightSource(position, rgb, radius, intensity);

        lightSources.add(lightSource);
        lightSource.addObserver(this);

        notifyObservers(new LightSourceChangePayload(lightSource, lightSources.size() - 1));
        notifyObservers(new LightsMetaPayload(lightSources.size()));

        return lightSource;
    }
    private Vector3f getNormalizedColor(final float red, final float green, final float blue){
        final float r = red/255f;
        final float g = green/255f;
        final float b = blue/255f;
        return new Vector3f(r,g,b);
    }

    public LightSource createLightSource(final Vector3f color,
                                         final float radius,
                                         final float intensity) {
        return createLightSource(new Vector2f(), color, radius, intensity);
    }

    public void update() {
        removeDestroyedLights();
    }
    private void removeDestroyedLights(){
        int firstIndexDeleted = -1;
        for(int i = 0; i < lightSources.size(); i++) {
            final LightSource light = lightSources.get(i);
            if (light.isDestroyed()){
                light.removeObserver(this);
                lightSources.remove(i);
                i--;
                if(firstIndexDeleted == -1){
                    firstIndexDeleted = i;
                }
            }
        }
        if (firstIndexDeleted == -1){
            return;
        }
        for (int i = firstIndexDeleted; i < lightSources.size(); i++){
            final LightSource light = lightSources.get(i);
            notifyObservers(new LightSourceChangePayload(light, i));
        }
        notifyObservers(new LightsMetaPayload(lightSources.size()));
    }

    public List<LightSource> lightsOnCamera(){
        return lightSources;
    }

    public void clear() {
        lightSources.clear();
    }

    @Override
    public void addObserver(Observer observer) {
        observers.add(observer);
    }

    @Override
    public void removeObserver(Observer observer) {
        observers.remove(observer);
    }

    @Override
    public void clearObservers() {
        observers.clear();
    }

    @Override
    public void notifyObservers(ObservePayload payload) {
        observers.forEach(observer -> observer.update(this, payload));
    }

    @Override
    public void update(ObserveSubject subject, ObservePayload payload) {
        if (subject instanceof LightSource lightSource){
            final LightSourceChangePayload nextPayload = new LightSourceChangePayload(lightSource, lightSources.indexOf(lightSource));
            observers.forEach(observer -> observer.update(this, nextPayload));
        }
    }
}
