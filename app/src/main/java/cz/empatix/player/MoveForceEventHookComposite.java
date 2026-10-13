package cz.empatix.player;

import java.util.ArrayList;
import java.util.List;

public class MoveForceEventHookComposite implements MoveForceEventHook{
    private final List<MoveForceEventHook> hooks = new ArrayList<>();
    public void addHook(final MoveForceEventHook hook) {
        hooks.add(hook);
    }

    public void removeHook(MoveForceEventHook hook) {
        hooks.remove(hook);
    }

    @Override
    public long onMoveForce(long moveForce) {
        for (MoveForceEventHook hook : hooks) {
            moveForce = hook.onMoveForce(moveForce);
        }
        return moveForce;
    }
}

