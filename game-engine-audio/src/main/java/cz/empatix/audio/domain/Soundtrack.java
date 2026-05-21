package cz.empatix.audio.domain;

public enum Soundtrack {
    MAIN_MENU("intro.ogg"),
    INGAME("audio.ogg"),
    PROGRESS_ROOM("ambient.ogg"),
    BOSS_ROOM("boss.ogg");
    private final String path;
    Soundtrack(final String path) {
        this.path = path;
    }
    public String getPath() {
        return path;
    }
}
