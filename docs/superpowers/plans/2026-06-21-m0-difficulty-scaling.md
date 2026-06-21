# M0 — Per-Floor Difficulty Scaling (common enemies) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make regular enemies get tougher as the player descends, by scaling each spawned **common** enemy's health and contact damage by a per-floor multiplier — the first slice of milestone **M0 (Stabilize & scale)** in [`TODO.md`](../../../TODO.md), where enemies currently do not scale at all.

**Architecture:** A pure `DifficultyScaling` utility computes health/damage multipliers from a floor number (floor 0 = neutral). The `Enemy` interface gains a `scaleToFloor(int)` **default no-op**; `BaseEnemy` (the parent of every common enemy) overrides it to multiply its `maxHealth`/`health`/`damage`. `EnemyFactory.createEnemy(...)` / `createBoss(...)` call `scaleToFloor(currentFloor)` on each freshly built enemy. Bosses (which split across `BossBaseEnemy` and a `final`-field `BossImpl`) keep the no-op and are intentionally **out of scope for v1** — boss scaling is a documented follow-up so we don't ship an inconsistent half-measure.

**Tech Stack:** Java 17, Maven multi-module reactor, JUnit 5 (`junit-jupiter`), Mockito, the headless test harness `cz.empatix.testsupport.HeadlessAssets`.

## Global Constraints

- Code in the **`app`** module: `app/src/main/java/cz/empatix/enemies/...`, tests under `app/src/test/java/cz/empatix/enemies/...`.
- Fast unit tests run by default; the slow suite is `@Tag("integration")`, excluded unless `-Pintegration`. **All tests in this plan are fast and MUST NOT be tagged `integration`.**
- Run a single test class with surefire's pattern filter; because `-am` also runs upstream engine modules that lack the named class, pass `-Dsurefire.failIfNoSpecifiedTests=false` so those modules don't error with *"No tests matching pattern"*:
  `mvn -q -pl app -am -Dtest=<Class> -Dsurefire.failIfNoSpecifiedTests=false test`
- Tests that construct a real enemy use `@ExtendWith(HeadlessAssets.class)`. Pure tests (`DifficultyScaling`) need no harness.
- `final` on parameters/locals, matching the surrounding enemy classes.
- Floor numbers come from `StatisticsService.getFloor()` and are already passed into `EnemyFactory` by `EnemyService` — **no change needed there**. Floor 0 is the first floor and must be a true no-op.

---

## File Structure

- **Create** `app/src/main/java/cz/empatix/enemies/service/DifficultyScaling.java` — pure multiplier math. One responsibility: "given a floor, how much do stats scale, and how is a single stat scaled to an int".
- **Create** `app/src/test/java/cz/empatix/enemies/service/DifficultyScalingTest.java` — pure unit tests (no harness).
- **Modify** `app/src/main/java/cz/empatix/enemies/domain/Enemy.java` — add `default void scaleToFloor(int floor) {}`.
- **Modify** `app/src/main/java/cz/empatix/enemies/domain/BaseEnemy.java` — override `scaleToFloor` to scale `maxHealth`/`health`/`damage`.
- **Modify** `app/src/test/java/cz/empatix/enemies/domain/BaseEnemyHeadlessTest.java` — add two scaling tests reusing the existing `TestEnemy`.
- **Modify** `app/src/main/java/cz/empatix/enemies/service/EnemyFactory.java` — call `scaleToFloor(currentFloor)` in `createEnemy` and `createBoss`.
- **Modify** `app/src/test/java/cz/empatix/enemies/service/EnemyFactoryTest.java` — add two `verify(...)` tests.

No other files change.

---

## Task 1: `DifficultyScaling` pure multiplier math

**Files:**
- Create: `app/src/main/java/cz/empatix/enemies/service/DifficultyScaling.java`
- Test: `app/src/test/java/cz/empatix/enemies/service/DifficultyScalingTest.java`

