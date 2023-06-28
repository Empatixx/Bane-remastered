package cz.empatix.cursors.service;

import cz.empatix.cursors.domain.AimCursor;
import cz.empatix.cursors.domain.Cursor;
import cz.empatix.cursors.domain.CursorType;
import cz.empatix.cursors.domain.DefaultCursor;
import cz.empatix.render.attributes.Renderable;

import java.util.Map;

public class CursorService {
    private final static Map<CursorType, Cursor> cursors;
    private static Cursor currentCursor;
    static {
        cursors = Map.of(
                CursorType.DEFAULT, new DefaultCursor(),
                CursorType.DISABLED, new DefaultCursor(),
                CursorType.CROSSHAIR, new AimCursor()
        );
        currentCursor = cursors.get(CursorType.DISABLED);
    }
    public static void setCursor(CursorType cursorType){
        currentCursor = cursors.get(cursorType);
    }
    public static Renderable getCursor(){
        return currentCursor;
    }
    public static void updateMouse(final float x, final float y){
        currentCursor.updateMouse(x, y);
    }

}
