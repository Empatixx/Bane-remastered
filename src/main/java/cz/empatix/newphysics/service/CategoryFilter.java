package cz.empatix.newphysics.service;

import org.dyn4j.collision.Filter;

public class CategoryFilter implements Filter {
    public static final CategoryFilter PLAYER = new CategoryFilter(CollisionType.PLAYER);
    public static final CategoryFilter ENEMY = new CategoryFilter(CollisionType.ENEMY);
    private final CollisionType type;

    public CategoryFilter(final CollisionType type) {
        this.type = type;
    }

    @Override
    public boolean isAllowed(Filter filter) {
        if (!(filter instanceof CategoryFilter categoryFilter)) {
            return true;
        }
        return categoryFilter.type != type;
    }
}
