package cz.empatix.death.service;

public class VictoryService {
    public static final int FINAL_FLOOR = 5;
    private boolean achieved;

    public boolean isFinalFloor(final int floor) {
        return floor >= FINAL_FLOOR;
    }

    public void achieve() {
        achieved = true;
    }

    public boolean isAchieved() {
        return achieved;
    }
}
