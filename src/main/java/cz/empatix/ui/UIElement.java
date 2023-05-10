package cz.empatix.ui;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.RenderScene;
import cz.empatix.render.domain.RenderDataType;

public interface UIElement extends Drawable {
    void update(double dt, double mouseX, double mouseY);
    void mousePressed(int button, double mouseX, double mouseY);
    void mouseReleased(int button, double mouseX, double mouseY);
    void keyPressed(int key);
    void keyReleased(int key);
    void mouseScrolled(double xoffset, double yoffset);
    boolean isHovered();

    @Override
    default RenderScene renderScene() {
        return RenderScene.HUD;
    }
    @Override
    default RenderDataType renderType() {
        return RenderDataType.DYNAMIC;
    }

}