**Interfaces:**
- Consumes: nothing (pure, JDK `Math`).
- Produces (used by Tasks 2):
  - `static float healthMultiplier(int floor)` — `1 + 0.12 * max(0, floor)`.
  - `static float damageMultiplier(int floor)` — `1 + 0.08 * max(0, floor)`.
  - `static int scaleStat(int base, float multiplier)` — `max(1, round(base * multiplier))`.
  - Public constants `HEALTH_GROWTH_PER_FLOOR = 0.12f`, `DAMAGE_GROWTH_PER_FLOOR = 0.08f`.

- [ ] **Step 1: Write the failing test**

Create `app/src/test/java/cz/empatix/enemies/service/DifficultyScalingTest.java`:

```java
package cz.empatix.enemies.service;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Pure coverage of the per-floor difficulty multipliers and integer stat scaling. No GL, no harness.
 */
class DifficultyScalingTest {

    @Test
    void floorZeroIsNeutral() {
        assertEquals(1.0f, DifficultyScaling.healthMultiplier(0), 1e-6);
        assertEquals(1.0f, DifficultyScaling.damageMultiplier(0), 1e-6);
    }

    @Test
    void multipliersGrowLinearlyPerFloor() {
        assertEquals(1.6f, DifficultyScaling.healthMultiplier(5), 1e-4); // 1 + 0.12*5
        assertEquals(1.4f, DifficultyScaling.damageMultiplier(5), 1e-4); // 1 + 0.08*5
    }

    @Test
    void negativeFloorsAreClampedToNeutral() {
        assertEquals(1.0f, DifficultyScaling.healthMultiplier(-3), 1e-6);
        assertEquals(1.0f, DifficultyScaling.damageMultiplier(-3), 1e-6);
    }

    @Test
    void scaleStatRoundsToNearestIntAndNeverDropsBelowOne() {
        assertEquals(16, DifficultyScaling.scaleStat(10, 1.6f));
        assertEquals(7, DifficultyScaling.scaleStat(5, 1.4f));
        assertEquals(1, DifficultyScaling.scaleStat(1, 0.1f), "a stat never scales below 1");
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mvn -q -pl app -am -Dtest=DifficultyScalingTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **FAILS to compile** — `cannot find symbol: class DifficultyScaling`. That is our red state.

- [ ] **Step 3: Write the minimal implementation**

Create `app/src/main/java/cz/empatix/enemies/service/DifficultyScaling.java`:

```java
package cz.empatix.enemies.service;

/**
 * Per-floor difficulty multipliers for enemy stats. Floor 0 (the first floor) is neutral (x1.0);
 * each subsequent floor adds a fixed fraction. Pure and GL-free so it is trivially unit-tested and
 * easy to retune in one place.
 */
public final class DifficultyScaling {

    public static final float HEALTH_GROWTH_PER_FLOOR = 0.12f;
    public static final float DAMAGE_GROWTH_PER_FLOOR = 0.08f;

    private DifficultyScaling() {
    }

    public static float healthMultiplier(final int floor) {
        return 1f + HEALTH_GROWTH_PER_FLOOR * Math.max(0, floor);
    }

    public static float damageMultiplier(final int floor) {
        return 1f + DAMAGE_GROWTH_PER_FLOOR * Math.max(0, floor);
    }

