package cz.empatix.ui;

import cz.empatix.hud.service.text.CommonText;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;

public class SlidingList<E> {
    private final static Logger LOGGER = LoggerFactory.getLogger(SlidingList.class);
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

    public void slideToValue(E value) {
        selectedIndex = elements.indexOf(value);
        if (selectedIndex == -1) {
            LOGGER.error("Value {} not found in list, fallbacking to first value", value);
            selectedIndex = 0;
        }
    }
}
