package cz.empatix.assets.domain.font;

public enum FontSize {
    FONTSIZE_22(22),
    FONTSIZE_32(32),
    FONT_SIZE_42(42);
    private final int size;
    FontSize(final int size) {
        this.size = size;
    }
    public int getSize() {
        return size;
    }
}