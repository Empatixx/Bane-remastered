package cz.empatix.items.domain;

import cz.empatix.artefacts.domain.Artefact;
import cz.empatix.assets.service.AssetsService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.interaction.service.InteractionTracker;
import cz.empatix.items.service.FloatingEffectService;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.domain.Vertex;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.OutlineBinding;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4f;
import org.lwjgl.glfw.GLFW;

import java.util.Set;
import java.util.function.Consumer;

public class ArtefactDrop implements ItemDrop {
    private static final int SCALE = 2;
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final CommonInteractable interactable;
    private final Vector2f position = new Vector2f();
    private final TextureAtlas.Entry texture;
    private boolean pickedUp = false;
    private final DropSource source;
    private final FloatingEffectService floatingEffectService;

    public ArtefactDrop(final DropSource source,
                        final Artefact artefact,
                        final Consumer<Artefact> addArtefact) {
        texture = AssetsService.getTexture(artefact.getTexturePath());
        interactable = new CommonInteractable(pressedKeys -> {
            if (pressedKeys.contains(GLFW.GLFW_KEY_E)) {
                addArtefact.accept(artefact);
                pickedUp = true;
            }
        }, keys -> keys.contains(GLFW.GLFW_KEY_E), () -> pickedUp, position, getWidth(), getHeight());
        this.source = source;
        floatingEffectService = new FloatingEffectService();
    }

    @Override
    public void interact(final Set<Integer> keys) {
        interactable.interact(keys);
    }

    @Override
    public boolean canInteract(final Set<Integer> keys) {
        interactionTracker.interaction();
        return interactable.canInteract(keys);
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public void update(float dt) {
        floatingEffectService.update(dt);
    }

    @Override
    public boolean canRemove() {
        return interactable.canRemove();
    }

    @Override
    public boolean isPickedUp() {
        return pickedUp;
    }

    @Override
    public void setPosition(float x, float y) {
        position.set(x, y);
    }

    @Override
    public void setDespawnable(boolean despawn) {

    }

    @Override
    public int getWidth() {
        return texture.width() * SCALE;
    }

    @Override
    public int getHeight() {
        return texture.height() * SCALE;
    }

    @Override
    public void destroy() {
        InteractionService.getInstance().removeInteractable(this);
    }

    @Override
    public RenderData renderData() {
        final Vector2f floatingPosition = new Vector2f(position).add(0, floatingEffectService.getOffsetY());
        return new RenderData(
                Vertex.builder()
                        .zDepth(getZIndex())
                        .position(floatingPosition)
                        .posDimensions(getWidth(), getHeight())
                        .texture(texture)
                        .offset(getWidth(), getHeight())
                        .buildQuad(),
                texture.textureId(),
                new OutlineBinding(Camera.getInstance().getProjectionMatrix(), new Vector2f(getWidth(), getHeight()),
                        texture.normalizedStartPositions(), texture.normalizedSpriteEndPosition(1, 1, 1, 1), new Vector4f(1f, 1f, 0f, interactionTracker.isInteracting() ? 1f : 0f))
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 3;
    }

    @Override
    public boolean canRender() {
        return Camera.getInstance().isInProjection(position, getWidth(), getHeight());
    }

    @Override
    public DropSource getSource() {
        return source;
    }
}
