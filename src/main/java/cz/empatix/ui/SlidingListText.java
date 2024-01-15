package cz.empatix.ui;

import cz.empatix.hud.service.HUDElement;
import cz.empatix.hud.service.text.CommonText;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.RenderData;
import org.joml.Vector2f;
import org.joml.Vector3i;

import java.util.List;
import java.util.function.Consumer;
import java.util.function.Function;

public class SlidingListText<E> implements HUDElement {
    private final static Consumer EMPTY_CONSUMER = e -> {};
    private final SlidingList<E> slidingList;
    private final CommonText commonText;
    private final Function<E, String> textFunction;
    private Consumer<E> onChange = EMPTY_CONSUMER;
    private float maxX;
    private float minX;
    public SlidingListText(final SlidingList<E> slidingList,
                           final Function<E, String> textFunction) {
        this.slidingList = slidingList;
        this.commonText = new CommonText(textFunction.apply(slidingList.getSelected()), "PixelFJVerdana12pt.ttf", new Vector2f(), 1);
        this.textFunction = textFunction;
    }
    public void center(final float xMin, final float xMax, final float y) {
        this.minX = xMin;
        this.maxX = xMax;
        commonText.setPosition(xMin, y);
        commonText.setCenterX(xMin, xMax);
    }
    @Override
    public void update(double dt) {
        commonText.setText(textFunction.apply(slidingList.getSelected()));
    }

    @Override
    public RenderData renderData() {
        throw new IllegalStateException("Not implemented");
    }

    @Override
    public int getZIndex() {
        throw new IllegalStateException("Not implemented");
    }

    @Override
    public boolean canRender() {
        throw new IllegalStateException("Not implemented");
    }

    @Override
    public List<Renderable> getSubRenderables() {
        return commonText.getSubRenderables();
    }

    public void setColorRGB(final Vector3i textColor) {
        commonText.setColorRGB(textColor);
    }

    public void next() {
        slidingList.next();
        commonText.setText(textFunction.apply(slidingList.getSelected()));
        commonText.setCenterX(minX, maxX);
        onChange.accept(slidingList.getSelected());
    }
    public void previous() {
        slidingList.previous();
        commonText.setText(textFunction.apply(slidingList.getSelected()));
        commonText.setCenterX(minX, maxX);
        onChange.accept(slidingList.getSelected());
    }

    public void onChange(Consumer<E> onChange) {
        this.onChange = onChange;
    }

    public void slideToValue(E value) {
        slidingList.slideToValue(value);
        commonText.setText(textFunction.apply(slidingList.getSelected()));
        commonText.setCenterX(minX, maxX);
        onChange.accept(slidingList.getSelected());
    }
}
