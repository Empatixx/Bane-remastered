# M1 — Dodge-Roll (i-frames) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give the player a cooldown-gated **dodge-roll** — a short directional movement burst that grants brief invulnerability (i-frames) — the keystone skill mechanic of milestone **M1 (Combat feel)** in [`TODO.md`](../../../TODO.md).

**Architecture:** A pure, GL-free `DodgeRoll` state machine (cooldown + active window + remembered direction, with an injectable millisecond clock for deterministic tests) lives in `cz.empatix.player`. `Player` owns one `DodgeRoll`, starts it from a new `SPACE` key case using the current movement direction, applies a fixed dodge force in `fixedUpdate` while it is active (bypassing the normal walk force), and reports `isInvulnerable()` during the active window. Input is already routed `InputHandler → GameLoop → GameStateService → InGameState → player.keyPressed(key)`, so **no upstream wiring changes are needed.**

**Tech Stack:** Java 17, Maven multi-module reactor, JUnit 5 (`junit-jupiter`), Mockito (already on the `app` test classpath), dyn4j physics (via the existing `PhysicsObject`/`InterpolatedPhysicsObject` wrappers), LWJGL/GLFW key codes.

## Global Constraints

- Player lives in the **`app`** module: `app/src/main/java/cz/empatix/player/`, tests in `app/src/test/java/cz/empatix/player/`.
- Fast unit tests run by default; the slow suite is `@Tag("integration")` and excluded unless `-Pintegration` is passed. **The tests in this plan are fast and MUST NOT be tagged `integration`.**
- Headless tests that construct `Player` use `@ExtendWith(cz.empatix.testsupport.HeadlessAssets.class)` (disables OpenAL, spins up a real headless dyn4j `PhysicsService`, stubs `AssetsService`). The pure `DodgeRoll` test needs **no** harness.
- Existing test style: JUnit 5 (`org.junit.jupiter.api.*`), GLFW key constants referenced fully-qualified as `org.lwjgl.glfw.GLFW.GLFW_KEY_*`, time-sensitive logic driven by injected/forced time rather than `Thread.sleep`.
- The dash sound already exists: `SoundEffect.PLAYER_DASH` (no asset or enum work required).
- `final` on parameters/locals where the surrounding class already does so (`Player` does).

---

## File Structure

- **Create** `app/src/main/java/cz/empatix/player/DodgeRoll.java` — pure dodge state machine. One responsibility: "am I ready, am I currently dodging, in which (normalized) direction", with time injected.
- **Create** `app/src/test/java/cz/empatix/player/DodgeRollTest.java` — deterministic unit tests for `DodgeRoll` using a mutable fake clock (no GL, no harness).
- **Modify** `app/src/main/java/cz/empatix/player/Player.java` — add a `DodgeRoll` field + `DODGE_FORCE` constant, a `SPACE` case calling a new `startDodge()`, include the dodge window in `isInvulnerable()`, and apply the dodge burst in `fixedUpdate()`.
- **Modify** `app/src/test/java/cz/empatix/player/PlayerTest.java` — add three wiring tests (i-frames during dodge, dodge needs a direction, active dodge bypasses the walk force).

No other files change. `InGameState`, `GameLoop`, `SoundEffect`, and assets are already sufficient.

---

## Task 1: `DodgeRoll` pure state machine

**Files:**
- Create: `app/src/main/java/cz/empatix/player/DodgeRoll.java`
- Test: `app/src/test/java/cz/empatix/player/DodgeRollTest.java`

**Interfaces:**
- Consumes: nothing (pure; `java.util.function.LongSupplier` from the JDK).
- Produces (relied on by Task 2):
  - `DodgeRoll()` — production constructor (1200 ms cooldown, 250 ms duration, `System::currentTimeMillis`).
  - `DodgeRoll(long cooldownMs, long durationMs, LongSupplier clockMs)` — test constructor.
  - `boolean tryStart(float dirX, float dirY)` — starts a dodge if ready and the direction is non-zero; normalizes and remembers the direction; returns whether it started.
  - `boolean isReady()` — cooldown elapsed since the last start.
  - `boolean isActive()` — currently inside the dodge window.
  - `float directionX()` / `float directionY()` — last normalized dodge direction.
  - Public constants `DEFAULT_COOLDOWN_MS = 1200`, `DEFAULT_DURATION_MS = 250`.

- [ ] **Step 1: Write the failing test**

Create `app/src/test/java/cz/empatix/player/DodgeRollTest.java`:

