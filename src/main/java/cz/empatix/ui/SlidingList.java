package cz.empatix.ui;

import cz.empatix.hud.service.text.CommonText;

import java.util.List;

public class SlidingList<E> {
    private List<E> elements;
    private int selectedIndex;
    public SlidingList(final List<E> elements) {
        this.elements = elements;
    }
    public void next() {
        selectedIndex = (selectedIndex + 1) % elements.size();
    }
    public void previous() {
        selectedIndex = (selectedIndex - 1 + elements.size()) % elements.size();
    }

    public E getSelected() {
        return elements.get(selectedIndex);
    }
}
