package cz.empatix.rooms.service;

import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;

import java.util.List;

public interface Room {
    void enterEvent(final Player p);
    void leaveEvent(final Player p);
    void init();
    void update(double dt);
    void fixedUpdate(double dt);
    List<Renderable> toRender();
    int getWidth();
    int getHeight();
    int getX();
    int getY();
    default int getMinX(){
        return getX();
    }
    default int getMinY(){
        return getY();
    }
    default int getMaxX(){
        return getX() + getWidth();
    }
    default int getMaxY(){
        return getY() + getHeight();
    }
    default boolean isInRoom(final float x, final float y){
        final int xStart = getX();
        final int yStart = getY();

        final int width = getWidth();
        final int height = getHeight();

        return x >= xStart && y >= yStart &&
                xStart + width >= x && yStart + height >= y;
    }

}
