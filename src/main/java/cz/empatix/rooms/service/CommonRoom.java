package cz.empatix.rooms.service;

import cz.empatix.enemies.service.EnemyService;
import cz.empatix.items.service.ItemService;
import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomLockService;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.roomobjects.suppliers.*;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.ProceduralTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;

import java.util.*;
import java.util.concurrent.ThreadLocalRandom;
import java.util.stream.Stream;

public class CommonRoom implements Room {
    private static final int MAX_ENEMIES = 5;
    private static final int MAX_SPIKES = 3;
    private static final int MAX_TORCHES = 2;
    private static final int MAX_BONES = 2;
    private static final int MAX_COBWEBS = 2;
    private static final int MAX_BANNERS = 2;
    private static final int MAX_BARRELS = 2;
    private final EnemyService enemyService;
    private final RoomObjectService roomObjectService;
    private final ItemService itemService;
    private final RoomLockService roomLockService;
    private final Map<RoomDirection, Room> neighbors;
    private final TileMap tileMap;
    private boolean visited;

    public CommonRoom(final ItemService itemService, final int rows, final int cols, final int xOffset, final int yOffset,
                      final Set<RoomDirection> neighbors) {

        this.itemService = itemService;
        tileMap = new ProceduralTileMap(rows, cols, xOffset, yOffset, neighbors);
        enemyService = new EnemyService(itemService);
        roomObjectService = new RoomObjectService();
        roomLockService = new RoomLockService(this, neighbors);
        visited = false;
        this.neighbors = new HashMap<>(neighbors.size());
    }

    @Override
    public void enterEvent(final Player p) {
        if (!visited) {
            visited = true;
            for (int i = 0; i < MAX_ENEMIES; i++) {
                enemyService.spawnEnemy(this);
            }
            enemyService.setTarget(p);
            roomLockService.setState(true);
        }
    }

    @Override
    public void leaveEvent(final Player p) {
    }

    @Override
    public void init() {
        final SpikeSupplier spikeSupplier = new SpikeSupplier(this, tileMap);
        final TorchSupplier torchSupplier = new TorchSupplier(this, tileMap);
        final BonesSupplier bonesSupplier = new BonesSupplier(this, tileMap);
        final CobwebSupplier cobwebSupplier = new CobwebSupplier(this, tileMap);
        final BannerSupplier bannerSupplier = new BannerSupplier(this, tileMap);
        final BarrelSupplier barrelSupplier = new BarrelSupplier(this, tileMap);
        final FlamethrowerSupplier flamethrowerSupplier = new FlamethrowerSupplier(this, tileMap);

        final List<Spike> spikes = Stream.generate(spikeSupplier::supply)
                .limit(ThreadLocalRandom.current().nextInt(1, MAX_SPIKES))
                .toList();
        final List<Torch> torches = Stream.generate(torchSupplier::supply)
                .limit(MAX_TORCHES)
                .toList();
        final List<Bones> bones = Stream.generate(bonesSupplier::supply)
                .limit(MAX_BONES)
                .toList();
        final List<Cobweb> cobwebs = Stream.generate(cobwebSupplier::supply)
                .limit(MAX_COBWEBS)
                .toList();
        final List<Banner> banners = Stream.generate(bannerSupplier::supply)
                .limit(MAX_BANNERS)
                .toList();
        final List<Barrel> barrels = Stream.generate(barrelSupplier::supply)
                .limit(MAX_BARRELS)
                .toList();
        final List<Flamethrower> flamethrowers = Stream.generate(flamethrowerSupplier::supply)
                .limit(1)
                .filter(Objects::nonNull)
                .toList();
        roomObjectService.addAll(spikes);
        roomObjectService.addAll(torches);
        roomObjectService.addAll(bones);
        roomObjectService.addAll(cobwebs);
        roomObjectService.addAll(banners);
        roomObjectService.addAll(barrels);
        roomObjectService.addAll(flamethrowers);
    }

    @Override
    public void cleanUp() {
        enemyService.cleanUp();
        roomObjectService.cleanUp();
        roomLockService.cleanUp();
        tileMap.cleanUp();
    }

    @Override
    public void update(double dt) {
        enemyService.update(dt);
        roomObjectService.update(dt);
        roomLockService.update(dt);
        if (enemyService.areEnemiesDead() && roomLockService.isLocked()) {
            roomLockService.setState(false);
        }
    }

    @Override
    public void fixedUpdate(double dt) {
        enemyService.fixedUpdate(dt);
    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable roomLocker = (DrawableBatch) this.roomLockService::toRender;
        final List<Renderable> roomObjects = List.copyOf(roomObjectService.toRender());

        final List<Renderable> renderables = new ArrayList<>(enemyService.toRender());
        renderables.add(tiles);
        renderables.addAll(roomObjects);
        renderables.add(roomLocker);
        return renderables;
    }

    @Override
    public int getWidth() {
        return tileMap.getWidth();
    }

    @Override
    public int getHeight() {
        return tileMap.getHeight();
    }

    @Override
    public int getX() {
        return tileMap.getX();
    }

    @Override
    public int getY() {
        return tileMap.getY();
    }

    @Override
    public void addNeighbour(RoomDirection direction, Room room) {
        neighbors.put(direction, room);
    }

    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        return neighbors;
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.NORMAL;
    }

}
