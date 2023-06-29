package cz.empatix.cursors.service;

import cz.empatix.cursors.domain.*;
import cz.empatix.render.attributes.Renderable;

import java.util.EnumMap;
import java.util.Map;

public class CursorService {
    private final static Map<CursorType, Cursor> cursors;
    private static Cursor currentCursor;

    static {
        cursors = new EnumMap<>(CursorType.class);
        cursors.put(CursorType.DEFAULT, new DefaultCursor());
        cursors.put(CursorType.DISABLED, new DefaultCursor());
        cursors.put(CursorType.CROSSHAIR, new AimCursor());
        cursors.put(CursorType.RELOAD, new ReloadCursor());

        currentCursor = cursors.get(CursorType.DISABLED);
    }

    public static void setCursor(CursorType cursorType) {
        currentCursor = cursors.get(cursorType);
    }

    public static Renderable getCursor() {
        return currentCursor;
    }

    public static void updateMouse(final float x, final float y, final float dt) {
        currentCursor.updateMouse(x, y, dt);
    }

}
