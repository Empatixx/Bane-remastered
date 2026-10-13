package cz.empatix.cursors.service;

import cz.empatix.cursors.domain.*;
import cz.empatix.render.attributes.Renderable;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.EnumMap;
import java.util.Map;

public class CursorService {
    private final static Logger LOGGER = LoggerFactory.getLogger(CursorService.class);
    private final static Map<CursorType, Cursor> cursors;
    private static Cursor currentCursor;
    private static Vector2f lastMousePosition = new Vector2f();

    static {
        cursors = new EnumMap<>(CursorType.class);
        cursors.put(CursorType.DEFAULT, new DefaultCursor());
        cursors.put(CursorType.DISABLED, new DisabledCursor());
        cursors.put(CursorType.CROSSHAIR, new AimCursor());
        cursors.put(CursorType.RELOAD, new ReloadCursor());

        currentCursor = cursors.get(CursorType.DISABLED);
    }

    public static void setCursor(final CursorType cursorType) {
        if(currentCursor.getType() == cursorType) {
            return;
        }
        LOGGER.info("Setting cursor to {}", cursorType);
        currentCursor = cursors.get(cursorType);
        currentCursor.updateMouse(lastMousePosition.x(), lastMousePosition.y(), 0);
    }
    public static CursorType getType(){
        return currentCursor.getType();
    }
    public static Renderable getCursor() {
        return currentCursor;
    }

    public static void updateMouse(final float x, final float y, final float dt) {
        lastMousePosition.set(x, y);
        currentCursor.updateMouse(x, y, dt);
    }

}
