package cz.empatix.minimap.service;

import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.service.CursorService;
import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class MinimapService {
    private static final Logger LOGGER = LoggerFactory.getLogger(MinimapService.class);
    private final List<MinimapRenderer> renderers;
    public MinimapService() {
        final SmallMinimapRenderer minimapRenderer = new SmallMinimapRenderer();
        final BigMinimapRenderer bigMinimapRenderer = new BigMinimapRenderer();

        renderers = List.of(minimapRenderer, bigMinimapRenderer);
    }
    public void apply(final MapGenerationResult mapGenerationResult){
        renderers.forEach(minimapRenderer -> minimapRenderer.apply(mapGenerationResult));
        visit(mapGenerationResult.startRoom());
    }

    public void visit(final Room room) {
        renderers.forEach(minimapRenderer -> minimapRenderer.visit(room));
    }
    public void visitAll() {
        renderers.forEach(MinimapRenderer::visitAll);
    }
    public void keyPressed(final int key) {
        switch (key){
            case GLFW.GLFW_KEY_TAB -> {
                final boolean isActive = ((BigMinimapRenderer) renderers.get(1)).isActive();
                if (isActive) {
                    renderers.get(1).deactivate();
                } else {
                    renderers.get(1).activate();
                }
            }
            case GLFW.GLFW_KEY_ESCAPE -> renderers.get(1).deactivate();
        }
    }
    public List<Renderable> toRender(final double dt) {
        return renderers.stream()
                .map(MinimapRenderer::toRender)
                .flatMap(Collection::stream)
                .toList();
    }

    public void update(final float dt,
                       final float mouseX,
                       final float mouseY) {
        renderers.forEach(minimapRenderer -> minimapRenderer.update(dt, mouseX, mouseY));
    }
    public void mouseReleased(final int button, final float mouseX, final float mouseY) {
        renderers.forEach(minimapRenderer -> minimapRenderer.mouseReleased(button, mouseX, mouseY));
    }
    public void mousePressed(final int button, final float mouseX, final float mouseY) {
        renderers.forEach(minimapRenderer -> minimapRenderer.mousePressed(button, mouseX, mouseY));
    }
    public boolean isActiveBigMinimap() {
        final BigMinimapRenderer bigMinimapRenderer = (BigMinimapRenderer) renderers.get(1);
        return bigMinimapRenderer.isActive();
    }

    public void deactivateBigMinimap() {
        final BigMinimapRenderer bigMinimapRenderer = (BigMinimapRenderer) renderers.get(1);
        bigMinimapRenderer.deactivate();
    }
}
