package cz.empatix.hud.service;

import cz.empatix.hud.domain.StatusBarType;
import cz.empatix.hud.domain.StatusController;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.StatusBarBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Matrix4f;
import org.joml.Vector2f;
import org.joml.Vector3f;

import java.util.List;
import java.util.function.Supplier;

public class StatusBar implements HUDElement {
    private final Vector2f position;
    private final Supplier<Integer> currentValueSupplier;
    private final Supplier<Integer> maxValueSupplier;
    private final StatusController statusController;
    private final StatusBarType statusBarType;

    public StatusBar(final Supplier<Integer> currentValueSupplier,
                     final Supplier<Integer> maxValueSupplier,
                     final StatusBarType statusBarType,
                     final Vector2f position) {
        this.currentValueSupplier = currentValueSupplier;
        this.maxValueSupplier = maxValueSupplier;
        this.statusController = new StatusController();
        this.statusBarType = statusBarType;
        this.position = position;
    }

    @Override
    public void update(double dt) {
        statusController.update(dt, currentValueSupplier.get(), maxValueSupplier.get());
    }

    @Override
    public RenderData renderData() {
        final float fillAmount = statusController.delta();
        final float normXOffset = statusBarType.getNormalizedXOffset();
        final float normYOffset = statusBarType.getNormalizedYOffset();
        final float normWidth = statusBarType.getNormalizedWidth();
        final float normHeight = statusBarType.getNormalizedHeight();
        final Vector3f color = new Vector3f();
        final Matrix4f projection = Camera.getOriginWithoutZoomProjection();
        return new RenderData(vertices(),
                statusBarType.getTexture().textureId(),
                new StatusBarBinding(projection, color, fillAmount, normXOffset, normYOffset, normWidth, normHeight),
                RenderData.RenderDataType.DYNAMIC);
    }

    private List<Vertex> vertices() {
        final TextureAtlas.Entry texture = statusBarType.getTexture();
        return Vertex.builder()
                .position(position)
                .posDimensions(texture.width(), texture.height())
                .texture(texture)
                .scale(statusBarType.getScale())
                .buildQuad();
    }

    @Override
    public boolean canRender() {
        return true;
    }

    @Override
    public int getZIndex() {
        return 1;
    }
}
