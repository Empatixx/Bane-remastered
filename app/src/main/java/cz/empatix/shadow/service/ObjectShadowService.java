package cz.empatix.shadow.service;

import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.shadow.domain.ObjectShadow;
import cz.empatix.shadow.domain.ShadowCaster;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

/**
 * Service for computing shadows for generic objects (enemies, pets).
 * Each object gets its own shadow angle computed from its position relative to the light.
 */
public class ObjectShadowService {
    private static final float MAX_SHADOW_DISTANCE = 400f;
    private static final float MAX_SHADOW_DISTANCE_SQ = MAX_SHADOW_DISTANCE * MAX_SHADOW_DISTANCE;
    /** A light closer than this (squared) sits on top of the caster: direction is undefined, so skip it. */
    private static final float MIN_SHADOW_DISTANCE_SQ = 0.001f * 0.001f;
    private static final float SHADOW_LENGTH = 15f;
    /** Quadratic distance falloff applied when ranking lights by apparent strength. */
    private static final float DISTANCE_FALLOFF = 0.0001f;
    /** Caster intensity is scaled by light intensity and this factor, then clamped to the band below. */
    private static final float INTENSITY_SCALE = 0.2f;
    private static final float MIN_SHADOW_INTENSITY = 0.02f;
    private static final float MAX_SHADOW_INTENSITY = 0.10f;

    private final LightningService lightningService;

    public ObjectShadowService() {
        this.lightningService = LightningService.getInstance();
    }

    /**
     * Compute shadows for a list of shadow casters.
     * Each caster gets its own shadow angle based on its position relative to the strongest light.
     */
    public List<Renderable> computeShadows(final List<? extends ShadowCaster> casters, final float dt) {
        if (casters.isEmpty()) {
            return List.of();
        }

        // Find the globally strongest light (using first visible caster as reference)
        final LightSource strongestLight = findStrongestLight(casters);
        if (strongestLight == null) {
            return List.of();
        }

        final List<Renderable> shadows = new ArrayList<>();
        for (final ShadowCaster caster : casters) {
            if (!caster.canRender()) {
                continue;
            }

            final ObjectShadow shadow = createShadow(caster, strongestLight);
            if (shadow != null) {
                shadows.add(shadow);
            }
        }

        return shadows;
    }

    /**
     * Find the strongest light visible to any of the casters.
     */
    private LightSource findStrongestLight(final List<? extends ShadowCaster> casters) {
        float strongestStrength = 0f;
        LightSource strongestLight = null;

        for (final LightSource light : lightningService.lightsOnCamera()) {
            if (!light.canDisplay()) {
                continue;
            }

            // Check strength relative to each visible caster
            for (final ShadowCaster caster : casters) {
                if (!caster.canRender()) {
                    continue;
                }

                final Vector2f pos = caster.getShadowPosition();
                final float centerX = pos.x() + caster.getShadowWidth() / 2f;
                final float centerY = pos.y() + caster.getShadowHeight() / 2f;
                final Vector2f lightPos = light.getPosition();
                final float distSq = distanceSquared(centerX, centerY, lightPos.x(), lightPos.y());

                if (distSq > MAX_SHADOW_DISTANCE_SQ) {
                    continue;
                }

                // strength only needs dist^2, so rank lights without a square root in this O(lights*casters) loop
                final float strength = light.getIntensity() / (distSq * DISTANCE_FALLOFF + 1f);
                if (strength > strongestStrength) {
                    strongestStrength = strength;
                    strongestLight = light;
                }
            }
        }

        return strongestLight;
    }

    /**
     * Create shadow for a single caster with angle computed from its position relative to the light.
     */
    private ObjectShadow createShadow(final ShadowCaster caster, final LightSource light) {
        final RenderData casterData = caster.renderData();
        final List<Vertex> casterVertices = casterData.vertices();

        if (casterVertices.size() != 4) {
            return null;
        }

        final Vector2f pos = caster.getShadowPosition();
        final float feetX = pos.x() + caster.getShadowWidth() / 2f;
        final float feetY = pos.y() + caster.getShadowHeight() + caster.getShadowFeetOffsetY();
        final float height = (float) caster.getShadowHeight();

        // Compute angle for THIS caster based on its position relative to the light
        final float centerX = pos.x() + caster.getShadowWidth() / 2f;
        final float centerY = pos.y() + caster.getShadowHeight() / 2f;
        final Vector2f lightPos = light.getPosition();

        final float dirX = centerX - lightPos.x();
        final float dirY = centerY - lightPos.y();
        final float distSq = dirX * dirX + dirY * dirY;

        // Light on top of the caster (undefined direction) or out of range: no shadow.
        if (distSq < MIN_SHADOW_DISTANCE_SQ || distSq > MAX_SHADOW_DISTANCE_SQ) {
            return null;
        }

        // atan2 is scale-invariant, so feed it the raw direction (no normalization / sqrt needed).
        final float angle = (float) Math.atan2(dirY, dirX) - (float) Math.PI / 2f;

        final float distanceFactor = 1f - distSq / MAX_SHADOW_DISTANCE_SQ;
        final float intensity = Math.max(MIN_SHADOW_INTENSITY,
                Math.min(MAX_SHADOW_INTENSITY, distanceFactor * light.getIntensity() * INTENSITY_SCALE));

        // Compute rotated shadow vertices
        final float cos = (float) Math.cos(angle);
        final float sin = (float) Math.sin(angle);

        final List<Vertex> relativeVertices = new ArrayList<>();
        for (final Vertex v : casterVertices) {
            final float relX = v.position().x() - feetX;
            final float relY = v.position().y() - feetY;

            final float mirroredY = -relY;
            final float heightRatio = mirroredY / height;
            final float baseY = mirroredY + SHADOW_LENGTH * heightRatio * 0.5f;

            final float rotatedX = relX * cos - baseY * sin;
            final float rotatedY = relX * sin + baseY * cos;

            relativeVertices.add(new Vertex(
                    new Vector3f(rotatedX, rotatedY, 2f),
                    v.texCoords()
            ));
        }

        return new ObjectShadow(caster, relativeVertices, casterData.textureId(), intensity);
    }

    private float distanceSquared(final float x1, final float y1, final float x2, final float y2) {
        final float dx = x2 - x1;
        final float dy = y2 - y1;
        return dx * dx + dy * dy;
    }
}
