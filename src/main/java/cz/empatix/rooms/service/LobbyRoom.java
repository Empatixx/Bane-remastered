package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.Tile;
import cz.empatix.tiles.tilemaps.LobbyTileMap;
import cz.empatix.tiles.tilemaps.TileMap;
import org.joml.Vector2f;

import java.util.List;
import java.util.Map;

public class LobbyRoom implements Room {
    private final TileMap tileMap;
    private final RoomObjectService roomObjectService;

    public LobbyRoom() {
        tileMap = new LobbyTileMap();
        roomObjectService = new RoomObjectService();
    }

    @Override
    public void enterEvent(Player p) {

    }

    @Override
    public void leaveEvent(Player p) {

    }

    @Override
    public void init() {
        {
            final Portal portal = new Portal(new Vector2f((getMinX() + getMaxX()) / 2f - 96, Tile.SCALE * Tile.TILE_SIZE / 2f));
            roomObjectService.add(portal);
        }
        {
            final WeaponRack weaponRack = new WeaponRack();
            final WeaponRack weaponRack2 = new WeaponRack();
            weaponRack.setPosition(Tile.SCALE_TILE_SIZE * 22.75f, Tile.SCALE_TILE_SIZE/2f);
            weaponRack2.setPosition(Tile.SCALE_TILE_SIZE * 9.25f, Tile.SCALE_TILE_SIZE/2f);
            roomObjectService.add(weaponRack);
            roomObjectService.add(weaponRack2);
        }
        {
            final Torch torch1 = new Torch();
            final Torch torch2 = new Torch();
            final Torch torch3 = new Torch();
            final Torch torch4 = new Torch();

            torch1.setPosition(Tile.TILE_SIZE * Tile.SCALE, Tile.TILE_SIZE * Tile.SCALE * 2);
            torch2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 10, Tile.TILE_SIZE * Tile.SCALE * 2);
            torch3.setPosition(Tile.TILE_SIZE * Tile.SCALE * 18, Tile.TILE_SIZE * Tile.SCALE * 2);
            torch4.setPosition(Tile.TILE_SIZE * Tile.SCALE * 27, Tile.TILE_SIZE * Tile.SCALE * 2);

            torch2.setState(AnimationState.RIGHT_SIDE);
            torch4.setState(AnimationState.RIGHT_SIDE);

            roomObjectService.add(torch1);
            roomObjectService.add(torch2);
            roomObjectService.add(torch3);
            roomObjectService.add(torch4);
        }
        {
            final Bones bones1 = new Bones();
            final Bones bones2 = new Bones();

            bones1.setPosition(Tile.TILE_SIZE * Tile.SCALE * 20, Tile.SCALE_TILE_SIZE * 7);
            bones2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 23, Tile.TILE_SIZE * Tile.SCALE * 4);

            bones1.setState(AnimationState.TYPE1);
            bones2.setState(AnimationState.TYPE0);

