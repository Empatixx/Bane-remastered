package cz.empatix.cursors.domain;

public class DisabledCursor extends DefaultCursor {
    @Override
    public boolean canRender() {
        return false;
    }

    @Override
    public void updateMouse(float x, float y, final float dt) {
    }
}
