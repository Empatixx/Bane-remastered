package cz.empatix.minimap.domain;

import cz.empatix.assets.service.AssetsService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.minimap.service.MinimapService;
import cz.empatix.render.attributes.Drawable;
import cz.empatix.render.domain.*;
import cz.empatix.render.service.Camera;
import cz.empatix.render.shaderbindings.MiniMapBinding;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.rooms.service.Room;
import cz.empatix.textureatlas.domain.TextureAtlas;
import org.joml.Vector2f;
import org.joml.Vector4i;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class MinimapRoom  {
    private final int x;
    private final int y;
    private boolean visited;
    private boolean current;
    private final RoomType roomType;
    private final List<RoomDirection> connections;

    public MinimapRoom(final int x, final int y, final Room room) {
        this.x = x;
        this.y = y;
        this.visited = false;
        this.roomType = room.getRoomType();
        this.connections = List.copyOf(room.getNeighboursMap().keySet());
    }
    public void visit() {
        visited = true;
    }

    public void setCurrent(final boolean current) {
        this.current = current;
    }

    public int getX() {
        return x;
    }

    public int getY() {
        return y;
    }

    public RoomType getRoomType() {
        return roomType;
    }

    public List<RoomDirection> getRoomDirections() {
        return connections;
    }

    /*public void update(final MinimapRoom room, final float dt) {
        transform.update(dt);
        setPositionToCenterRoom(dt, room.getX(), room.getY());
    }*/


    /*public void setPositionToCenterRoom(final float dt, final int x, final int y) {
        final float nextX = MinimapService.MINIMAP_POSITION_X + (this.x - x) * DIMENSIONS * SCALE;
        final float nextY = MinimapService.MINIMAP_POSITION_Y + (this.y - y) * DIMENSIONS * SCALE;

        final float currentX = transform.getPosition().x();
        final float currentY = transform.getPosition().y();

        float tween = TWEEN * dt;
        if(tween > 1){
            tween = 1;
        }

        final float newX = (nextX - currentX) * tween + currentX;
        final float newY = (nextY - currentY) * tween + currentY;

        transform.setInterpolatedPosition(newX, newY);
    }*/

}
