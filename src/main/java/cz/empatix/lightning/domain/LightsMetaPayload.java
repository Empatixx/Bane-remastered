package cz.empatix.lightning.domain;

import cz.empatix.render.service.ObservePayload;
import cz.empatix.render.service.ObserveSubject;

public record LightsMetaPayload(
        int numberLights
) implements ObservePayload {
}
