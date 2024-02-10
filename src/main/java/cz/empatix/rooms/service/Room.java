package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.domain.RoomType;

import java.util.Collection;
import java.util.List;
import java.util.Map;

public interface Room {
    void enterEvent(final Player p);
    void leaveEvent(final Player p);
    void init();
    void cleanUp();
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

        final int xEnd = getX() + getWidth();
        final int yEnd = getY() + getHeight();

        return x >= xStart && y >= yStart &&
                xEnd >= x && yEnd >= y;
    }

    void addNeighbour(final RoomDirection direction, final Room room);

    default Collection<Room> getNeighbours(){
        return getNeighboursMap().values();
    }
    Map<RoomDirection, Room> getNeighboursMap();
    RoomType getRoomType();
    default void hover(final float mouseX, final float mouseY){
    }
    default void mousePressed(final int button, final float mouseX, final float mouseY){
    }
    default void mouseReleased(final int button, final float mouseX, final float mouseY){
    }
}
