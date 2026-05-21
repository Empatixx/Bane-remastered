package cz.empatix.enemies.targetting;

import cz.empatix.collision.CollisionType;
import cz.empatix.newphysics.service.PhysicsService;
import org.joml.Vector2f;
import org.joml.Vector2i;
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
    private Targetable target;
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

    public void target(final Targetable targetable) {
        this.target = targetable;
        this.path.clear();
        lastTargetX = Integer.MAX_VALUE;
        lastTargetY = Integer.MAX_VALUE;
    }


    /**
     * @return true if path was found
     */
    public AStarTargetingState updatePath() {
        if (target == null) {
            return AStarTargetingState.NOT_FOUND;
        }
        if (Math.abs(target.getX() - lastTargetX) <= CELL_SIZE && Math.abs(target.getY() - lastTargetY) <= CELL_SIZE) {
            this.path.clear();
            return AStarTargetingState.CLOSE;
        }

        final Vector2f position = positionSupplier.get();
        final int startX = (int) position.x();
        final int startY = (int) position.y();

        final int targetX = (int) target.getX();
        final int targetY = (int) target.getY();

        lastTargetX = (int) target.getX();
        lastTargetY = (int) target.getY();

        this.path.clear();
        this.path.addAll(aStar(startX, startY, targetX, targetY));
        AStarTargetingState aStarTargetingState = this.path.isEmpty() ? AStarTargetingState.NOT_FOUND : AStarTargetingState.FOUND;
        return aStarTargetingState;
    }

    private List<PathCell> aStar(final int startX, final int startY,
                                 final int targetX, final int targetY) {
        final PriorityQueue<PathCell> openSet = new PriorityQueue<>(500, Comparator.comparingDouble(PathCell::f));
        final Set<PathCell> used = new HashSet<>(1000);

        final PathCell startNode = new PathCell(startX, startY, 0, getDistance(startX, startY, targetX, targetY), null);
        openSet.add(startNode);
        used.add(startNode);

        while (!openSet.isEmpty()) {
            if (used.size() > 25_000) {
                LOGGER.error("A* failed to find path, exhausted max used set size");
                return List.of();
            }
            final PathCell currentNode = openSet.peek();
            openSet.poll();
            if (intersects(currentNode)) {
                PathCell endNode = currentNode;
                final List<PathCell> path = new ArrayList<>();
                while (endNode != null) {
                    path.add(endNode);
                    endNode = endNode.parent();
                }
                Collections.reverse(path);
                return path;
            }
            used.add(currentNode);

            final List<PathCell> cells = createCells(currentNode, used, targetX, targetY);
            openSet.addAll(cells);
            used.addAll(cells);
        }
        return List.of();
    }

    private boolean intersects(final PathCell currentNode) {
        final float xMin = currentNode.x();
        final float xMax = currentNode.x() + selfWidth;
        final float yMin = currentNode.y();
        final float yMax = currentNode.y() + selfHeight;

        final float playerXMin = target.getX();
        final float playerXMax = target.getX() + target.getWidth();
        final float playerYMin = target.getY();
        final float playerYMax = target.getY() + target.getHeight();

        return xMin < playerXMax && xMax > playerXMin && yMin < playerYMax && yMax > playerYMin;
    }

    private List<PathCell> createCells(final PathCell cell,
                                       final Set<PathCell> used,
                                       final int targetX, final int targetY) {
        return Stream.of(
                        new PathCell(cell.x() + CELL_SIZE, cell.y(), cell.g() + 10, getDistance(cell.x() + CELL_SIZE, cell.y(), targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y(), cell.g() + 10, getDistance(cell.x() - CELL_SIZE, cell.y(), targetX, targetY), cell),
                        new PathCell(cell.x(), cell.y() + CELL_SIZE, cell.g() + 10, getDistance(cell.x(), cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x(), cell.y() - CELL_SIZE, cell.g() + 10, getDistance(cell.x(), cell.y() - CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() + CELL_SIZE, cell.y() + CELL_SIZE, cell.g() + 14f, getDistance(cell.x() + CELL_SIZE, cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y() - CELL_SIZE, cell.g() + 14f, getDistance(cell.x() - CELL_SIZE, cell.y() - CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() - CELL_SIZE, cell.y() + CELL_SIZE, cell.g() + 14f, getDistance(cell.x() - CELL_SIZE, cell.y() + CELL_SIZE, targetX, targetY), cell),
                        new PathCell(cell.x() + CELL_SIZE, cell.y() - CELL_SIZE, cell.g() + 14f, getDistance(cell.x() + CELL_SIZE, cell.y() - CELL_SIZE, targetX, targetY), cell)
                )
                .filter(newCell -> !used.contains(newCell) && isNotBlocked(newCell))
                .toList();
    }

    private boolean isNotBlocked(PathCell pathCell) {
        final PhysicsService service = PhysicsService.getInstance();
        return !service.queryExists(new Vector2f(pathCell.x(), pathCell.y()), new Vector2i(selfWidth, selfHeight),
                EnumSet.of(CollisionType.ENEMY, CollisionType.PLAYER, CollisionType.TRAP, CollisionType.ENEMY_BULLET, CollisionType.PLAYER_BULLET, CollisionType.ITEM, CollisionType.ROOM_OBJECT));
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
        final float x = position.x();
        final float y = position.y();
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
            final Vector2f direction = new Vector2f(target.getX() + target.getWidth() / 2f - position.x() - selfWidth / 2f, target.getY() + target.getHeight() / 2f - position.y() - selfHeight / 2f);
            if (direction.length() > 0) {
                direction.normalize();
            }
            return direction;
        }
        final PathCell target = path.peek();
        final float x = position.x();
        final float y = position.y();
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

    public boolean hasTarget() {
        return target != null;
    }
}
