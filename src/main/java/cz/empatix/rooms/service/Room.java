package cz.empatix.rooms.service;

import cz.empatix.render.attributes.Drawable;

import java.util.List;

public interface Room {
    void enterEvent();
    void leaveEvent();
    void init();
    void update();
    List<Drawable> toRender();
    int getWidth();
    int getHeight();
}
