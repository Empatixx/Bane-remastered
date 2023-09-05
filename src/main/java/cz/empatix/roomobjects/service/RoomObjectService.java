package cz.empatix.roomobjects.service;

import cz.empatix.render.attributes.Drawable;
import cz.empatix.roomobjects.domain.RoomObject;

import java.util.ArrayList;
import java.util.List;

public class RoomObjectService {
    private final List<RoomObject> roomObjects;
    public RoomObjectService(){
        roomObjects = new ArrayList<>();
    }
    public void add(final RoomObject roomObject){
        roomObjects.add(roomObject);
    }
    public List<Drawable> toRender(){
        return List.copyOf(roomObjects);
    }
    public void update(final double dt){
        roomObjects.forEach(roomObject -> roomObject.update(dt));
        roomObjects.removeIf(RoomObject::canRemove);
    }

    public void cleanUp() {
        // remove physics bodies when will be used.
    }
}