    /** Applies a multiplier to a base stat, rounding to the nearest int and never dropping below 1. */
    public static int scaleStat(final int base, final float multiplier) {
        return Math.max(1, Math.round(base * multiplier));
    }
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `mvn -q -pl app -am -Dtest=DifficultyScalingTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **PASS** — `Tests run: 4, Failures: 0, Errors: 0`.

- [ ] **Step 5: Commit and push**

```bash
git add app/src/main/java/cz/empatix/enemies/service/DifficultyScaling.java \
        app/src/test/java/cz/empatix/enemies/service/DifficultyScalingTest.java
git commit -m "feat(enemies): add DifficultyScaling per-floor multiplier math"
git push
```

---

## Task 2: `Enemy.scaleToFloor` default + `BaseEnemy` override

**Files:**
- Modify: `app/src/main/java/cz/empatix/enemies/domain/Enemy.java`
- Modify: `app/src/main/java/cz/empatix/enemies/domain/BaseEnemy.java`
- Test: `app/src/test/java/cz/empatix/enemies/domain/BaseEnemyHeadlessTest.java`

**Interfaces:**
- Consumes (from Task 1): `DifficultyScaling.healthMultiplier`, `damageMultiplier`, `scaleStat`.
- Produces (used by Task 3): `Enemy.scaleToFloor(int floor)` — default no-op on the interface, overridden by `BaseEnemy` to scale `maxHealth`, refill `health` to the new max, and scale `damage`.

- [ ] **Step 1: Write the failing tests**

Add these two methods inside the existing `class BaseEnemyHeadlessTest` (it already defines the `TestEnemy` subclass and the `enemy(health, damage)` helper that leaves the spawn window):

```java
    @Test
    void scaleToFloorRaisesHealthAndDamageByTheFloorMultipliers() {
        final TestEnemy enemy = enemy(10, 5); // past the spawn window

        enemy.scaleToFloor(5);

        // health: round(10 * (1 + 0.12*5)) = round(16.0) = 16, current health refilled to the new max
        assertEquals(16, enemy.getHealth());
        // damage: round(5 * (1 + 0.08*5)) = round(7.0) = 7
        assertEquals(7, enemy.damage());
    }

