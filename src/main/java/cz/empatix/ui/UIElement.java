package cz.empatix.ui;

import cz.empatix.render.attributes.Drawable;

public interface UIElement extends Drawable {
    void update(double dt, double mouseX, double mouseY);
    void mousePressed(int button, double mouseX, double mouseY);
    void mouseReleased(int button, double mouseX, double mouseY);
    void keyPressed(int key);
    void keyReleased(int key);
    void mouseScrolled(double xoffset, double yoffset);
    boolean isHovered();
}
