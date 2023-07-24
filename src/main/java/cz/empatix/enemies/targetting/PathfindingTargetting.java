package cz.empatix.enemies.targetting;

import cz.empatix.player.Player;

public class PathfindingTargetting {
    private final static int CELL_SIZE = 16;
    private Player target;
    public void target(final Player player){
        this.target = player;
    }
    public void updatePathfinding(final float dt){

    }
}