```java
package cz.empatix.player;

import org.junit.jupiter.api.Test;

import java.util.function.LongSupplier;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * Deterministic, GL-free coverage of the dodge-roll state machine. Time is a mutable fake clock
 * (millis) so the cooldown and active window are exercised without sleeping.
 */
class DodgeRollTest {

    private final long[] now = {10_000L};
    private final LongSupplier clock = () -> now[0];

    private DodgeRoll newDodge() {
        return new DodgeRoll(1000, 200, clock); // 1000ms cooldown, 200ms window
    }

    @Test
    void freshDodgeIsReadyAndInactive() {
        final DodgeRoll dodge = newDodge();

        assertTrue(dodge.isReady());
        assertFalse(dodge.isActive());
    }

    @Test
    void startingInADirectionActivatesAndNormalizesTheDirection() {
        final DodgeRoll dodge = newDodge();

        final boolean started = dodge.tryStart(3f, 4f); // length 5

        assertTrue(started);
        assertTrue(dodge.isActive());
        assertEquals(0.6f, dodge.directionX(), 1e-4);
        assertEquals(0.8f, dodge.directionY(), 1e-4);
    }

    @Test
    void aDodgeWithNoDirectionDoesNotStart() {
        final DodgeRoll dodge = newDodge();

        final boolean started = dodge.tryStart(0f, 0f);

        assertFalse(started);
        assertFalse(dodge.isActive());
        assertTrue(dodge.isReady(), "a rejected dodge does not consume the cooldown");
    }

    @Test
    void theDodgeStaysActiveForItsDurationThenEnds() {
        final DodgeRoll dodge = newDodge();
        dodge.tryStart(1f, 0f);

        now[0] += 199; // still inside the 200ms window
        assertTrue(dodge.isActive());

        now[0] += 1; // exactly at 200ms -> window closed
        assertFalse(dodge.isActive());
    }

    @Test
    void theCooldownBlocksASecondDodgeUntilItElapses() {
        final DodgeRoll dodge = newDodge();
        assertTrue(dodge.tryStart(1f, 0f));

        now[0] += 200; // window over, cooldown (1000) not yet
        assertFalse(dodge.isReady());
        assertFalse(dodge.tryStart(1f, 0f), "still on cooldown");

        now[0] += 800; // 1000ms total since the start -> cooldown elapsed
        assertTrue(dodge.isReady());
        assertTrue(dodge.tryStart(1f, 0f));
    }

    @Test
    void theRememberedDirectionIsNotOverwrittenWhileActive() {
        final DodgeRoll dodge = newDodge();
        dodge.tryStart(0f, -2f); // up; normalizes to (0, -1)

        assertEquals(0f, dodge.directionX(), 1e-4);
        assertEquals(-1f, dodge.directionY(), 1e-4);

        now[0] += 50; // still active and on cooldown
        assertFalse(dodge.tryStart(1f, 0f), "rejected while active");
        assertEquals(0f, dodge.directionX(), 1e-4);
        assertEquals(-1f, dodge.directionY(), 1e-4);
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `mvn -pl app -am -Dtest=DodgeRollTest test`
Expected: **FAILS to compile** — `cannot find symbol: class DodgeRoll` (the production class does not exist yet). That compile failure is our red state.

- [ ] **Step 3: Write the minimal implementation**

Create `app/src/main/java/cz/empatix/player/DodgeRoll.java`:

```java
package cz.empatix.player;

import java.util.function.LongSupplier;

/**
 * Pure, GL-free state machine for the player's dodge-roll: a short directional burst that also
 * grants brief invulnerability (i-frames), gated by a cooldown measured from the start of the dodge.
 * Time is injected as a millisecond {@link LongSupplier} so the logic is deterministically testable
 * without sleeping. Absolute timestamps avoid any "uninitialized" subtraction overflow.
 */
public class DodgeRoll {

    public static final long DEFAULT_COOLDOWN_MS = 1200;
    public static final long DEFAULT_DURATION_MS = 250;

    private final long cooldownMs;
    private final long durationMs;
    private final LongSupplier clockMs;

    private long readyAtMs = Long.MIN_VALUE;     // ready immediately
    private long activeUntilMs = Long.MIN_VALUE; // inactive until a dodge starts
    private float directionX;
    private float directionY;

    public DodgeRoll() {
        this(DEFAULT_COOLDOWN_MS, DEFAULT_DURATION_MS, System::currentTimeMillis);
    }

    public DodgeRoll(final long cooldownMs, final long durationMs, final LongSupplier clockMs) {
        this.cooldownMs = cooldownMs;
        this.durationMs = durationMs;
        this.clockMs = clockMs;
    }

