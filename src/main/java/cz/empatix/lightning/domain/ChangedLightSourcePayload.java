package cz.empatix.lightning.domain;

import cz.empatix.render.service.ObservePayload;

public record ChangedLightSourcePayload(
        LightSource light,
        int index
) implements ObservePayload {
}
