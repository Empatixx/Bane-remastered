package cz.empatix.death.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

class VictoryServiceTest {

    @Test
    void freshServiceIsNotAchieved() {
        assertFalse(new VictoryService().isAchieved());
    }

    @Test
    void achieveFlipsTheFlag() {
        final VictoryService victory = new VictoryService();

        victory.achieve();

        assertTrue(victory.isAchieved());
    }

    @Test
    void isFinalFloorHoldsAtOrPastTheFinalFloor() {
        final VictoryService victory = new VictoryService();

        assertFalse(victory.isFinalFloor(VictoryService.FINAL_FLOOR - 1));
        assertTrue(victory.isFinalFloor(VictoryService.FINAL_FLOOR));
        assertTrue(victory.isFinalFloor(VictoryService.FINAL_FLOOR + 1));
    }
}
