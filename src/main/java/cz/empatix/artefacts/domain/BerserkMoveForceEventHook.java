package cz.empatix.artefacts.domain;

import cz.empatix.player.MoveForceEventHook;

public class BerserkMoveForceEventHook implements MoveForceEventHook {
    private static final float INCREASE = 1.3f;
    @Override
    public long onMoveForce(long moveForce) {
        return (long) (moveForce * INCREASE);
    }
}
