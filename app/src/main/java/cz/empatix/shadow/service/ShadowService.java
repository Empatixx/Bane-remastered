package cz.empatix.shadow.service;

import cz.empatix.lightning.domain.LightSource;
import cz.empatix.lightning.service.LightningService;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.shadow.domain.PlayerTextureShadow;
import cz.empatix.utils.InterpolatedAngle;
import cz.empatix.utils.InterpolatedFloat;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.ArrayList;
import java.util.List;

public class ShadowService {
    private static final float MAX_SHADOW_DISTANCE = 400f;
    private static final float INTERPOLATION_SPEED = 8f;

    private final Player player;
    private final LightningService lightningService;
    private final InterpolatedAngle shadowAngle;
    private final InterpolatedFloat shadowIntensity;
    private final InterpolatedFloat shadowLength;

    private LightSource currentLight;
    private int cachedTextureId;

    public ShadowService(final Player player) {
        this.player = player;
        this.lightningService = LightningService.getInstance();
        this.shadowAngle = new InterpolatedAngle(0f, INTERPOLATION_SPEED);
        this.shadowIntensity = new InterpolatedFloat(0f, INTERPOLATION_SPEED);
        this.shadowLength = new InterpolatedFloat(20f, INTERPOLATION_SPEED);
    }

    public void update() {
        // Empty - computation moved to toRender() for correct timing
    }

    public List<Renderable> toRender(final float dt) {
        updateShadowTargets(dt);

        if (shadowIntensity.get() < 0.01f) {
            return List.of();
        }

        final PlayerTextureShadow shadow = createShadow();
        if (shadow == null) {
            return List.of();
        }
        return List.of(shadow);
    }

    private void updateShadowTargets(final float dt) {
        final Vector2f playerPos = player.getRenderPosition();
        final int playerWidth = player.getWidth();
        final int playerHeight = player.getHeight();
        final LightSource playerLight = player.getLightSource();
        final float playerCenterX = playerPos.x() + playerWidth / 2f;
        final float playerCenterY = playerPos.y() + playerHeight / 2f;

        // Find the strongest light
        float strongestStrength = 0f;
        LightSource strongestLight = null;
        float strongestDist = 0f;

        for (final LightSource light : lightningService.lightsOnCamera()) {
            if (!light.canDisplay() || light == playerLight) {
                continue;
            }
            final Vector2f lightPos = light.getPosition();
            final float dist = distance(playerCenterX, playerCenterY, lightPos.x(), lightPos.y());
            if (dist > MAX_SHADOW_DISTANCE) {
                continue;
            }
            final float strength = light.getIntensity() / (dist * dist * 0.0001f + 1f);
            if (strength > strongestStrength) {
                strongestStrength = strength;
                strongestLight = light;
                strongestDist = dist;
            }
        }

        if (strongestLight != null) {
            currentLight = strongestLight;
            final Vector2f lightPos = strongestLight.getPosition();

            // Calculate target angle
            final float dirX = playerCenterX - lightPos.x();
            final float dirY = playerCenterY - lightPos.y();
            final float len = (float) Math.sqrt(dirX * dirX + dirY * dirY);
            if (len > 0.001f) {
                final float nx = dirX / len;
                final float ny = dirY / len;
                final float targetAngle = (float) Math.atan2(ny, nx) - (float) Math.PI / 2f;
                shadowAngle.setTarget(targetAngle);
            }

            // Calculate target intensity
            final float distanceFactor = 1f - (strongestDist / MAX_SHADOW_DISTANCE) * (strongestDist / MAX_SHADOW_DISTANCE);
            final float lightFactor = strongestLight.getIntensity() * 0.3f;
            final float targetIntensity = Math.max(0.03f, Math.min(0.15f, distanceFactor * lightFactor));
            shadowIntensity.setTarget(targetIntensity);

            // Calculate target shadow length
            final float targetLength = Math.max(10f, Math.min(35f, 50f - strongestDist / 10f));
            shadowLength.setTarget(targetLength);

            // Cache texture ID
            final RenderData playerData = player.renderData();
            cachedTextureId = playerData.textureId();
        } else {
            // No light - fade out shadow
            shadowIntensity.setTarget(0f);
        }

        // Update interpolated values
        shadowAngle.update(dt);
        shadowIntensity.update(dt);
        shadowLength.update(dt);
    }

    private PlayerTextureShadow createShadow() {
        final RenderData playerData = player.renderData();
        final List<Vertex> playerVertices = playerData.vertices();

        if (playerVertices.size() != 4) {
            return null;
        }

        final Vector2f playerPos = player.getRenderPosition();
        final float feetX = playerPos.x() + player.getWidth() / 2f;
        final float feetY = playerPos.y() + player.getHeight();
        final float height = (float) player.getHeight();

        // Use interpolated values
        final float angle = shadowAngle.get();
        final float cos = (float) Math.cos(angle);
        final float sin = (float) Math.sin(angle);
        final float length = shadowLength.get();

        final List<Vertex> relativeVertices = new ArrayList<>();
        for (final Vertex v : playerVertices) {
            final float relX = v.position().x() - feetX;
            final float relY = v.position().y() - feetY;

            final float mirroredY = -relY;
            final float heightRatio = mirroredY / height;
            final float baseY = mirroredY + length * heightRatio * 0.5f;

            final float rotatedX = relX * cos - baseY * sin;
            final float rotatedY = relX * sin + baseY * cos;

            relativeVertices.add(new Vertex(
                    new Vector3f(rotatedX, rotatedY, 3f),
                    v.texCoords()
            ));
        }

        return new PlayerTextureShadow(player, relativeVertices, cachedTextureId, shadowIntensity.get());
    }

    private float distance(final float x1, final float y1, final float x2, final float y2) {
        final float dx = x2 - x1;
        final float dy = y2 - y1;
        return (float) Math.sqrt(dx * dx + dy * dy);
    }
}
