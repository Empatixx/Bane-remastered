# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Build the project (creates shaded JAR)
mvn clean package

# Run the game
java -jar target/Bane-1.0-SNAPSHOT.jar

# Compile without tests
mvn compile
```

## Project Overview

Bane is a 2D top-down roguelike shooter game built with Java 17, LWJGL 3 (OpenGL), and dyn4j physics engine. It features procedural map generation, multiple weapons, enemies with AI, and Steam/Discord integration.

## Architecture

### Game Loop and State Machine

The entry point is `BaneApplication.main()` which creates `GameLoop`. The game uses a fixed timestep for physics (60 UPS) with variable render rate:

- `GameLoop`: Main loop with `fixedUpdate()` (physics) and `update()`/`render()` (variable timestep)
- `GameStateService`: Manages state transitions between `LoadingState`, `MenuState`, `LobbyState`, and `InGameState`
- `GameStateController`: Singleton that handles state change requests from anywhere in the codebase

### Rendering Pipeline

The rendering system uses a layered approach with post-processing:

1. **RenderEngine** collects all `Renderable` objects (including nested sub-renderables via tree traversal)
2. Groups by `RenderScene` (SCENE, HUD, OVERLAYS) and renders each to framebuffer
3. **PostprocessService** applies effects per scene
4. Objects implement `Drawable` interface which provides `RenderData`, `RenderScene`, and z-index

Key abstractions:
- `Renderable`: Base interface with `getRenderableGroup()` and `getSubRenderables()`
- `Drawable`: Extends Renderable, adds render data, scene, z-index, and visibility check
- `RenderableGroup`: Groups drawables by shader binding for batch rendering

### Physics System

Uses dyn4j library wrapped in custom services:

- `PhysicsService`: Singleton facade for physics operations
- `PhysicsEngine`: Manages physics world, collision detection, and interpolation
- `PhysicsObject`/`InterpolatedPhysicsObject`: Physics body wrappers with position interpolation for smooth rendering
- `CollisionType`: Enum categorizing collision bodies (PLAYER, ENEMY, BULLET, WALL, etc.)
- `DamageCollisionListener`: Handles damage when DamageSource hits Damageable

### Asset Management

Assets are loaded asynchronously during `LoadingState`:

- `AssetsService`: Static service providing `getTexture()`, `getAudio()`, `getShader()`, `getFont()`
- Loaders (`TexturesLoader`, `AudioLoader`, etc.): Read raw data from disk
- Creators (`TexturesCreator`, `AudioCreator`, etc.): Upload to GPU/audio system
- `TextureAtlas`: Packs multiple textures into atlas for efficient rendering

### Domain/Service Pattern

The codebase follows a consistent package structure:
- `domain/`: Data classes and entities (e.g., `Gun`, `Enemy`, `Affix`)
- `service/`: Business logic and managers (e.g., `GunService`, `EnemyService`)
- `factory/`: Object creation (e.g., `BulletFactory`, `PhysicsObjectFactory`)

### Key Systems

- **Player**: Implements `Drawable`, `InteractEntity`, `Damageable`. Uses hook composites for damage/movement modifiers
- **Guns**: Abstract gun system with specific implementations (Pistol, Shotgun, M4, etc.)
- **Enemies**: AI behaviors, targeting system, boss variants
- **Map**: Procedural generation via `ProceduralMapGenerator`, rooms with objects and enemies
- **Items**: Collectibles handled by `ItemService` and `ItemFactory`
- **Upgrades**: Persistent upgrades loaded from file, applied via `UpgradesApplicationService`

### Initialization

`GameInitializer` sets up rendering context in order:
1. `GLFWInit`: Window and GLFW context
2. `GLInit`: OpenGL state
3. `ALInit`: OpenAL audio context

### Asset Locations

- `assets/textures/`: Sprite textures (.tga format)
- `assets/audio/`: Sound effects and music
- `assets/shaders/`: GLSL vertex/fragment shaders
- `assets/fonts/`: Font files
