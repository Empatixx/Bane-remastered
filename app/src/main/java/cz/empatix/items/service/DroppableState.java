package cz.empatix.items.service;

import java.util.function.Supplier;

public class DroppableState {
    private final Supplier<Boolean> conditionToDrop;
    private boolean dropped;
    public DroppableState(final Supplier<Boolean> conditionToDrop){
        this.conditionToDrop = conditionToDrop;
        dropped = false;
    }
    public boolean canDrop(){
        final boolean canDrop = conditionToDrop.get();
        try{
            return canDrop && !dropped;
        } finally {
            dropped = canDrop;
        }
    }
}