    @Test
    void scaleToFloorZeroLeavesStatsUnchanged() {
        final TestEnemy enemy = enemy(9, 2);

        enemy.scaleToFloor(0);

        assertEquals(9, enemy.getHealth());
        assertEquals(2, enemy.damage());
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mvn -q -pl app -am -Dtest=BaseEnemyHeadlessTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **FAIL** — `scaleToFloorRaisesHealthAndDamageByTheFloorMultipliers` fails because the interface default no-op leaves health at 10 / damage at 5 (`expected: <16> but was: <10>`). `scaleToFloorZeroLeavesStatsUnchanged` passes already (no-op). NOTE: this step compiles only after Step 3a adds the method to the interface — so do 3a first if the compiler complains the symbol is missing, then re-run. (The default method makes the call site compile; the override in 3b makes the assertion pass.)

- [ ] **Step 3: Implement**

3a. In `app/src/main/java/cz/empatix/enemies/domain/Enemy.java`, add the default method just before the ShadowCaster defaults (after the `despawn()` default, around line 40):

```java
    default void despawn(){
        hit(getHealth());
    }

    /** Scales this enemy's stats for the given floor. Default is a no-op; common enemies override. */
    default void scaleToFloor(final int floor) {
    }
```

3b. In `app/src/main/java/cz/empatix/enemies/domain/BaseEnemy.java`, add the import and the override. Add the import near the other service import (top of file):

```java
import cz.empatix.enemies.service.DifficultyScaling;
```

Add the override method (e.g. directly after the `heal(int)` method, around line 161):

```java
    @Override
    public void scaleToFloor(final int floor) {
        maxHealth = DifficultyScaling.scaleStat(maxHealth, DifficultyScaling.healthMultiplier(floor));
        health = maxHealth;
        damage = DifficultyScaling.scaleStat(damage, DifficultyScaling.damageMultiplier(floor));
    }
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `mvn -q -pl app -am -Dtest=BaseEnemyHeadlessTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **PASS** — all `BaseEnemyHeadlessTest` tests green (the existing 11 plus the 2 new ones).

- [ ] **Step 5: Commit and push**

```bash
git add app/src/main/java/cz/empatix/enemies/domain/Enemy.java \
        app/src/main/java/cz/empatix/enemies/domain/BaseEnemy.java \
        app/src/test/java/cz/empatix/enemies/domain/BaseEnemyHeadlessTest.java
git commit -m "feat(enemies): scale common-enemy health and damage per floor"
git push
```

---

## Task 3: Apply scaling in `EnemyFactory`

**Files:**
- Modify: `app/src/main/java/cz/empatix/enemies/service/EnemyFactory.java`
- Test: `app/src/test/java/cz/empatix/enemies/service/EnemyFactoryTest.java`

**Interfaces:**
- Consumes (from Task 2): `Enemy.scaleToFloor(int)`.
- Produces: every enemy returned by `createEnemy(floor)` / `createBoss(floor)` has been scaled to that floor.

- [ ] **Step 1: Write the failing tests**

Add the Mockito `verify` import to `EnemyFactoryTest` (next to the existing `import static org.mockito.Mockito.mock;`):

```java
import static org.mockito.Mockito.verify;
```

Add these two test methods inside `class EnemyFactoryTest`:

```java
    @Test
    void createEnemyScalesTheSpawnedEnemyToTheFloor() throws Exception {
        final Enemy enemy = mock(Enemy.class);
        seed("commonEnemiesSuppliers", List.of(supplierFor(enemy, 0, 100)));

        EnemyFactory.createEnemy(7);

        verify(enemy).scaleToFloor(7);
    }

    @Test
    void createBossScalesTheSpawnedBossToTheFloor() throws Exception {
        final Enemy boss = mock(Enemy.class);
        seed("bossEnemiesSuppliers", List.of(supplierFor(boss, 0, 100)));

        EnemyFactory.createBoss(7);

        verify(boss).scaleToFloor(7);
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mvn -q -pl app -am -Dtest=EnemyFactoryTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **FAIL** — both new tests fail with a Mockito "Wanted but not invoked: enemy.scaleToFloor(7)" error, because `createEnemy`/`createBoss` do not call it yet. The five existing tests still pass.

- [ ] **Step 3: Implement**

In `app/src/main/java/cz/empatix/enemies/service/EnemyFactory.java`, replace the final `return` of `createEnemy` (currently `return possibleSuppliers.get(index).supplier().get();`) with:

```java
        final int index = random.nextInt(possibleSuppliers.size());
        final Enemy enemy = possibleSuppliers.get(index).supplier().get();
        enemy.scaleToFloor(currentFloor);
        return enemy;
```

And replace the matching final `return` of `createBoss` with:

```java
        final int index = random.nextInt(possibleSuppliers.size());
        final Enemy boss = possibleSuppliers.get(index).supplier().get();
        boss.scaleToFloor(currentFloor);
        return boss;
```

(`Enemy` is already imported in `EnemyFactory` via `import cz.empatix.enemies.domain.*;`.)

- [ ] **Step 4: Run the tests to verify they pass**

Run: `mvn -q -pl app -am -Dtest=EnemyFactoryTest -Dsurefire.failIfNoSpecifiedTests=false test`
Expected: **PASS** — all `EnemyFactoryTest` tests green (5 existing + 2 new). The existing `assertSame` tests still hold because the same instance is returned after being scaled.

- [ ] **Step 5: Commit and push**

```bash
git add app/src/main/java/cz/empatix/enemies/service/EnemyFactory.java \
        app/src/test/java/cz/empatix/enemies/service/EnemyFactoryTest.java
git commit -m "feat(enemies): apply per-floor scaling to spawned enemies and bosses"
git push
```

---

## Final verification

- [ ] **Run the module's fast unit suite to confirm no regressions**

Run: `mvn -q -pl app -am test`
Expected: **BUILD SUCCESS**, `Failures: 0, Errors: 0` (the `@Tag("integration")` suite is excluded by default).

- [ ] **Push the final state**

```bash
git push
```

## Scope notes / follow-ups (YAGNI for v1)

- **Boss scaling:** `Spider` (`BossBaseEnemy`) and `KingSlime`/`Golem` (composing `BossImpl`, whose `maxHealth` is `final`) keep the no-op `scaleToFloor`. A follow-up M0 task should make `BossImpl.maxHealth` mutable and override `scaleToFloor` on `BossBaseEnemy` + the `BossImpl`-delegating bosses, so all bosses scale consistently.
- **Move-force / spawn-density scaling:** out of scope here; this slice scales health + contact damage only. Speed and spawn count are separate tuning knobs for later M0 work.
- **Tuning:** `HEALTH_GROWTH_PER_FLOOR` / `DAMAGE_GROWTH_PER_FLOOR` live in one place; adjust after playtesting. A future curve (compounding or capped) can replace the linear form without touching call sites.
