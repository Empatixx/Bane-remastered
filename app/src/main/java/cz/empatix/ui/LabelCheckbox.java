package cz.empatix.ui;

import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.domain.RenderData;
import cz.empatix.render.domain.RenderableState;
import org.joml.Vector2f;
import org.joml.Vector2i;
import org.joml.Vector3i;

public class LabelCheckbox implements UIElement {
    private final static Vector3i DEFAULT_COLOR = new Vector3i(222, 112, 38);
    private final CommonText label;
    private final Checkbox checkbox;

    public LabelCheckbox(final String label) {
        this.checkbox = new Checkbox();
        this.label = new CommonText(label, "PixelFJVerdana12pt.ttf", new Vector2f(), checkbox.getZIndex());
        this.label.setColorRGB(DEFAULT_COLOR);
    }
    public void setTextPosition(final float x, final float y){
        final Vector2f checkboxPosition = checkbox.getPosition();
        final Vector2i dimensions = checkbox.getDimensions();
        label.setPosition(x, y);
        label.setCenterY(checkboxPosition.y(), checkboxPosition.y() + dimensions.y());
    }

    public void setPosition(final float x, final float y){
        checkbox.setPosition(x, y);

        final Vector2f checkboxPosition = checkbox.getPosition();
        final Vector2i dimensions = checkbox.getDimensions();
        label.setPosition(x, y);
        label.setCenterY(checkboxPosition.y(), checkboxPosition.y() + dimensions.y());
    }

    @Override
    public RenderData renderData() {
        return checkbox.renderData();
    }

    @Override
    public void collectSubRenderables(RenderableState state) {
        label.collectSubRenderables(state);
    }

    @Override
    public int getZIndex() {
        return checkbox.getZIndex();
    }

    @Override
    public boolean canRender() {
        return checkbox.canRender();
    }

    @Override
    public void update(double dt, double mouseX, double mouseY) {
        checkbox.update(dt, mouseX, mouseY);
    }

    @Override
    public void mousePressed(int button, double mouseX, double mouseY) {
        checkbox.mousePressed(button, mouseX, mouseY);
    }

    @Override
    public void mouseReleased(int button, double mouseX, double mouseY) {
        checkbox.mouseReleased(button, mouseX, mouseY);
    }

    @Override
    public void keyPressed(int key) {
        checkbox.keyPressed(key);
    }

    @Override
    public void keyReleased(int key) {
        checkbox.keyReleased(key);
    }

    @Override
    public void mouseScrolled(double xoffset, double yoffset) {
        checkbox.mouseScrolled(xoffset, yoffset);
    }

    @Override
    public boolean isHovered() {
        return checkbox.isHovered();
    }
}
