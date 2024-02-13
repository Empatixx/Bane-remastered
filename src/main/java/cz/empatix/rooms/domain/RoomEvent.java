package cz.empatix.rooms.domain;

public record RoomEvent(
        RoomState state,
        RoomState lastState,
        RoomType roomType
) {
}
