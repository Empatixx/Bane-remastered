package cz.empatix.minimap.service;

import cz.empatix.map.domain.MapGenerationResult;
import cz.empatix.minimap.domain.MinimapRoom;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.rooms.service.Room;

import java.util.List;

public interface MinimapRenderer {
    void apply(final MapGenerationResult result);
    void visit(final Room room);
    void visitAll();
    void update(final float dt, final float mouseX, final float mouseY);

    List<Renderable> toRender();

    void mouseReleased(final int button, final float mouseX, final float mouseY);

    void mousePressed(final int button, final float mouseX, final float mouseY);

    void keyPressed(int key);

    void keyReleased(int key);
}