            roomObjectService.add(bones1);
            roomObjectService.add(bones2);
        }
        {
            final Cobweb cobweb1 = new Cobweb();
            final Cobweb cobweb2 = new Cobweb();

            cobweb1.setPosition(Tile.TILE_SIZE * Tile.SCALE * 10, Tile.TILE_SIZE * Tile.SCALE);
            cobweb2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 18, Tile.TILE_SIZE * Tile.SCALE * 8);

            cobweb1.setState(AnimationState.TYPE1);
            cobweb2.setState(AnimationState.TYPE0);

            roomObjectService.add(cobweb1);
            roomObjectService.add(cobweb2);
        }
        {
            final Armorstand armorstand1 = new Armorstand();
            final Armorstand armorstand2 = new Armorstand();

            armorstand1.setPosition(Tile.TILE_SIZE * Tile.SCALE * 11.5f, Tile.TILE_SIZE * Tile.SCALE * 1.75f);
            armorstand2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 16.5f, Tile.TILE_SIZE * Tile.SCALE * 1.75f);

            roomObjectService.add(armorstand1);
            roomObjectService.add(armorstand2);

        }
        {
            final TaskBoard taskBoard = new TaskBoard();
            taskBoard.setPosition(Tile.TILE_SIZE * Tile.SCALE * 18.5f, 0);
            roomObjectService.add(taskBoard);
        }
        {
            final Pot pot = new Pot();
            final Pot pot2 = new Pot();
            final Pot pot3 = new Pot();

            pot.setPosition(Tile.TILE_SIZE * Tile.SCALE * 14f, Tile.SCALE_TILE_SIZE * 6);
            pot2.setPosition(Tile.TILE_SIZE * Tile.SCALE * 13f, Tile.SCALE_TILE_SIZE * 6);
            pot3.setPosition(Tile.TILE_SIZE * Tile.SCALE * 12f, Tile.SCALE_TILE_SIZE * 6);

            roomObjectService.add(pot);
            roomObjectService.add(pot2);
            roomObjectService.add(pot3);

            pot.setImmovable();
            pot2.setImmovable();
            pot3.setImmovable();
        }
        {
            //todo; chest for daily reward
            /*final Chest chest = new Chest();
            chest.setPosition(Tile.TILE_SIZE * Tile.SCALE * 24, Tile.TILE_SIZE * Tile.SCALE * 2 - 16);
            roomObjectService.add(chest);*/
        }
        {
            final CraftingTable craftingTable = new CraftingTable();
            craftingTable.setPosition(Tile.SCALE_TILE_SIZE * 23.5f, Tile.SCALE_TILE_SIZE * 0.5f);
            roomObjectService.add(craftingTable);
        }
        {
            final Box box = new Box();
            box.setPosition(Tile.SCALE_TILE_SIZE * 26, Tile.SCALE_TILE_SIZE * 7);
            roomObjectService.add(box);
        }
        {
            final Chains chains = new Chains();
            final Chains chains2 = new Chains();

            chains.setPosition(Tile.SCALE_TILE_SIZE * 18, 0);
            chains2.setPosition(Tile.SCALE_TILE_SIZE * 5, 0);

            roomObjectService.add(chains);
            roomObjectService.add(chains2);
        }
        {
            final Rock rock = new Rock();
            final Rock rock2 = new Rock();

            final Rock rock3 = new Rock();
            final Rock rock4 = new Rock();

            rock.setState(AnimationState.TYPE0);
            rock2.setState(AnimationState.TYPE1);

            rock3.setState(AnimationState.TYPE0);
            rock4.setState(AnimationState.TYPE1);

            rock.setPosition(Tile.SCALE_TILE_SIZE * 26, Tile.SCALE_TILE_SIZE * 4);
            rock2.setPosition(Tile.SCALE_TILE_SIZE * 17, Tile.SCALE_TILE_SIZE * 5);

            rock3.setPosition(Tile.SCALE_TILE_SIZE * 3, Tile.SCALE_TILE_SIZE * 3);
            rock4.setPosition(Tile.SCALE_TILE_SIZE * 8, Tile.SCALE_TILE_SIZE * 7);

            roomObjectService.add(rock);
            roomObjectService.add(rock2);
            roomObjectService.add(rock3);
            roomObjectService.add(rock4);
        }
        {
            final Bookshelf bookshelf = new Bookshelf();
            final Bookshelf bookshelf2 = new Bookshelf();
            bookshelf.setPosition(Tile.SCALE_TILE_SIZE * 2, 0);
            bookshelf2.setPosition(Tile.SCALE_TILE_SIZE * 6f, 0);
            roomObjectService.add(bookshelf);
            roomObjectService.add(bookshelf2);
        }
        {
            /*final PotionStation potionStation = new PotionStation();
            potionStation.setPosition(Tile.TILE_SIZE * Tile.SCALE * 2 - 16, Tile.TILE_SIZE * Tile.SCALE * 3 - 16);
            roomObjectService.add(potionStation);*/
        }
    }
    @Override
    public void cleanUp() {
        roomObjectService.cleanUp();
        tileMap.cleanUp();
    }

    @Override
    public void update(double dt) {
        roomObjectService.update(dt);
    }

    @Override
    public void fixedUpdate(double dt) {

    }

    @Override
    public List<Renderable> toRender() {
        final Renderable tiles = (DrawableBatch) tileMap::toRender;
        final Renderable roomObjects = (DrawableBatch) roomObjectService::toRender;
        return List.of(tiles, roomObjects);
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
        throw new UnsupportedOperationException("Lobby room cannot have neighbours");
    }

    @Override
    public Map<RoomDirection, Room> getNeighboursMap() {
        throw new UnsupportedOperationException("Lobby room cannot have neighbours");
    }

    @Override
    public RoomType getRoomType() {
        return RoomType.UNKNOWN;
    }

    @Override
    public List<RoomObject> getRoomObjects() {
        return roomObjectService.getRoomObjects();
    }
}
