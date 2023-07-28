package cz.empatix.enemies.targetting;

import cz.empatix.player.Player;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class AStarTargetting {
    private final static Logger LOGGER = LoggerFactory.getLogger(AStarTargetting.class);
    private final static int CELL_SIZE = 16;
    private final Queue<PathCell> path = new ArrayDeque<>(15);
    private final Vector2f position;
    private Player target;
    private int lastTargetX;
    private int lastTargetY;

    public AStarTargetting(final Vector2f position) {
        this.position = position;
    }

    public void target(final Player player) {
        this.target = player;
    }

    public void updatePath() {
        if (target == null) {
            return;
        }
        if (Math.abs(target.getX() - lastTargetX) <= CELL_SIZE && Math.abs(target.getY() - lastTargetY) <= CELL_SIZE) {
            return;
        }
        final int startX = (int) position.x();
        final int startY = (int) position.y();

        final int targetX = (int) target.getX();
        final int targetY = (int) target.getY();

        lastTargetX = targetX;
        lastTargetY = targetY;

        LOGGER.info("Starting A* for player {} at {},{} to {},{}", target, startX, startY, targetX, targetY);
        this.path.clear();
        this.path.addAll(aStar(startX, startY, targetX, targetY));
    }

    private List<PathCell> aStar(final int startX, final int startY,
                                 final int targetX, final int targetY) {
        final PriorityQueue<PathCell> openSet = new PriorityQueue<>(100, Comparator.comparingInt(PathCell::f));
        final Set<PathCell> openSetLookup = new HashSet<>(100);

        final Set<PathCell> closedSet = new HashSet<>(100);

        final PathCell startNode = new PathCell(startX, startY, 0, getDistance(startX, startY, targetX, targetY), null);
        openSet.add(startNode);
        openSetLookup.add(startNode);

        while (!openSet.isEmpty()) {
            final PathCell currentNode = openSet.peek();
            openSetLookup.remove(currentNode);

            if (Math.abs(currentNode.x() - targetX) <= CELL_SIZE && Math.abs(currentNode.y() - targetY) <= CELL_SIZE) {
                PathCell endNode = currentNode;
                final List<PathCell> path = new ArrayList<>();
                while (endNode != null) {
                    path.add(endNode);
                    endNode = endNode.parent();
                }
                Collections.reverse(path);
                return path;
            }
            closedSet.add(currentNode);

            final List<PathCell> cells = createCells(currentNode, targetX, targetY)
                    .stream()
                    .filter(cell -> !closedSet.contains(cell) && !openSetLookup.contains(cell))
                    .toList();
            openSet.addAll(cells);
            openSetLookup.addAll(cells);
        }
        return List.of();
    }
    private List<PathCell> createCells(final PathCell cell,
                                       final int targetX, final int targetY) {
        return List.of(
                new PathCell(cell.x() + CELL_SIZE, cell.y(), cell.g() + 1, getDistance(cell.x() + CELL_SIZE, cell.y(), targetX, targetY), cell),
                new PathCell(cell.x() - CELL_SIZE, cell.y(), cell.g() + 1, getDistance(cell.x() - CELL_SIZE, cell.y(), targetX, targetY), cell),
                new PathCell(cell.x(), cell.y() + CELL_SIZE, cell.g() + 1, getDistance(cell.x(), cell.y() + CELL_SIZE, targetX, targetY), cell),
                new PathCell(cell.x(), cell.y() - CELL_SIZE, cell.g() + 1, getDistance(cell.x(), cell.y() - CELL_SIZE, targetX, targetY), cell)
        );
    }

    private int getDistance(final int startX, final int startY, final int targetX, final int targetY) {
        return Math.abs(startX - targetX) + Math.abs(startY - targetY);
    }

    public void update(final float dt) {
        if (path.isEmpty()) {
            return;
        }
        final PathCell target = path.peek();
        final float x = position.x();
        final float y = position.y();
        final float targetX = target.x();
        final float targetY = target.y();

        final float dx = targetX - x;
        final float dy = targetY - y;

        final float distance = (float) Math.sqrt(dx * dx + dy * dy);
        if (distance <= 1) {
            path.poll();
        }
    }
    public Vector2f getDirectionTarget() {
        if (path.isEmpty()){
            return new Vector2f().zero();
        }
        final PathCell target = path.peek();
        final float x = position.x();
        final float y = position.y();
        final float targetX = target.x();
        final float targetY = target.y();
        return new Vector2f(targetX - x, targetY - y).normalize();
    }
}