    /**
     * Attempts to start a dodge in the given direction. Succeeds only if the cooldown has elapsed and
     * the direction is non-zero. The direction is normalized and remembered for the whole active
     * window, so releasing the movement key mid-roll does not change it.
     *
     * @return {@code true} if a dodge actually started.
     */
    public boolean tryStart(final float dirX, final float dirY) {
        if (!isReady()) {
            return false;
        }
        final float length = (float) Math.sqrt(dirX * dirX + dirY * dirY);
        if (length == 0f) {
            return false;
        }
        directionX = dirX / length;
        directionY = dirY / length;
        final long now = clockMs.getAsLong();
        activeUntilMs = now + durationMs;
        readyAtMs = now + cooldownMs;
        return true;
    }

    public boolean isReady() {
        return clockMs.getAsLong() >= readyAtMs;
    }

    public boolean isActive() {
        return clockMs.getAsLong() < activeUntilMs;
    }

    public float directionX() {
        return directionX;
    }

    public float directionY() {
        return directionY;
    }
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `mvn -pl app -am -Dtest=DodgeRollTest test`
Expected: **PASS** — `Tests run: 6, Failures: 0, Errors: 0, Skipped: 0`.

- [ ] **Step 5: Commit**

```bash
git add app/src/main/java/cz/empatix/player/DodgeRoll.java app/src/test/java/cz/empatix/player/DodgeRollTest.java
git commit -m "feat(player): add DodgeRoll cooldown/i-frame state machine"
```

---

## Task 2: Wire the dodge-roll into `Player`

**Files:**
- Modify: `app/src/main/java/cz/empatix/player/Player.java`
- Test: `app/src/test/java/cz/empatix/player/PlayerTest.java`

**Interfaces:**
- Consumes (from Task 1): `DodgeRoll()`, `tryStart(float,float)`, `isActive()`, `directionX()`, `directionY()`.
- Produces: behavioral changes only — no new public methods. `SPACE` starts a dodge; an active dodge makes `isInvulnerable()` true (observable via `hit(...)` returning 0) and replaces the walk force with a fixed dodge burst in `fixedUpdate(...)`.

- [ ] **Step 1: Write the failing tests**

Add these three test methods inside the existing `class PlayerTest` in `app/src/test/java/cz/empatix/player/PlayerTest.java` (e.g. just before the closing brace; the class is already `@ExtendWith(HeadlessAssets.class)`):

```java
    @Test
    void dodgingGrantsInvulnerabilityFrames() {
        final Player player = newPlayer();
        player.setArmor(0);
        player.keyPressed(org.lwjgl.glfw.GLFW.GLFW_KEY_D);     // a movement direction exists
        player.keyPressed(org.lwjgl.glfw.GLFW.GLFW_KEY_SPACE); // start the dodge

        final int applied = player.hit(3);

        assertEquals(0, applied, "a hit during the dodge i-frames is ignored");
        assertEquals(10, player.getHealth());
    }

    @Test
    void dodgingWithoutAMovementDirectionDoesNothing() {
        final Player player = newPlayer();
        player.setArmor(0);
        player.keyPressed(org.lwjgl.glfw.GLFW.GLFW_KEY_SPACE); // no W/A/S/D held -> no dodge

        final int applied = player.hit(3);

        assertEquals(3, applied, "no dodge means no i-frames, so the hit lands");
        assertEquals(7, player.getHealth());
    }

    @Test
    void anActiveDodgeBypassesTheNormalMoveForceHook() {
        final Player player = newPlayer();
        final boolean[] hookCalled = {false};
        player.addHook(force -> {
            hookCalled[0] = true;
            return force;
        });
        player.keyPressed(org.lwjgl.glfw.GLFW.GLFW_KEY_D);     // direction
        player.keyPressed(org.lwjgl.glfw.GLFW.GLFW_KEY_SPACE); // dodge active

        player.fixedUpdate(0.016);

        assertFalse(hookCalled[0],
                "while dodging, movement is the fixed dodge burst, not the hooked walk force");
    }
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `mvn -pl app -am -Dtest=PlayerTest test`
Expected: **FAIL** (compiles fine — all referenced methods already exist — but the new assertions fail at runtime). Specifically:
- `dodgingGrantsInvulnerabilityFrames` → `expected: <0> but was: <3>` (SPACE currently does nothing, so the hit lands).
- `anActiveDodgeBypassesTheNormalMoveForceHook` → `expected: <false> but was: <true>` (no dodge, so `move()` still consults the hook).
- `dodgingWithoutAMovementDirectionDoesNothing` → passes already (no dodge wiring yet), which is fine.

- [ ] **Step 3: Implement the wiring in `Player.java`**

3a. Add the dodge force constant next to `MOVE_FORCE` (after `Player.java:40`):

```java
    private final static int MOVE_FORCE = 5_000_000;
    private final static int DODGE_FORCE = 30_000_000;
```

3b. Add the `DodgeRoll` field alongside the other final fields (e.g. right after the `moveForceHook` field declaration near `Player.java:53`):

```java
    private final MoveForceEventHookComposite moveForceHook;
    private final DodgeRoll dodgeRoll = new DodgeRoll();
```

3c. Replace `isInvulnerable()` (currently `Player.java:102-104`) so the dodge window also counts as invulnerable:

```java
    private boolean isInvulnerable() {
        return dodgeRoll.isActive() || System.currentTimeMillis() - lastHitTime < INVULNERABLE_TIME;
    }
```

3d. Replace `fixedUpdate(...)` (currently `Player.java:165-175`) so an active dodge applies the fixed burst and skips the normal walk force:

```java
    public void fixedUpdate(final double dt) {
        if (isDead()) {
            return;
        }
        if (dodgeRoll.isActive()) {
            physicsBody.applyForce(dodgeRoll.directionX() * DODGE_FORCE, dodgeRoll.directionY() * DODGE_FORCE);
        } else {
            move(dt);
        }
        final Vector2f velocity = physicsBody.getVelocity();
        if (Math.abs(velocity.x()) >= GAZE_SPEED_PRELIMIT || Math.abs(velocity.y()) >= GAZE_SPEED_PRELIMIT) {
            final Vector2f nextGazePosition = getPossibleGazePosition();
            gazewindService.spawn(nextGazePosition.x(), nextGazePosition.y());
        }
    }
```

3e. Add a `SPACE` case to the `keyPressed(...)` switch (the switch at `Player.java:300-317`), immediately after the `GLFW_KEY_D` case and before the closing brace of the switch:

```java
            case GLFW.GLFW_KEY_D -> {
                right = true;
                left = false;
            }
            case GLFW.GLFW_KEY_SPACE -> startDodge();
```

3f. Add the `startDodge()` helper. Place it directly below the `move(double dt)` method (after `Player.java:354`). It derives the direction from the current movement flags (note this codebase's convention: `bottom` is +y, `top` is −y, matching `move()`), and plays the dash sound only when a dodge actually starts:

```java
    private void startDodge() {
        if (isDead()) {
            return;
        }
        final float dirX = (right ? 1f : 0f) - (left ? 1f : 0f);
        final float dirY = (bottom ? 1f : 0f) - (top ? 1f : 0f);
        if (dodgeRoll.tryStart(dirX, dirY)) {
            AUDIO_SERVICE.play(SoundEffect.PLAYER_DASH);
        }
    }
```

- [ ] **Step 4: Run the tests to verify they pass**

Run: `mvn -pl app -am -Dtest=PlayerTest test`
Expected: **PASS** — all `PlayerTest` tests green, including the three new ones (`Failures: 0, Errors: 0`). The existing `moveForceHookIsAppliedDuringFixedUpdate` still passes because, with no dodge active, `fixedUpdate` takes the `else` branch and calls `move(dt)`.

- [ ] **Step 5: Run both new test classes together as a regression check**

Run: `mvn -pl app -am -Dtest=DodgeRollTest,PlayerTest test`
Expected: **PASS** — both classes green.

- [ ] **Step 6: Commit**

```bash
git add app/src/main/java/cz/empatix/player/Player.java app/src/test/java/cz/empatix/player/PlayerTest.java
git commit -m "feat(player): SPACE dodge-roll with i-frames and fixed dash burst"
```

---

## Manual playtest & tuning (after the automated tasks)

The unit tests verify the *logic*; the *feel* needs a human. Build and run, then in a run press **SPACE** while holding a movement direction:

```bash
mvn -q -pl app -am package -DskipTests
java -jar app/target/Bane-1.0-SNAPSHOT.jar
```

Confirm: the player lunges in the held direction, briefly flickers (the existing invulnerability flicker now also fires during the dodge), the dash sound plays, walking through enemy bullets mid-dodge deals no damage, and SPACE does nothing for ~1.2 s afterwards (cooldown). Tune `DodgeRoll.DEFAULT_COOLDOWN_MS`, `DodgeRoll.DEFAULT_DURATION_MS`, and `Player.DODGE_FORCE` to taste — actual roll distance depends on dyn4j damping, so these are deliberately easy to adjust in one place.

## Scope notes / what is intentionally NOT here (YAGNI)

- **Stationary dodge / aim-direction dodge:** v1 requires a held movement direction (`Player` does not currently track an aim vector; aim lives in `GunService`). A future plan can roll toward the cursor.
- **Dedicated roll animation:** no roll sprite exists; the current directional/idle animation is reused. A roll animation is a separate art + plan.
- **The rest of M1** (hitstop + screenshake, enemy attack telegraphs, shot-pattern system, weapon game-feel, gamepad, active reload) are **sibling plans** under `docs/superpowers/plans/`, per [`TODO.md`](../../../TODO.md).
