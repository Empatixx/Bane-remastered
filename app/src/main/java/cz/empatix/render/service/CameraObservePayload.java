package cz.empatix.render.service;

public record CameraObservePayload(
        CameraChangeType change
) implements ObservePayload{
}
