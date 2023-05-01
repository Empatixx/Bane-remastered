package cz.empatix.audio.domain;

public enum Soundtrack {
    MAIN_MENU("main_menu.ogg"),
    INGAME("ingame.ogg"),
    PROGRESS_ROOM("progress_room.ogg"),
    BOSS_ROOM("boss_room.ogg");
    private final String path;
    Soundtrack(final String path) {
        this.path = path;
    }
    public String getPath() {
        return path;
    }
}
