package cz.empatix.enemies.targetting;

import cz.empatix.newphysics.service.CollisionType;
import cz.empatix.newphysics.service.PhysicsService;
import cz.empatix.player.Player;
import org.joml.Vector2f;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.function.Supplier;
import java.util.stream.Stream;

public class AStarTargetting {
    private final static Logger LOGGER = LoggerFactory.getLogger(AStarTargetting.class);
    private final static int CELL_SIZE = 16;
    private final Queue<PathCell> path = new ArrayDeque<>(15);
    private final Supplier<Vector2f> positionSupplier;
    private Player target;
    private int lastTargetX;
    private int lastTargetY;
    private final int selfWidth;
    private final int selfHeight;

    public AStarTargetting(final Supplier<Vector2f> positionSupplier,
                           final int selfWidth,
                           final int selfHeight) {
        this.positionSupplier = positionSupplier;
        this.selfWidth = selfWidth;
        this.selfHeight = selfHeight;
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
        final Vector2f position = positionSupplier.get();
        final int startX = (int) position.x() + selfWidth / 2;
        final int startY = (int) position.y() + selfHeight / 2;

        final int targetX = (int) target.getX() + target.getWidth() / 2;
        final int targetY = (int) target.getY() + target.getHeight() / 2;

        lastTargetX = (int) target.getX();
        lastTargetY = (int) target.getY();

        LOGGER.info("Starting A* for player {} at {},{} to {},{}", target, startX, startY, targetX, targetY);
        this.path.clear();
        this.path.addAll(aStar(startX, startY, targetX, targetY));
    }

    private List<PathCell> aStar(final int startX, final int startY,
                                 final int targetX, final int targetY) {
        final PriorityQueue<PathCell> openSet = new PriorityQueue<>(100, Comparator.comparingDouble(PathCell::f));
        final Set<PathCell> openSetLookup = new HashSet<>(100);

        final Set<PathCell> closedSet = new HashSet<>(100);

        final PathCell startNode = new PathCell(startX, startY, 0, getDistance(startX, startY, targetX, targetY), null);
        openSet.add(startNode);
        openSetLookup.add(startNode);

        while (!openSet.isEmpty()) {
            if (openSet.size() > 40000){
                LOGGER.error("A* failed to find path");
                return List.of();
            }
            final PathCell currentNode = openSet.peek();
            openSetLookup.remove(currentNode);
            openSet.poll();

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
        return Stream.of(
                        new PathCell(cell.x() + CELL_SIZE, cell.y(), cell.g() + 1, getDistance(cell.x() + CELL_SIZE, cell.y(), targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y(), cell.g() + 1, getDistance(cell.x() - CELL_SIZE, cell.y(), targetX, targetY), cell),
                        new PathCell(cell.x(), cell.y() + CELL_SIZE, cell.g() + 1, getDistance(cell.x(), cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x(), cell.y() - CELL_SIZE, cell.g() + 1, getDistance(cell.x(), cell.y() - CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() + CELL_SIZE, cell.y() + CELL_SIZE, cell.g() + 1.4f, getDistance(cell.x() + CELL_SIZE, cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y() - CELL_SIZE, cell.g() + 1.4f, getDistance(cell.x() - CELL_SIZE, cell.y() - CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y() + CELL_SIZE, cell.g() + 1.4f, getDistance(cell.x() - CELL_SIZE, cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() + CELL_SIZE, cell.y() - CELL_SIZE, cell.g() + 1.4f, getDistance(cell.x() + CELL_SIZE, cell.y() - CELL_SIZE, targetX, targetY), cell)
                )
                .filter(this::isNotBlocked)
                .toList();
    }

    private boolean isNotBlocked(PathCell pathCell) {
        final PhysicsService service = PhysicsService.getInstance();
        return !service.query(new Vector2f(pathCell.x(), pathCell.y()), new Vector2f(CELL_SIZE, CELL_SIZE),
                EnumSet.of(CollisionType.ENEMY, CollisionType.PLAYER, CollisionType.TRAP, CollisionType.ENEMY_BULLET, CollisionType.PLAYER_BULLET,CollisionType.ITEM, CollisionType.ROOM_OBJECT));
    }

    private int getDistance(final int startX, final int startY, final int targetX, final int targetY) {
        return Math.abs(startX - targetX) + Math.abs(startY - targetY);
    }

    public void update(final float dt) {
        if (path.isEmpty()) {
            return;
        }
        final PathCell target = path.peek();
        final Vector2f position = positionSupplier.get();
        final float x = position.x() + selfWidth / 2;
        final float y = position.y() + selfHeight / 2;
        final float targetX = target.x();
        final float targetY = target.y();

        final float dx = targetX - x;
        final float dy = targetY - y;

        final float distance = (float) Math.sqrt(dx * dx + dy * dy);
        if (distance <= CELL_SIZE) {
            path.poll();
        }
    }

    public Vector2f getDirectionTarget() {
        final Vector2f position = positionSupplier.get();
        if (path.isEmpty()) {
            /*final Vector2f direction = new Vector2f(target.getX() + target.getWidth() / 2f - position.x() - selfWidth / 2f, target.getY() + target.getHeight() / 2f - position.y() - selfHeight / 2f);
            if (direction.length() > 0) {
                direction.normalize();
            }*/
            return new Vector2f(0, 0);
        }
        final PathCell target = path.peek();
        final float x = position.x() + selfWidth / 2;
        final float y = position.y() + selfHeight / 2;
        final float targetX = target.x();
        final float targetY = target.y();

        final Vector2f direction = new Vector2f(targetX - x, targetY - y);
        if (direction.length() > 0) {
            direction.normalize();
        }
        return direction;
    }

    public boolean isTargetLeftFromEnemy() {
        final Vector2f position = positionSupplier.get();
        return target.getX() + target.getWidth() / 2f < position.x() + selfWidth / 2f;
    }
}
