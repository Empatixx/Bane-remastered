package cz.empatix.enemies.service;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.rooms.service.Room;
import org.joml.Vector2f;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

/**
 * Verifies {@link EnemyBoundsService} clamps enemies back to their last in-bounds coordinate when
 * they cross a room edge. The first pass only records a baseline; the second pass enforces it.
 */
class EnemyBoundsServiceTest {

    private static Room room(final int x, final int y, final int w, final int h) {
        final Room room = mock(Room.class);
        when(room.getX()).thenReturn(x);
        when(room.getY()).thenReturn(y);
        when(room.getWidth()).thenReturn(w);
        when(room.getHeight()).thenReturn(h);
        return room;
    }

    @Test
    void firstPassOnlyRecordsBaselineAndDoesNotReposition() {
        final Room room = room(0, 0, 100, 100);
        final EnemyService enemyService = mock(EnemyService.class);
        final Enemy enemy = mock(Enemy.class);
        when(enemy.getWidth()).thenReturn(10);
        when(enemy.getHeight()).thenReturn(10);
        when(enemy.getPosition()).thenReturn(new Vector2f(50, 50));
        when(enemyService.getEnemies()).thenReturn(List.of(enemy));

        final EnemyBoundsService bounds = new EnemyBoundsService(room, enemyService);
        bounds.fixedUpdate(0.016);

        verify(enemy, never()).setPosition(org.mockito.ArgumentMatchers.anyFloat(), org.mockito.ArgumentMatchers.anyFloat());
    }

    @Test
    void enemyLeavingLeftEdgeIsClampedBackOnXAxis() {
        final Room room = room(0, 0, 100, 100);
        final EnemyService enemyService = mock(EnemyService.class);
        final Enemy enemy = mock(Enemy.class);
        when(enemy.getWidth()).thenReturn(10);
        when(enemy.getHeight()).thenReturn(10);
        // first pass at (50,50) -> recorded baseline, second pass at (-5,50) -> out of bounds on x
        when(enemy.getPosition()).thenReturn(new Vector2f(50, 50), new Vector2f(-5, 50));
        when(enemyService.getEnemies()).thenReturn(List.of(enemy));

        final EnemyBoundsService bounds = new EnemyBoundsService(room, enemyService);
        bounds.fixedUpdate(0.016); // baseline
        bounds.fixedUpdate(0.016); // enforce

        // x reset to previous (50), y kept at current (50)
        verify(enemy).setPosition(eq(50f), eq(50f));
    }

    @Test
    void enemyLeavingBottomEdgeIsClampedBackOnYAxis() {
        final Room room = room(0, 0, 100, 100);
        final EnemyService enemyService = mock(EnemyService.class);
        final Enemy enemy = mock(Enemy.class);
        when(enemy.getWidth()).thenReturn(10);
        when(enemy.getHeight()).thenReturn(10);
        // second pass overshoots the bottom edge: y(95)+height(10) > 100
        when(enemy.getPosition()).thenReturn(new Vector2f(40, 40), new Vector2f(40, 95));
        when(enemyService.getEnemies()).thenReturn(List.of(enemy));

        final EnemyBoundsService bounds = new EnemyBoundsService(room, enemyService);
        bounds.fixedUpdate(0.016);
        bounds.fixedUpdate(0.016);

        // x kept at current (40), y reset to previous (40)
        verify(enemy).setPosition(eq(40f), eq(40f));
    }

    @Test
    void inBoundsMovementIsNotRepositioned() {
        final Room room = room(0, 0, 100, 100);
        final EnemyService enemyService = mock(EnemyService.class);
        final Enemy enemy = mock(Enemy.class);
        when(enemy.getWidth()).thenReturn(10);
        when(enemy.getHeight()).thenReturn(10);
        when(enemy.getPosition()).thenReturn(new Vector2f(20, 20), new Vector2f(30, 30));
        when(enemyService.getEnemies()).thenReturn(List.of(enemy));

        final EnemyBoundsService bounds = new EnemyBoundsService(room, enemyService);
        bounds.fixedUpdate(0.016);
        bounds.fixedUpdate(0.016);

        verify(enemy, never()).setPosition(org.mockito.ArgumentMatchers.anyFloat(), org.mockito.ArgumentMatchers.anyFloat());
    }
}
