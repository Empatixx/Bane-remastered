package cz.empatix.ui;

import cz.empatix.render.attributes.Renderable;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicBoolean;

public class UISelector {
    private final List<SelectableUI> elements;
    private SelectableUI selectedElement;
    public UISelector() {
        elements = new ArrayList<>();
    }

    public void addElement(final SelectableUI element) {
        elements.add(element);
    }

    public void update(final double dt, final float mouseX, final float mouseY) {
        elements.forEach(element -> element.update(dt, mouseX, mouseY));
    }

    public void mousePressed(final int button, final float mouseX, final float mouseY) {
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

    public void mouseReleased(final int button, final float mouseX, final float mouseY) {
        elements.forEach(element -> element.mouseReleased(button, mouseX, mouseY));
    }

    public void keyPressed(final int key) {
        elements.forEach(element -> element.keyPressed(key));
    }

    public void keyReleased(final int key) {
        elements.forEach(element -> element.keyReleased(key));
    }

    public void mouseScrolled(final double xoffset, final double yoffset) {
        elements.forEach(element -> element.mouseScrolled(xoffset, yoffset));
    }

    public List<Renderable> toRender(final double dt) {
        return elements.stream()
                .filter(UIElement::canRender)
                .map(element -> (Renderable) element)
                .toList();
    }

    public void addElements(List<SelectableUI> elements) {
        this.elements.addAll(elements);
    }
}
