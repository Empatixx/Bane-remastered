package cz.empatix.ui;

import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

public class UISelector implements UIElement{
    private final List<SelectableUI> elements;
    private SelectableUI selectedElement;
    public UISelector() {
        elements = new ArrayList<>();
    }

    public void addElement(final SelectableUI element) {
        elements.add(element);
    }

    public void mouseReleased(final int button, final float mouseX, final float mouseY) {
        elements.forEach(element -> element.mouseReleased(button, mouseX, mouseY));
    }

    @Override
    public void update(double dt, double mouseX, double mouseY) {
        elements.forEach(element -> element.update(dt, mouseX, mouseY));
    }

    @Override
    public void mousePressed(int button, double mouseX, double mouseY) {
        final AtomicBoolean selected = new AtomicBoolean(false);
        elements.forEach(element -> {
            if (element == selectedElement) {
                return;
            }
            if (selected.get()) {
                return;
            }
            element.mousePressed(button, mouseX, mouseY);
            if (element.isHovered()) {
                selectedElement = element;
                selected.set(true);
            }
        });
        elements.stream()
                .filter(element -> element != selectedElement)
                .forEach(SelectableUI::unselect);
    }

    @Override
    public void mouseReleased(int button, double mouseX, double mouseY) {
        elements.forEach(element -> element.mouseReleased(button, mouseX, mouseY));
    }

    @Override
    public void keyPressed(final int key) {
        elements.forEach(element -> element.keyPressed(key));
    }

    @Override
    public void keyReleased(final int key) {
        elements.forEach(element -> element.keyReleased(key));
    }

    @Override
    public void mouseScrolled(final double xoffset, final double yoffset) {
        elements.forEach(element -> element.mouseScrolled(xoffset, yoffset));
    }

    @Override
    public boolean isHovered() {
        return false;
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return List.copyOf(elements);
    }

    public void addElements(List<SelectableUI> elements) {
        this.elements.addAll(elements);
    }

    @Override
    public RenderData renderData() {
        throw new IllegalStateException("Should not be called, call subRenderables() instead");
    }

    @Override
    public int getZIndex() {
        throw new IllegalStateException("Should not be called, call subRenderables() instead");
    }

    @Override
    public boolean canRender() {
        throw new IllegalStateException("Should not be called, call subRenderables() instead");
    }
}
