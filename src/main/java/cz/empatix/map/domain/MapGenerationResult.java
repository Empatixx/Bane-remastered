package cz.empatix.map.domain;

import cz.empatix.rooms.service.Room;

import java.util.List;

public record MapGenerationResult (
        Room startRoom,
        List<Room> rooms,
        List<Room> paths
){

}
