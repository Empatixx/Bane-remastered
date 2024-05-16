package cz.empatix.pets;

import cz.empatix.enemies.domain.Enemy;
import cz.empatix.enemies.targetting.AStarTargetingState;
import cz.empatix.enemies.targetting.AStarTargetting;
import cz.empatix.enemies.targetting.Targetable;
import cz.empatix.interaction.domain.Interactable;
import cz.empatix.interaction.service.InteractionService;
import cz.empatix.items.domain.*;
import cz.empatix.newphysics.attributes.PhysicsObject;
import org.joml.Vector2f;
import org.joml.Vector2i;

import java.util.List;

public class DropFollowingStrategy implements FollowingStrategy{
    private static final int AREA_LOOT = 500;
    private AStarTargetting targetting;
    private ItemDrop followTarget;
    private final Pet self;
    private static final List<Class<?>> IGNORED_DROPS = List.of(ShopItem.class, SacrificialItem.class,
            ArtefactDrop.class, WeaponDrop.class);
    public DropFollowingStrategy(Pet self) {
        targetting = new AStarTargetting(self::getPosition, self.getWidth(), self.getHeight());
        this.self = self;
    }

    public void findDrop() {
        final InteractionService interactionService = InteractionService.getInstance();

        final PhysicsObject physicsObject = self.getPhysicsObject();
        final int width = self.getWidth();
        final int height = self.getHeight();

        final Vector2f selfPosition = new Vector2f(physicsObject.getPhysicsTranslation().add(width / 2, height / 2))
                .sub(AREA_LOOT / 2, AREA_LOOT / 2);
        final List<Interactable> objects = interactionService.query(selfPosition, new Vector2i(AREA_LOOT, AREA_LOOT));
        if (!objects.isEmpty()) {
            final List<ItemDrop> closest = objects.stream()
                    .sorted((o1, o2) -> {
                        final Vector2f o1Position = o1.getPosition();
                        final Vector2f o2Position = o2.getPosition();
                        return (int) (o1Position.distanceSquared(selfPosition) - o2Position.distanceSquared(selfPosition));
                    })
                    .filter(o -> o instanceof ItemDrop && !IGNORED_DROPS.contains(o.getClass()))
                    .map(o -> (ItemDrop) o)
                    .filter(item -> item.canInteract())
                    .toList();
            if (closest.isEmpty()) {
                return;
            }
            follow(closest.get(0));
        }
    }

    @Override
    public void fixedUpdate(final float dt) {
        if (!hasTarget()){
            findDrop();
            return;
        }
        performPathUpdate(dt);
    }

    @Override
    public boolean hasTarget() {
        return followTarget != null;
    }

    @Override
    public void follow(Targetable followTarget) {
        this.followTarget = (ItemDrop) followTarget;
        targetting.target(followTarget);
    }

    @Override
    public void update(final float dt) {
        if (followTarget == null){
            return;
        }
        if (!followTarget.canInteract() || followTarget.canRemove() || tooFarFromItem()) {
            followTarget = null;
        }
        targetting.update(dt);
    }

    private boolean tooFarFromItem() {
        final Vector2f selfPosition = self.getPhysicsObject().getPhysicsTranslation();
        final Vector2f targetPosition = followTarget.getPosition();
        return selfPosition.distance(targetPosition) > AREA_LOOT;
    }

    protected AStarTargetingState performPathUpdate(final float dt) {
        final AStarTargetingState pathFound = targetting.updatePath();
        targetting.update(dt);
        return pathFound;
    }
    @Override

    public Vector2f getDirection() {
        return targetting.getDirectionTarget();
    }
}
