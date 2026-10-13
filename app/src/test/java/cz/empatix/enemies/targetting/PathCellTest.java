package cz.empatix.enemies.targetting;

import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

class PathCellTest {

    @Test
    void convenienceConstructorComputesFAsGPlusH() {
        final PathCell cell = new PathCell(1, 2, 3f, 4f, null);
        assertEquals(3f, cell.g());
        assertEquals(4f, cell.h());
        assertEquals(7f, cell.f());
        assertNull(cell.parent());
    }

    @Test
    void canonicalConstructorKeepsExplicitF() {
        final PathCell cell = new PathCell(0, 0, 1f, 1f, 99f, null);
        assertEquals(99f, cell.f());
    }

    @Test
    void parentIsRetained() {
        final PathCell parent = new PathCell(0, 0, 0f, 0f, null);
        final PathCell child = new PathCell(1, 1, 1f, 1f, parent);
        assertSame(parent, child.parent());
    }

    @Test
    void equalityDependsOnlyOnCoordinates() {
        final PathCell a = new PathCell(5, 6, 1f, 2f, null);
        final PathCell b = new PathCell(5, 6, 99f, 99f, new PathCell(0, 0, 0f, 0f, null));

        assertEquals(a, b, "Cells with same x,y are equal regardless of cost/parent");
        assertEquals(a.hashCode(), b.hashCode());
    }

    @Test
    void differentCoordinatesAreNotEqual() {
        final PathCell a = new PathCell(5, 6, 0f, 0f, null);
        final PathCell c = new PathCell(5, 7, 0f, 0f, null);
        final PathCell d = new PathCell(6, 6, 0f, 0f, null);

        assertNotEquals(a, c);
        assertNotEquals(a, d);
    }

    @Test
    void notEqualToNullOrOtherType() {
        final PathCell a = new PathCell(1, 1, 0f, 0f, null);
        assertNotEquals(a, null);
        assertNotEquals(a, "not a cell");
    }

    @Test
    void equalsToItself() {
        final PathCell a = new PathCell(1, 1, 0f, 0f, null);
        assertEquals(a, a);
    }

    @Test
    void setMembershipUsesCoordinateEquality() {
        final Set<PathCell> visited = new HashSet<>();
        visited.add(new PathCell(2, 3, 0f, 0f, null));

        assertTrue(visited.contains(new PathCell(2, 3, 5f, 5f, null)),
                "A cell with the same coordinates should be considered already visited");
        assertFalse(visited.contains(new PathCell(2, 4, 0f, 0f, null)));
    }

    @Test
    void hashCodeMatchesDocumentedFormula() {
        final PathCell cell = new PathCell(4, 9, 0f, 0f, null);
        assertEquals(31 * 4 + 9, cell.hashCode());
    }
}
