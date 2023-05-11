package cz.empatix.rooms.service;

import cz.empatix.render.attributes.Drawable;

import java.util.List;

public interface Room {
    void enterEvent();
    void leaveEvent();
    void init();
    void update(double dt);
    List<Drawable> toRender();
    int getWidth();
    int getHeight();
    int getX();
    int getY();
    default boolean isInRoom(final float x, final float y){
        final int xStart = getX();
        final int yStart = getY();

        final int width = getWidth();
        final int height = getHeight();

        return x <= xStart && y <= yStart &&
                xStart + width >= x && yStart + height >= y;
    }

}
