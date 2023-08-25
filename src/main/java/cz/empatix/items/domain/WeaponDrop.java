package cz.empatix.items.domain;

import cz.empatix.guns.domain.Gun;
import cz.empatix.hud.service.image.AlternativeViewImage;
import cz.empatix.interaction.domain.CommonInteractable;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderScene;
import cz.empatix.render.service.Camera;
import org.joml.Vector2f;
import org.lwjgl.glfw.GLFW;

import java.util.Set;
import java.util.function.Consumer;

public class WeaponDrop implements ItemDrop {
    private final CommonInteractable interactable;
    private final AlternativeViewImage weapon;
    private boolean pickedUp = false;
    private final Vector2f position = new Vector2f();
    public WeaponDrop(final Gun gun, final Consumer<Gun> addGun) {
        interactable = new CommonInteractable(pressedKeys -> {
            if(pressedKeys.contains(GLFW.GLFW_KEY_E)){
                addGun.accept(gun);
                pickedUp = true;
            }
        }, () -> true, () -> pickedUp, position, 0, 0);
        weapon = new AlternativeViewImage(gun.getTexturePath(), position, 1, 2, RenderScene.SCENE, Camera.getProjection());
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
        weapon.setPosition(x, y);
    }

    @Override
    public void setDespawnable(boolean despawn) {

    }

    @Override
    public int getWidth() {
        return weapon.getWidth();
    }

    @Override
    public int getHeight() {
        return weapon.getHeight();
    }

    @Override
    public void destroy() {

    }

    @Override
    public RenderData renderData() {
        return weapon.renderData();
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
