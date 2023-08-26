package cz.empatix.items.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.audio.domain.SoundEffect;
import cz.empatix.audio.service.AudioService;
import cz.empatix.guns.domain.Gun;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.interaction.service.InteractionTracker;
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

public class WeaponDrop implements ItemDrop {
    private final static AudioService AUDIO_SERVICE = AudioService.getInstance();
    private final InteractionTracker interactionTracker = new InteractionTracker();
    private final CommonInteractable interactable;
    private final Vector2f position = new Vector2f();
    private final TextureAtlas.Entry texture;
    private boolean pickedUp = false;
    public WeaponDrop(final Gun gun, final Consumer<Gun> addGun) {
        texture = AssetsService.getTexture(gun.getTexturePath());
        interactable = new CommonInteractable(pressedKeys -> {
            interactionTracker.interaction();
            if(pressedKeys.contains(GLFW.GLFW_KEY_E)){
                addGun.accept(gun);
                pickedUp = true;
                AUDIO_SERVICE.play(SoundEffect.PICKUP);
            }
        }, () -> true, () -> pickedUp, position, getWidth(), getHeight());
    }
    @Override
    public void interact(final Set<Integer> keys) {
        interactable.interact(keys);
    }

    @Override
    public boolean canInteract() {
        return interactable.canInteract();
    }

    @Override
    public Vector2f getPosition() {
        return position;
    }

    @Override
    public void update(float dt) {

    }

    @Override
    public boolean canRemove() {
        return interactable.canRemove();
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
        return texture.width();
    }

    @Override
    public int getHeight() {
        return texture.height();
    }

    @Override
    public void destroy() {

    }

    @Override
    public RenderData renderData() {
        return new RenderData(
                Vertex.builder()
                        .position(position)
                        .posDimensions(getWidth(), getHeight())
                        .texture(texture)
                        .offset(getWidth(), getHeight())
                        .buildQuad(),
                texture.textureId(),
                new OutlineBinding(Camera.getProjection(), new Vector2f(getWidth(), getHeight()),
                        texture.normalizedStartPositions(), texture.normalizedSpriteEndPosition(1,1,1,1), new Vector4f(1f, 1f, 0f, interactionTracker.isInteracting() ? 1f : 0f))
        );
    }

    @Override
    public RenderScene renderScene() {
        return RenderScene.SCENE;
    }

    @Override
    public int getZIndex() {
        return 2;
    }

    @Override
    public boolean canRender() {
        return true;
    }
}
