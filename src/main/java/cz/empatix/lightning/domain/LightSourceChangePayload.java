package cz.empatix.lightning.domain;

import cz.empatix.render.service.ObservePayload;

public record LightSourceChangePayload(
        LightSource light,
        int index
) implements ObservePayload {
}
