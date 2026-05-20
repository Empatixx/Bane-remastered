package cz.empatix.rooms.service;

import cz.empatix.map.domain.RoomDirection;
import cz.empatix.player.Player;
import cz.empatix.render.attributes.DrawableBatch;
import cz.empatix.render.attributes.Renderable;
import cz.empatix.render.domain.AnimationState;
import cz.empatix.roomobjects.domain.*;
import cz.empatix.roomobjects.service.RoomObjectService;
import cz.empatix.rooms.domain.RoomType;
import cz.empatix.tiles.domain.TileImpl;
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
            final Portal portal = new Portal(new Vector2f((getMinX() + getMaxX()) / 2f - 96, TileImpl.SCALE * TileImpl.TILE_SIZE / 2f));
            roomObjectService.add(portal);
        }
        {
            final WeaponRack weaponRack = new WeaponRack();
            final WeaponRack weaponRack2 = new WeaponRack();
            weaponRack.setPosition(TileImpl.SCALE_TILE_SIZE * 22.75f, TileImpl.SCALE_TILE_SIZE/2f);
            weaponRack2.setPosition(TileImpl.SCALE_TILE_SIZE * 9.25f, TileImpl.SCALE_TILE_SIZE/2f);
            roomObjectService.add(weaponRack);
            roomObjectService.add(weaponRack2);
        }
        {
            final Torch torch1 = new Torch();
            final Torch torch2 = new Torch();
            final Torch torch3 = new Torch();
            final Torch torch4 = new Torch();

            torch1.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE, TileImpl.TILE_SIZE * TileImpl.SCALE * 2);
            torch2.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 10, TileImpl.TILE_SIZE * TileImpl.SCALE * 2);
            torch3.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 18, TileImpl.TILE_SIZE * TileImpl.SCALE * 2);
            torch4.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 27, TileImpl.TILE_SIZE * TileImpl.SCALE * 2);

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

            bones1.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 20, TileImpl.SCALE_TILE_SIZE * 7);
            bones2.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 23, TileImpl.TILE_SIZE * TileImpl.SCALE * 4);

            bones1.setState(AnimationState.TYPE1);
            bones2.setState(AnimationState.TYPE0);

            roomObjectService.add(bones1);
            roomObjectService.add(bones2);
        }
        {
            final Cobweb cobweb1 = new Cobweb();
            final Cobweb cobweb2 = new Cobweb();

            cobweb1.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 10, TileImpl.TILE_SIZE * TileImpl.SCALE);
            cobweb2.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 18, TileImpl.TILE_SIZE * TileImpl.SCALE * 8);

            cobweb1.setState(AnimationState.TYPE1);
            cobweb2.setState(AnimationState.TYPE0);

            roomObjectService.add(cobweb1);
            roomObjectService.add(cobweb2);
        }
        {
            final Armorstand armorstand1 = new Armorstand();
            final Armorstand armorstand2 = new Armorstand();

            armorstand1.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 11.5f, TileImpl.TILE_SIZE * TileImpl.SCALE * 1.75f);
            armorstand2.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 16.5f, TileImpl.TILE_SIZE * TileImpl.SCALE * 1.75f);

            roomObjectService.add(armorstand1);
            roomObjectService.add(armorstand2);

        }
        {
            final TaskBoard taskBoard = new TaskBoard();
            taskBoard.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 18.5f, 0);
            roomObjectService.add(taskBoard);
        }
        {
            final Pot pot = new Pot(null);
            final Pot pot2 = new Pot(null);
            final Pot pot3 = new Pot(null);

            pot.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 14f, TileImpl.SCALE_TILE_SIZE * 6);
            pot2.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 13f, TileImpl.SCALE_TILE_SIZE * 6);
            pot3.setPosition(TileImpl.TILE_SIZE * TileImpl.SCALE * 12f, TileImpl.SCALE_TILE_SIZE * 6);

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
            craftingTable.setPosition(TileImpl.SCALE_TILE_SIZE * 23.5f, TileImpl.SCALE_TILE_SIZE * 0.5f);
            roomObjectService.add(craftingTable);
        }
        {
            final Box box = new Box();
            box.setPosition(TileImpl.SCALE_TILE_SIZE * 26, TileImpl.SCALE_TILE_SIZE * 7);
            roomObjectService.add(box);
        }
        {
            final Chains chains = new Chains();
            final Chains chains2 = new Chains();

            chains.setPosition(TileImpl.SCALE_TILE_SIZE * 18, 0);
            chains2.setPosition(TileImpl.SCALE_TILE_SIZE * 5, 0);

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

            rock.setPosition(TileImpl.SCALE_TILE_SIZE * 26, TileImpl.SCALE_TILE_SIZE * 4);
            rock2.setPosition(TileImpl.SCALE_TILE_SIZE * 17, TileImpl.SCALE_TILE_SIZE * 5);

            rock3.setPosition(TileImpl.SCALE_TILE_SIZE * 3, TileImpl.SCALE_TILE_SIZE * 3);
            rock4.setPosition(TileImpl.SCALE_TILE_SIZE * 8, TileImpl.SCALE_TILE_SIZE * 7);

            roomObjectService.add(rock);
            roomObjectService.add(rock2);
            roomObjectService.add(rock3);
            roomObjectService.add(rock4);
        }
        {
            final Bookshelf bookshelf = new Bookshelf();
            final Bookshelf bookshelf2 = new Bookshelf();
            bookshelf.setPosition(TileImpl.SCALE_TILE_SIZE * 2, 0);
            bookshelf2.setPosition(TileImpl.SCALE_TILE_SIZE * 6f, 0);
            roomObjectService.add(bookshelf);
            roomObjectService.add(bookshelf2);
        }
        {
            final MapTable mapTable = new MapTable();
            mapTable.setPosition(TileImpl.SCALE_TILE_SIZE * 5, TileImpl.SCALE_TILE_SIZE * 5);
            roomObjectService.add(mapTable);
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
